use std::io::Read;
use std::{collections::HashMap, sync::Arc};

use std::net::{IpAddr, SocketAddr};

use base64::prelude::*;
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use flate2::read::GzDecoder;
use serde::Deserialize;
use subtle::ConstantTimeEq;
use tar::Archive;
use thiserror::Error;
use wasmtime_wasi::{sockets::SocketAddrUse, WasiCtx};

wasmtime::component::bindgen!();

#[derive(Error, Debug)]
pub enum PluginError {
    #[error("The file is not a valid base64-encoded string")]
    InvalidBase64,
    #[error("The plugin is not a valid .tar.gz archive")]
    InvalidArchive,
    #[error("The plugin manifest is invalid")]
    InvalidManifest,
    #[error("The plugin manifest declares an unsupported WASI API: {0}")]
    InvalidWasiApi(String),
    #[error("The WASM module could not be loaded")]
    InvalidWasm,
    #[error("The manifest signature is invalid")]
    InvalidSignature,
    #[error("The WASM file's checksum does not match the one defined in the manifest")]
    InvalidChecksum,
    #[error("The plugin.wasm file is missing")]
    MissingWasm,
    #[error("The manifest.toml file is missing")]
    MissingManifest,
    #[error("The manifest.sig file is missing")]
    MissingSignature,
    #[error("The {0} function is missing in the WASM module")]
    MissingFunction(String),
    #[error("Internal wasmtime error: {0}")]
    WasmtimeError(String),
}

#[derive(Deserialize)]
struct Manifest {
    name: String,
    version: String,
    checksum: u32,
    permissions: Vec<String>,
}

struct PluginDesc {
    wasm: Vec<u8>,
    permissions: Vec<String>,
}

struct WasmContext {
    table: wasmtime::component::ResourceTable,
    wasi_ctx: WasiCtx,
}

impl wasmtime_wasi::WasiView for WasmContext {
    fn ctx(&mut self) -> wasmtime_wasi::WasiCtxView<'_> {
        wasmtime_wasi::WasiCtxView {
            ctx: &mut self.wasi_ctx,
            table: &mut self.table,
        }
    }
}

