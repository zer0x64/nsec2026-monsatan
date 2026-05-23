mod auth;
mod packet;

use std::net::TcpStream;

use auth::{build_handshake_response, native_password, parse_handshake};
use packet::{read_lenenc_int, read_lenenc_str, read_packet, write_packet};

/// An open, authenticated MySQL connection.
pub struct Connection {
    stream: TcpStream,
    /// Packet sequence number. Resets to 0 at the start of each new command.
    seq: u8,
}

impl Connection {
    /// Connect to `addr` and authenticate with the given credentials.
    ///
    /// `addr` should be in `"host:port"` format, e.g. `"127.0.0.1:3306"`.
    pub fn connect(addr: &str, user: &str, pass: &str, db: &str) -> Result<Self, String> {
        let stream = TcpStream::connect(addr).map_err(|e| e.to_string())?;
        let mut conn = Connection { stream, seq: 0 };

        // Step 1 — receive the server's Initial Handshake Packet (seq 0).
        let greeting = conn.recv()?;

        // Step 2 — extract the 20-byte nonce and auth plugin name.
        let (nonce, plugin) = parse_handshake(&greeting)?;

        // Only mysql_native_password is implemented. For MySQL 8+ with
        // caching_sha2_password, set: default_authentication_plugin=mysql_native_password
        if plugin != "mysql_native_password" {
            return Err(format!("Unsupported auth plugin: {plugin}"));
        }

        // Step 3 — send HandshakeResponse41 (seq 1).
        let auth_response = native_password(pass.as_bytes(), &nonce);
        conn.send(&build_handshake_response(user, &auth_response, db))?;

        // Step 4 — expect an OK packet (seq 2).
        let reply = conn.recv()?;
        check_ok(&reply)?;

        Ok(conn)
    }

    /// Execute a query and return the results as strings.
    ///
    /// The first element of the returned `Vec` is a tab-separated header row
    /// of column names; subsequent elements are tab-separated data rows.
    pub fn query(&mut self, sql: &str) -> Result<Vec<String>, String> {
        // Each new command starts a fresh sequence.
        self.seq = 0;

        // COM_QUERY = 0x03, followed immediately by the SQL text.
        let mut pkt = vec![0x03u8];
        pkt.extend_from_slice(sql.as_bytes());
        self.send(&pkt)?;

        self.read_text_resultset()
    }

    // ---- private helpers --------------------------------------------------------

    /// Write a packet and increment the sequence counter.
    fn send(&mut self, payload: &[u8]) -> Result<(), String> {
        write_packet(&mut self.stream, self.seq, payload).map_err(|e| e.to_string())?;
        self.seq = self.seq.wrapping_add(1);
        Ok(())
    }

    /// Read the next packet and update the sequence counter from it.
    fn recv(&mut self) -> Result<Vec<u8>, String> {
        let (payload, seq) = read_packet(&mut self.stream).map_err(|e| e.to_string())?;
        self.seq = seq.wrapping_add(1);
        Ok(payload)
    }

    /// Read a full text-protocol result set.
    ///
    /// Wire order after COM_QUERY:
    ///   1. Column-count packet (lenenc-int)
    ///   2. N × column-definition packets
    ///   3. EOF packet
    ///   4. M × row-data packets
    ///   5. EOF packet
    fn read_text_resultset(&mut self) -> Result<Vec<String>, String> {
        // First packet is either an error, an OK (no rows), or the column count.
        let pkt = self.recv()?;
        match pkt.first() {
            Some(0xff) => return Err(parse_error(&pkt)),
            Some(0x00) => return Ok(vec![]), // OK with no result set (e.g. DML)
            _ => {}
        }

        let (col_count, _) = read_lenenc_int(&pkt).ok_or("Failed to decode column count")?;
        let col_count = col_count as usize;

        // Collect column names from the definition packets.
        let mut col_names = Vec::with_capacity(col_count);
        for _ in 0..col_count {
            let col_def = self.recv()?;
            col_names.push(parse_column_name(&col_def));
        }

        // EOF separating column definitions from row data.
        let eof = self.recv()?;
        if !is_eof(&eof) {
            return Err(format!(
                "Expected EOF after column definitions, got: {:02x?}",
                &eof[..eof.len().min(4)]
            ));
        }

        // First row is the header.
        let mut rows = vec![col_names.join("\t")];

        // Read row packets until the closing EOF.
        loop {
            let row_pkt = self.recv()?;

            if is_eof(&row_pkt) {
                break;
            }
            if row_pkt.first() == Some(&0xff) {
                return Err(parse_error(&row_pkt));
            }

            rows.push(parse_text_row(&row_pkt, col_count));
        }

        Ok(rows)
    }
}

// ---- packet-level helpers -------------------------------------------------------

/// A MySQL EOF packet starts with `0xFE` and is always shorter than 9 bytes.
fn is_eof(pkt: &[u8]) -> bool {
    pkt.first() == Some(&0xfe) && pkt.len() < 9
}

/// Return `Ok(())` for an OK packet, or a descriptive error for an ERR packet.
fn check_ok(pkt: &[u8]) -> Result<(), String> {
    match pkt.first() {
        Some(0x00) => Ok(()),
        Some(0xff) => Err(parse_error(pkt)),
        other => Err(format!("Unexpected response byte: {other:02x?}")),
    }
}

/// Extract the human-readable message from an ERR packet.
///
/// ERR layout: `0xFF | u16 error_code | '#' | 5-byte sqlstate | message`
fn parse_error(pkt: &[u8]) -> String {
    // Byte 0 is 0xFF, bytes 1-2 are the error code.
    // If byte 3 is '#', bytes 4-8 are the SQL state and the message starts at 9.
    let msg_start = if pkt.len() > 3 && pkt[3] == b'#' {
        9
    } else {
        3
    };
    let msg = String::from_utf8_lossy(pkt.get(msg_start..).unwrap_or(b"(empty)"));
    format!("MySQL error: {msg}")
}

/// Extract the column name from a column-definition packet.
///
/// The definition packet is a sequence of lenenc-strings in this order:
/// `catalog | db | table | org_table | name | org_name | ...`
/// We skip the first four fields to reach `name`.
fn parse_column_name(data: &[u8]) -> String {
    let mut pos = 0;
    // Skip catalog, db, table, org_table (4 lenenc-strings).
    for _ in 0..4 {
        match read_lenenc_str(&data[pos..]) {
            Some((_, consumed)) => pos += consumed,
            None => return "<null>".into(),
        }
    }
    // The 5th field is the column name.
    match read_lenenc_str(&data[pos..]) {
        Some((bytes, _)) => String::from_utf8_lossy(bytes).into_owned(),
        None => "<null>".into(),
    }
}

/// Decode one text-protocol row into a tab-separated string.
///
/// Each column value is either:
///  - `0xFB` — NULL
///  - lenenc-string — the value as raw bytes
fn parse_text_row(data: &[u8], col_count: usize) -> String {
    let mut pos = 0;
    let mut fields = Vec::with_capacity(col_count);

    for _ in 0..col_count {
        if pos >= data.len() {
            fields.push(String::new());
            continue;
        }

        if data[pos] == 0xfb {
            // NULL value
            fields.push("NULL".to_string());
            pos += 1;
        } else {
            match read_lenenc_str(&data[pos..]) {
                Some((bytes, consumed)) => {
                    fields.push(String::from_utf8_lossy(bytes).into_owned());
                    pos += consumed;
                }
                None => {
                    fields.push(String::new());
                }
            }
        }
    }

    fields.join("\t")
}
