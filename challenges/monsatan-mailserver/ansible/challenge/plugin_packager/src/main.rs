use std::{fs::File, path::PathBuf};

use clap::Parser;
use ed25519_dalek::{Signer, SigningKey};
use flate2::{Compression, write::GzEncoder};

use serde::{Deserialize, Serialize};
use tar::{Builder, Header};

#[allow(unused)]
#[derive(Serialize, Deserialize)]
struct Manifest {
    name: String,
    version: String,
    checksum: u32,
    permissions: Vec<String>,
}

pub const SIGNING_KEY: [u8; 32] = [
    239, 230, 217, 149, 174, 6, 28, 184, 251, 45, 124, 249, 8, 0, 102, 82, 133, 114, 6, 155, 107,
    160, 118, 45, 3, 183, 180, 116, 102, 28, 17, 58,
];

#[derive(clap::Parser)]
struct Cli {
    pub wasm: PathBuf,
    pub manifest: PathBuf,

    #[clap(short, long)]
    pub output: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();

    let module = std::fs::read(&cli.wasm).expect("Failed to read wasm file!");
    let mut manifest: Manifest = toml::from_str(
        &std::fs::read_to_string(&cli.manifest).expect("Failed to read manifest file!"),
    )
    .expect("Failed to parse manifest file!");

    let filename = cli
        .output
        .unwrap_or((manifest.name.clone() + ".tar.gz").into());

    manifest.checksum = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC).checksum(&module);

    let manifest_bytes = toml::to_string(&manifest)
        .expect("Failed to serialize manifest!")
        .as_bytes()
        .to_vec();

    let key = SigningKey::from_bytes(&SIGNING_KEY);
    let signature = key.sign(&manifest_bytes).to_bytes();

    let archive_file = File::create(filename).expect("Failed to create archive file!");

    let enc = GzEncoder::new(archive_file, Compression::default());
    let mut archive = Builder::new(enc);

    let mut header = Header::new_gnu();
    header.set_size(manifest_bytes.len() as u64);
    header.set_mode(0x1b0);
    header.set_cksum();

    archive
        .append_data(&mut header, "manifest.toml", manifest_bytes.as_slice())
        .expect("Failed to append manifest to archive!");

    let mut header = Header::new_gnu();
    header.set_size(signature.len() as u64);
    header.set_mode(0x1b0);
    header.set_cksum();

    archive
        .append_data(&mut header, "manifest.sig", signature.as_slice())
        .expect("Failed to append signature to archive!");

    let mut header = Header::new_gnu();
    header.set_size(module.len() as u64);
    header.set_mode(0x1b0);
    header.set_cksum();

    archive
        .append_data(&mut header, "plugin.wasm", module.as_slice())
        .expect("Failed to append wasm to archive!");

    archive.finish().expect("Failed to finish archive!");
}
