/// Base64 alphabet length.
pub const ALPHABET_LEN: usize = 64;

/// Base64 reverse alphabet length.
pub const REV_ALPHABET_LEN: usize = 256;

/// Standard Base64 alphabet defined by [RFC 4648](https://datatracker.ietf.org/doc/html/rfc4648#section-4).
pub const STD_ALPHABET: &Base64Alphabet =
  b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// Standard base64 reverse alphabet lookup table.
pub const STD_REV_ALPHABET: &Base64ReverseAlphabet = &base64_reverse_alphabet(STD_ALPHABET);

/// URL-safe Base64 alphabet defined by [RFC 4648](https://datatracker.ietf.org/doc/html/rfc4648#section-5).
pub const URL_ALPHABET: &Base64Alphabet =
  b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

/// URL-safe base64 reverse alphabet lookup table.
pub const URL_REV_ALPHABET: &Base64ReverseAlphabet = &base64_reverse_alphabet(URL_ALPHABET);

/// Standard Base64 padding character defined by [RFC 4648](https://datatracker.ietf.org/doc/html/rfc4648#section-4).
pub const PAD_BYTE: u8 = b"="[0];

/// Base64 input bytes group length.
pub const IN_GROUP_BYTES: usize = 3;

/// Bases64 output bytes group length.
pub const OUT_GROUP_BYTES: usize = 4;

/// Base64 alphabet data type.
pub type Base64Alphabet = [u8; ALPHABET_LEN];

/// Base64 reverse alphabet lookup table.
pub type Base64ReverseAlphabet = [u8; REV_ALPHABET_LEN];

/// Create a reverse alphabet from the given alphabet.
pub const fn base64_reverse_alphabet(alphabet: &Base64Alphabet) -> Base64ReverseAlphabet {
  let mut rev = [0u8; REV_ALPHABET_LEN];
  let mut i = 0;
  while i < ALPHABET_LEN {
    rev[alphabet[i] as usize] = i as u8;
    i += 1;
  }
  rev
}
