use sha1::{Digest, Sha1};

// ---- Capability flags we use in the HandshakeResponse ----

const CLIENT_LONG_PASSWORD: u32 = 0x0000_0001;
const CLIENT_CONNECT_WITH_DB: u32 = 0x0000_0008;
const CLIENT_PROTOCOL_41: u32 = 0x0000_0200;
const CLIENT_SECURE_CONNECTION: u32 = 0x0000_8000;
const CLIENT_PLUGIN_AUTH: u32 = 0x0008_0000;

/// Compute the `mysql_native_password` auth token.
///
/// Formula: `SHA1(password) XOR SHA1(nonce || SHA1(SHA1(password)))`
pub fn native_password(password: &[u8], nonce: &[u8]) -> Vec<u8> {
    if password.is_empty() {
        return vec![];
    }

    let hash1 = Sha1::digest(password); // SHA1(password)
    let hash2 = Sha1::digest(&hash1); // SHA1(SHA1(password))

    // SHA1(nonce || hash2)
    let mut h = Sha1::new();
    h.update(nonce);
    h.update(&hash2);
    let hash3 = h.finalize();

    // XOR hash1 with hash3 to produce the token
    hash1.iter().zip(hash3.iter()).map(|(a, b)| a ^ b).collect()
}

/// Parse the server's Initial Handshake Packet (protocol version 10).
///
/// Returns `(nonce, plugin_name)` where `nonce` is the 20-byte auth salt.
///
/// Packet layout:
/// ```
///  1 byte   protocol version (0x0a)
///  NUL str  server version
///  4 bytes  connection id
///  8 bytes  auth-plugin-data part 1
///  1 byte   filler (0x00)
///  2 bytes  capability flags (lower)
///  1 byte   character set
///  2 bytes  status flags
///  2 bytes  capability flags (upper)
///  1 byte   auth-plugin-data length
/// 10 bytes  reserved
///  N bytes  auth-plugin-data part 2  (MAX(13, data_len - 8), includes trailing NUL)
///  NUL str  auth-plugin-name
/// ```
pub fn parse_handshake(data: &[u8]) -> Result<(Vec<u8>, String), String> {
    if data.first() != Some(&0x0a) {
        return Err(format!("Expected protocol v10, got: {:02x?}", data.first()));
    }

    let mut pos = 1;

    // Skip null-terminated server version string.
    pos += data[pos..]
        .iter()
        .position(|&b| b == 0)
        .ok_or("Missing NUL terminator after server version")?
        + 1;

    pos += 4; // Skip connection ID (4 bytes).

    // auth-plugin-data part 1: always 8 bytes.
    if pos + 8 > data.len() {
        return Err("Handshake packet too short (part 1)".into());
    }
    let part1 = data[pos..pos + 8].to_vec();
    pos += 8;

    pos += 1; // filler (0x00)
    pos += 2; // capability flags lower
    pos += 1; // character set
    pos += 2; // status flags
    pos += 2; // capability flags upper

    // Length of the full auth-plugin-data (part1 + part2).
    let auth_data_len = data[pos] as usize;
    pos += 1;

    pos += 10; // Reserved (all zeros).

    // auth-plugin-data part 2: MAX(13, auth_data_len - 8) bytes.
    // The last byte of part 2 is a trailing NUL that we strip.
    let part2_len = std::cmp::max(13usize, auth_data_len.saturating_sub(8));
    if pos + part2_len > data.len() {
        return Err("Handshake packet too short (part 2)".into());
    }
    let part2 = data[pos..pos + part2_len - 1].to_vec(); // strip trailing NUL
    pos += part2_len;

    // Auth plugin name: null-terminated string.
    let name_len = data[pos..]
        .iter()
        .position(|&b| b == 0)
        .unwrap_or(data.len() - pos);
    let plugin = String::from_utf8_lossy(&data[pos..pos + name_len]).into_owned();

    // The full nonce is always 20 bytes: 8 (part1) + 12 (part2).
    let mut nonce = part1;
    nonce.extend_from_slice(&part2);
    nonce.truncate(20);

    Ok((nonce, plugin))
}

/// Build a `HandshakeResponse41` packet (the client's reply to the server greeting).
///
/// Layout:
/// ```
///  4 bytes  capability flags
///  4 bytes  max packet size
///  1 byte   character set
/// 23 bytes  reserved (zeros)
///  NUL str  username
///  1 byte   auth response length   (CLIENT_SECURE_CONNECTION style)
///  N bytes  auth response
///  NUL str  database name          (CLIENT_CONNECT_WITH_DB)
///  NUL str  auth plugin name       (CLIENT_PLUGIN_AUTH)
/// ```
pub fn build_handshake_response(user: &str, auth_response: &[u8], db: &str) -> Vec<u8> {
    let caps = CLIENT_LONG_PASSWORD
        | CLIENT_CONNECT_WITH_DB
        | CLIENT_PROTOCOL_41
        | CLIENT_SECURE_CONNECTION
        | CLIENT_PLUGIN_AUTH;

    let mut pkt = Vec::new();

    pkt.extend_from_slice(&caps.to_le_bytes()); // Capability flags (4 bytes)
    pkt.extend_from_slice(&0x00FF_FFFFu32.to_le_bytes()); // Max packet size
    pkt.push(0x21); // Charset: utf8_general_ci
    pkt.extend_from_slice(&[0u8; 23]); // Reserved

    pkt.extend_from_slice(user.as_bytes()); // Username (NUL-terminated)
    pkt.push(0x00);

    // Auth response, length-prefixed (CLIENT_SECURE_CONNECTION style: 1-byte length).
    pkt.push(auth_response.len() as u8);
    pkt.extend_from_slice(auth_response);

    pkt.extend_from_slice(db.as_bytes()); // Database (NUL-terminated)
    pkt.push(0x00);

    pkt.extend_from_slice(b"mysql_native_password"); // Plugin name (NUL-terminated)
    pkt.push(0x00);

    pkt
}
