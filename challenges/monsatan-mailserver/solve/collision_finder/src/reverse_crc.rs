//! Rust translation of <https://github.com/theonlypwner/crc32>
//!
//! Provides CRC32 forward computation, reverse computation (rewinding through
//! data and finding byte sequences that produce a desired CRC), and CRC
//! combining using GF(2) matrix exponentiation.

use std::collections::HashSet;

// ── CRC32 lookup table ────────────────────────────────────────────────────────

/// CRC32 state machine backed by a 256-entry lookup table.
pub struct Crc32 {
    /// Forward lookup table: `table[b]` gives the next CRC state for input byte `b`.
    pub table: [u32; 256],
}

impl Crc32 {
    /// Build a [`Crc32`] for the given reversed (lsbit-first) polynomial.
    pub fn new(poly: u32) -> Self {
        let mut table = [0u32; 256];
        for i in 0u32..256 {
            let mut v = i;
            for _ in 0..8 {
                // XOR with poly when the lsb is 1, otherwise just right-shift.
                // `(v & 1).wrapping_neg()` is 0xFFFFFFFF when v&1=1 and 0 otherwise,
                // acting as a bitmask for the conditional XOR.
                v = (v >> 1) ^ (poly & (v & 1).wrapping_neg());
            }
            table[i as usize] = v;
        }
        Self { table }
    }

    /// Compute the CRC32 of `data`, starting from accumulator `accum`
    /// (use `0` for a fresh computation).
    pub fn calc(&self, data: &[u8], accum: u32) -> u32 {
        let mut acc = !accum;
        for &b in data {
            acc = self.table[((acc ^ b as u32) & 0xFF) as usize] ^ (acc >> 8);
        }
        !acc
    }
}

// ── Reverse lookup ────────────────────────────────────────────────────────────

/// Extends [`Crc32`] with a reverse lookup table, enabling backwards traversal
/// of the CRC state machine.
pub struct Crc32Reverse {
    pub crc32: Crc32,
    /// `table_reverse[i]` lists every byte `j` in `0..=255` where
    /// `table[j] >> 24 == i` (the inverse of the high-byte mapping).
    pub table_reverse: Vec<Vec<u8>>,
}

impl Crc32Reverse {
    /// Build a [`Crc32Reverse`] for the given reversed polynomial.
    pub fn new(poly: u32) -> Self {
        let crc32 = Crc32::new(poly);

        let mut table_reverse: Vec<Vec<u8>> = vec![Vec::new(); 256];
        for j in 0u32..256 {
            let top_byte = (crc32.table[j as usize] >> 24) as usize;
            table_reverse[top_byte].push(j as u8);
        }

        Self {
            crc32,
            table_reverse,
        }
    }

    /// Given `data` and the CRC value *after* processing it (`accum`), return
    /// the set of CRC values that could have existed *before* processing `data`.
    pub fn rewind(&self, data: &[u8], accum: u32) -> HashSet<u32> {
        if data.is_empty() {
            return HashSet::from([accum]);
        }

        // Stack holds (current offset, internal CRC state).
        // The internal state is the pre-final-inversion value.
        let mut stack: Vec<(usize, u32)> = vec![(data.len(), !accum)];
        let mut solutions = HashSet::new();

        while let Some((offset, node_crc)) = stack.pop() {
            let prev_offset = offset - 1;
            let top = ((node_crc >> 24) & 0xFF) as usize;

            for &i in &self.table_reverse[top] {
                // Undo one CRC step to recover the state before data[prev_offset].
                let prev_crc = ((node_crc ^ self.crc32.table[i as usize]) << 8)
                    | (i as u32 ^ data[prev_offset] as u32);

                if prev_offset > 0 {
                    stack.push((prev_offset, prev_crc));
                } else {
                    // Apply the final inversion to get the user-facing CRC.
                    solutions.insert(!prev_crc);
                }
            }
        }

        solutions
    }

    /// Find all 4-byte sequences that, when appended to a message with CRC state
    /// `accum`, produce the `desired` CRC output.
    pub fn find_reverse(&self, desired: u32, accum: u32) -> HashSet<Vec<u8>> {
        let mut solutions = HashSet::new();
        // Pre-invert accum to work in the internal (pre-inversion) CRC domain.
        let accum_internal = !accum;

        // Stack holds (internal CRC state, collected suffix bytes so far).
        let mut stack: Vec<(u32, Vec<u8>)> = vec![(!desired, Vec::new())];

        while let Some((v, s)) = stack.pop() {
            let top = ((v >> 24) & 0xFF) as usize;

            for &j in &self.table_reverse[top] {
                let mut next_str = s.clone();
                next_str.push(j);

                if next_str.len() == 4 {
                    // Reconstruct the 4 actual input bytes from the internal suffix.
                    // Iterates i from 3 down to 0, building data in that order.
                    let mut a = accum_internal;
                    let mut data = Vec::with_capacity(4);
                    for i in (0..4).rev() {
                        data.push((a ^ next_str[i] as u32) as u8);
                        a >>= 8;
                        a ^= self.crc32.table[next_str[i] as usize];
                    }
                    solutions.insert(data);
                } else {
                    // Continue building the suffix one byte at a time.
                    stack.push(((v ^ self.crc32.table[j as usize]) << 8, next_str));
                }
            }
        }

        solutions
    }
}

