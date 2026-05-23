use std::io::{Read, Write};

/// Read a single MySQL packet from the stream.
/// Returns `(payload, sequence_number)`.
pub fn read_packet<R: Read>(reader: &mut R) -> std::io::Result<(Vec<u8>, u8)> {
    // Every MySQL packet has a 4-byte header: 3 bytes payload length + 1 byte sequence number.
    let mut header = [0u8; 4];
    reader.read_exact(&mut header)?;

    let len = u32::from_le_bytes([header[0], header[1], header[2], 0]) as usize;
    let seq = header[3];

    let mut payload = vec![0u8; len];
    reader.read_exact(&mut payload)?;

    Ok((payload, seq))
}

/// Write a single MySQL packet with the given sequence number to the stream.
pub fn write_packet<W: Write>(writer: &mut W, seq: u8, payload: &[u8]) -> std::io::Result<()> {
    let len = payload.len();
    // Header: payload length as 3-byte little-endian, then sequence number.
    let header = [
        (len & 0xFF) as u8,
        ((len >> 8) & 0xFF) as u8,
        ((len >> 16) & 0xFF) as u8,
        seq,
    ];
    writer.write_all(&header)?;
    writer.write_all(payload)?;
    writer.flush()
}

/// Decode a length-encoded integer from the start of `data`.
/// Returns `Some((value, bytes_consumed))`, or `None` if the value is NULL (`0xFB`).
pub fn read_lenenc_int(data: &[u8]) -> Option<(u64, usize)> {
    match *data.first()? {
        // 0xFB signals a NULL value — caller decides how to handle it.
        0xfb => None,
        // 0xFC: value stored in the next 2 bytes (little-endian).
        0xfc => {
            if data.len() < 3 {
                return None;
            }
            Some((u16::from_le_bytes([data[1], data[2]]) as u64, 3))
        }
        // 0xFD: value stored in the next 3 bytes (little-endian).
        0xfd => {
            if data.len() < 4 {
                return None;
            }
            Some((u32::from_le_bytes([data[1], data[2], data[3], 0]) as u64, 4))
        }
        // 0xFE: value stored in the next 8 bytes (little-endian).
        0xfe => {
            if data.len() < 9 {
                return None;
            }
            Some((u64::from_le_bytes(data[1..9].try_into().unwrap()), 9))
        }
        // 0x00–0xFA: the byte itself is the value.
        n => Some((n as u64, 1)),
    }
}

/// Decode a length-encoded string from the start of `data`.
/// Returns `Some((string_bytes, bytes_consumed))`, or `None` for NULL (`0xFB`).
pub fn read_lenenc_str(data: &[u8]) -> Option<(&[u8], usize)> {
    if data.first() == Some(&0xfb) {
        return None; // NULL
    }
    let (len, header_size) = read_lenenc_int(data)?;
    let end = header_size + len as usize;
    if data.len() < end {
        return None;
    }
    Some((&data[header_size..end], end))
}
