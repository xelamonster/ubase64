use crate::alpha::{IN_GROUP_BYTES, OUT_GROUP_BYTES, PAD_BYTE};
use crate::config::{Base64Config, Base64Padding, STD_CONFIG, URL_CONFIG};
use crate::error::{Error, Result};

/// Standard Base64 encoder defined by [RFC 4648](https://datatracker.ietf.org/doc/html/rfc4648#section-4).
pub const STD_ENCODER: Base64Encoder = Base64Encoder::new(&STD_CONFIG);
/// URL-safe Base64 encoder defined by [RFC 4648](https://datatracker.ietf.org/doc/html/rfc4648#section-5).
pub const URL_ENCODER: Base64Encoder = Base64Encoder::new(&URL_CONFIG);

/// Base64 encoder.
pub struct Base64Encoder<'a> {
  /// Base64 encoder config.
  config: &'a Base64Config<'a>,
}

impl<'a> Base64Encoder<'a> {
  /// Creates a new Base64 encoder.
  pub const fn new(config: &'a Base64Config) -> Self {
    Self { config }
  }

  /// Encodes the given bytes as a base64 string.
  ///
  /// ## Example
  ///
  /// ```
  /// let encoder = ubase64::encode::STD_ENCODER;
  /// let encoded = encoder.encode(b"Hello, world!").unwrap();
  /// assert_eq!(encoded, "SGVsbG8sIHdvcmxkIQ==");
  /// ```
  pub fn encode(&self, bytes: &[u8]) -> Result<String> {
    let bytes_len: usize = bytes.len();
    let len = OUT_GROUP_BYTES * (bytes_len + IN_GROUP_BYTES - 1) / IN_GROUP_BYTES;
    let mut bytes_encoded = Vec::with_capacity(len);
    let chunks_iter = bytes.chunks_exact(IN_GROUP_BYTES);
    chunks_iter.clone().for_each(|chunk| {
      let n = (((chunk[0] as u32) << 16) & 0xff0000u32)
        + (((chunk[1] as u32) << 8) & 0xff00u32)
        + ((chunk[2] as u32) & 0xffu32);
      bytes_encoded.push(self.config.alphabet[((n >> 18) & 0x3f) as usize]);
      bytes_encoded.push(self.config.alphabet[((n >> 12) & 0x3f) as usize]);
      bytes_encoded.push(self.config.alphabet[((n >> 6) & 0x3f) as usize]);
      bytes_encoded.push(self.config.alphabet[(n & 0x3f) as usize]);
    });
    let rem = chunks_iter.remainder();
    match rem.len() {
      0 => {}
      1 => {
        let n = rem[0] as u32;
        bytes_encoded.push(self.config.alphabet[(n >> 2) as usize]);
        bytes_encoded.push(self.config.alphabet[((n << 4) & 0x3f) as usize]);
        if matches!(
          self.config.padding,
          Base64Padding::Auto | Base64Padding::Strict
        ) {
          bytes_encoded.push(PAD_BYTE);
          bytes_encoded.push(PAD_BYTE);
        }
      }
      2 => {
        let n = ((rem[0] as u32) << 8) + (rem[1] as u32);
        bytes_encoded.push(self.config.alphabet[(n >> 10) as usize]);
        bytes_encoded.push(self.config.alphabet[((n >> 4) & 0x3f) as usize]);
        bytes_encoded.push(self.config.alphabet[((n << 2) & 0x3f) as usize]);
        if matches!(
          self.config.padding,
          Base64Padding::Auto | Base64Padding::Strict
        ) {
          bytes_encoded.push(PAD_BYTE);
        }
      }
      _ => return Err(Error::EncodeUnexpectedRemainderBytes {}),
    }
    Ok(String::from_utf8(bytes_encoded)?)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  // (ref https://datatracker.ietf.org/doc/html/rfc4648#section-10)
  const RFC4648_TESTS: &[(&[u8], &str)] = &[
    (b"", ""),
    (b"f", "Zg=="),
    (b"fo", "Zm8="),
    (b"foo", "Zm9v"),
    (b"foob", "Zm9vYg=="),
    (b"fooba", "Zm9vYmE="),
    (b"foobar", "Zm9vYmFy"),
  ];

  #[test]
  fn test_std_encode_rfc4648_tests() {
    for (decoded, expected) in RFC4648_TESTS {
      let decoded = STD_ENCODER.encode(decoded).unwrap();
      assert_eq!(decoded, *expected);
    }
  }
}