// ── GF(2) Matrix ─────────────────────────────────────────────────────────────

/// A 32×32 binary matrix over GF(2), stored as 32 column vectors.
///
/// Used to represent the linear effect of feeding zero bytes into the CRC
/// state machine, enabling fast CRC combining via matrix exponentiation.
pub struct Matrix {
    /// Column vectors; `cols[i]` is the i-th column as a 32-bit bitmask.
    cols: [u32; 32],
}

impl Matrix {
    /// Return the 32×32 identity matrix.
    pub fn identity() -> Self {
        let mut cols = [0u32; 32];
        for i in 0..32 {
            cols[i] = 1 << i;
        }
        Self { cols }
    }

    /// Return the "zero-byte operator" matrix for the given polynomial.
    ///
    /// Multiplying the CRC state by this matrix advances it by one zero byte.
    pub fn zero_operator(poly: u32) -> Self {
        let mut cols = [0u32; 32];
        cols[0] = poly;
        let mut n: u32 = 1;
        for i in 1..32 {
            cols[i] = n;
            n <<= 1;
        }
        Self { cols }
    }

    /// Multiply this matrix by column vector `v` (over GF(2)),
    /// accumulating the result into `s` via XOR.
    pub fn multiply_vector(&self, mut v: u32, mut s: u32) -> u32 {
        for &c in &self.cols {
            // XOR column c into s when the corresponding bit of v is 1.
            // `(v & 1).wrapping_neg()` is 0xFFFFFFFF when v&1=1, else 0.
            s ^= c & (v & 1).wrapping_neg();
            v >>= 1;
            if v == 0 {
                break;
            }
        }
        s
    }

    /// Return `self * other` (matrix multiplication over GF(2)).
    pub fn mul(&self, other: &Matrix) -> Matrix {
        let mut cols = [0u32; 32];
        for (i, &col) in other.cols.iter().enumerate() {
            cols[i] = self.multiply_vector(col, 0);
        }
        Matrix { cols }
    }

    /// Return `self * self` (matrix squaring over GF(2)).
    pub fn sqr(&self) -> Matrix {
        self.mul(self)
    }
}

// ── CRC combining ─────────────────────────────────────────────────────────────

/// Combine two CRC values where `c2` covers a message of `l2` bytes,
/// repeated `n` times, using the given polynomial.
///
/// Uses GF(2) matrix exponentiation (exponentiation-by-squaring) to avoid
/// processing all the bytes explicitly.
///
/// See <https://github.com/madler/zlib/blob/v1.2.11/crc32.c#L341-L434>
pub fn combine(mut c1: u32, c2: u32, mut l2: u32, mut n: u32, poly: u32) -> u32 {
    // Build big_m = zero_operator^(8 * l2), the matrix that advances the CRC
    // state by l2 zero bytes. Start from zero_operator^4 and square repeatedly.
    let mut m = Matrix::zero_operator(poly).sqr().sqr();
    let mut big_m = Matrix::identity();

    while l2 > 0 {
        m = m.sqr();
        if l2 & 1 != 0 {
            big_m = m.mul(&big_m);
        }
        l2 >>= 1;
    }

    // Apply big_m n times to c1 using exponentiation-by-squaring.
    // `b` accumulates the additive (affine) term from c2.
    let mut b = c2;
    loop {
        if n & 1 != 0 {
            c1 = big_m.multiply_vector(c1, b);
        }
        n >>= 1;
        if n == 0 {
            break;
        }
        b = big_m.multiply_vector(b, b);
        big_m = big_m.sqr();
    }

    c1
}

// ── Helper functions ──────────────────────────────────────────────────────────

/// Reverse all 32 bits of `x`.
///
/// Note: Rust's `u32::reverse_bits()` does the same thing; this is provided
/// for API parity with the original Python library.
pub fn reverse_bits(x: u32) -> u32 {
    x.reverse_bits()
}

/// Return the reciprocal polynomial of a reversed (lsbit-first) polynomial.
pub fn reciprocal(poly: u32) -> u32 {
    (poly << 1) | 1
}