pub async fn process_plugins(
    body: &str,
    plugins: Vec<String>,
    key: &VerifyingKey,
) -> Result<String, PluginError> {
    let mut body = body.to_string();

    if plugins.is_empty() {
        return Ok(body);
    }

    let _span = tracing::debug_span!("process_plugins", body = body).entered();

    tracing::debug!(
        "Processing {} plugins on following mail: body={}",
        plugins.len(),
        body
    );

    // First, we decode and validate each plugin
    let plugins = plugins
        .into_iter()
        .map(|plugin| {
            let decoded = BASE64_STANDARD
                .decode(plugin)
                .map_err(|_| PluginError::InvalidBase64)?;

            // Decompress the gzip stream, then read the inner tar archive
            let gz = GzDecoder::new(decoded.as_slice());
            let mut archive = Archive::new(gz);

            // Extract all archive entries into a filename -> contents map
            let mut files: HashMap<String, Vec<u8>> = HashMap::new();
            for entry in archive.entries().map_err(|_| PluginError::InvalidArchive)? {
                let mut entry = entry.map_err(|_| PluginError::InvalidArchive)?;

                // Use only the filename component to avoid path traversal issues
                let path = entry
                    .path()
                    .map_err(|_| PluginError::InvalidArchive)?
                    .file_name()
                    .map(|n| n.to_string_lossy().into_owned())
                    .ok_or(PluginError::InvalidArchive)?;

                let mut contents = Vec::new();
                entry
                    .read_to_end(&mut contents)
                    .map_err(|_| PluginError::InvalidArchive)?;

                files.insert(path, contents);
            }

            // Ensure all required plugin files are present
            let wasm = files
                .remove("plugin.wasm")
                .ok_or(PluginError::MissingWasm)?;
            let manifest = files
                .get("manifest.toml")
                .ok_or(PluginError::MissingManifest)?;
            let signature = files
                .get("manifest.sig")
                .ok_or(PluginError::MissingSignature)?;

            // Validate manifest signature
            key.verify(
                manifest.as_slice(),
                &Signature::try_from(signature.as_slice())
                    .map_err(|_| PluginError::InvalidSignature)?,
            )
            .map_err(|_| PluginError::InvalidSignature)?;

            // Parse manifest
            let manifest: Manifest =
                toml::from_slice(manifest.as_slice()).map_err(|_| PluginError::InvalidManifest)?;

            let file_checksum: u32 = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC).checksum(&wasm);

            let checksum_matches: bool = file_checksum.ct_eq(&manifest.checksum).into();

            if !checksum_matches {
                return Err(PluginError::InvalidChecksum);
            }

            tracing::debug!("Plugin {}:{} accepted!", manifest.name, manifest.version);

            Ok(PluginDesc {
                wasm,
                permissions: manifest.permissions,
            })
        })
        .collect::<Result<Vec<PluginDesc>, PluginError>>()?;

    tracing::debug!("All {} plugins passed validation", plugins.len());

    tracing::debug!("Running plugins...");

    // Span is not thread-safe, so we drop it before spawning blocking tasks
    drop(_span);

    // Spawn a blocking task for each plugin to run it on a separate thread
    // This is necessary for network IO since async is not implemented in WASI p2
    for plugin in plugins {
        let result = tokio::task::spawn_blocking(move || {
            let engine = wasmtime::Engine::default();

            let mut linker = wasmtime::component::Linker::new(&engine);
            wasmtime_wasi::p2::add_to_linker_sync(&mut linker)
                .map_err(|e| PluginError::WasmtimeError(e.to_string()))?;

            // Build the WASI context with the plugin's permissions
            let mut wasi_builder = WasiCtx::builder();
            wasi_builder.allow_tcp(false).allow_udp(false);

            let mut ip_whitelist: Vec<IpAddr> = Vec::new();

            for perm in &plugin.permissions {
                if perm.starts_with("allow-ip-") {
                    let ip = perm
                        .strip_prefix("allow-ip-")
                        .ok_or_else(|| PluginError::InvalidWasiApi(perm.to_string()))?;

                    ip_whitelist.push(
                        ip.parse()
                            .map_err(|_| PluginError::InvalidWasiApi(perm.to_string()))?,
                    );
                } else {
                    setup_wasi(&mut wasi_builder, perm)?;
                }
            }

            // Create a function that allows the plugin to make network connections to whitelisted IPs only
            if !ip_whitelist.is_empty() {
                tracing::debug!("Whitelisting IP addresses: {:?}", ip_whitelist);
                let ip_whitelist = Arc::new(ip_whitelist);
                wasi_builder.socket_addr_check(move |address: SocketAddr, _: SocketAddrUse| {
                    let ip_whitelist = Arc::clone(&ip_whitelist);
                    Box::pin(async move { ip_whitelist.iter().any(|w| *w == address.ip()) })
                });
            }

            let wasi = wasi_builder.build();

            // Run the process() function in the WASM module
            let mut store = wasmtime::Store::new(
                &engine,
                WasmContext {
                    table: wasmtime::component::ResourceTable::new(),
                    wasi_ctx: wasi,
                },
            );

            let component = wasmtime::component::Component::new(&engine, plugin.wasm.as_slice())
                .map_err(|_| PluginError::InvalidWasm)?;

            let instance = linker
                .instantiate(&mut store, &component)
                .map_err(|e| PluginError::WasmtimeError(e.to_string()))?;

            let process_func = instance
                .get_typed_func::<(String,), (String,)>(&mut store, "process")
                .map_err(|_| PluginError::MissingFunction("process()".to_string()))?;

            process_func
                .call(&mut store, (body.clone(),))
                .map_err(|e| PluginError::WasmtimeError(e.to_string()))
        })
        .await
        .map_err(|e| PluginError::WasmtimeError(e.to_string()))??;

        body = result.0;
    }

    tracing::debug!("Returning transformed body...");

    Ok(body)
}

fn setup_wasi(
    wasi_builder: &mut wasmtime_wasi::WasiCtxBuilder,
    wasi: &str,
) -> Result<(), PluginError> {
    Ok(match wasi {
        "tcp" => {
            wasi_builder.allow_tcp(true);
        }
        "udp" => {
            wasi_builder.allow_udp(true);
        }
        "env" => {
            wasi_builder.inherit_env();
        }
        "blocking" => {
            wasi_builder.allow_blocking_current_thread(true);
        }
        _ => return Err(PluginError::InvalidWasiApi(wasi.to_string()))?,
    })
}
