use crate::alpha::{IN_GROUP_BYTES, OUT_GROUP_BYTES, PAD_BYTE};
use crate::config::{Base64Config, Base64Padding, STD_CONFIG, URL_CONFIG};
use crate::error::{Error, Result};

/// Standard Base64 decoder defined by [RFC 4648](https://datatracker.ietf.org/doc/html/rfc4648#section-4).
pub const STD_DECODER: Base64Decoder = Base64Decoder::new(&STD_CONFIG);
/// URL-safe Base64 decoder defined by [RFC 4648](https://datatracker.ietf.org/doc/html/rfc4648#section-5).
pub const URL_DECODER: Base64Decoder = Base64Decoder::new(&URL_CONFIG);

/// Base64 decoder.
pub struct Base64Decoder<'a> {
  /// Base64 decoder config.
  config: &'a Base64Config<'a>,
}

impl<'a> Base64Decoder<'a> {
  /// Creates a new Base64 decoder.
  pub const fn new(config: &'a Base64Config) -> Self {
    Self { config }
  }

  /// Decodes the given base64 string as a byte vector.
  ///
  /// ## Example
  ///
  /// ```
  /// let decoder = ubase64::decode::STD_DECODER;
  /// let decoded = decoder.decode("SGVsbG8sIHdvcmxkIQ==").unwrap();
  /// assert_eq!(decoded, b"Hello, world!");
  /// ```
  pub fn decode(&self, s: &str) -> Result<Vec<u8>> {
    if matches!(self.config.padding, Base64Padding::Strict) && s.len() % OUT_GROUP_BYTES != 0 {
      return Err(Error::DecodeInvalidInputLength {});
    }
    let s_trimmed = if matches!(
      self.config.padding,
      Base64Padding::Auto | Base64Padding::Strict
    ) {
      s.trim_end_matches(PAD_BYTE as char)
    } else {
      s
    };
    let len = IN_GROUP_BYTES * s_trimmed.len() / OUT_GROUP_BYTES;
    let mut bytes_decoded = Vec::with_capacity(len);
    let chunks_iter = s_trimmed.as_bytes().chunks_exact(OUT_GROUP_BYTES);
    chunks_iter.clone().for_each(|chunk| {
      let n = ((self.config.rev_alphabet[chunk[0] as usize] as u32) << 18)
        | ((self.config.rev_alphabet[chunk[1] as usize] as u32) << 12)
        | ((self.config.rev_alphabet[chunk[2] as usize] as u32) << 6)
        | (self.config.rev_alphabet[chunk[3] as usize] as u32);
      bytes_decoded.push(((n >> 16) & 0xff) as u8);
      bytes_decoded.push(((n >> 8) & 0xff) as u8);
      bytes_decoded.push((n & 0xff) as u8);
    });
    let rem = chunks_iter.remainder();
    match rem.len() {
      0 => {}
      2 => {
        let n = ((self.config.rev_alphabet[rem[0] as usize] as u32) << 2)
          | (self.config.rev_alphabet[rem[1] as usize] as u32 >> 4);
        bytes_decoded.push(n as u8);
      }
      3 => {
        let n = ((self.config.rev_alphabet[rem[0] as usize] as u32) << 10)
          | ((self.config.rev_alphabet[rem[1] as usize] as u32) << 4)
          | (self.config.rev_alphabet[rem[2] as usize] as u32 >> 2);
        bytes_decoded.push(((n >> 8) & 0xff) as u8);
        bytes_decoded.push((n & 0xff) as u8);
      }
      _ => {
        return Err(Error::DecodeUnexpectedRemainderChars {});
      }
    };
    Ok(bytes_decoded)
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
  fn test_std_decode_rfc4648_tests() {
    for (expected, encoded) in RFC4648_TESTS {
      let decoded = STD_DECODER.decode(encoded).unwrap();
      assert_eq!(decoded, *expected);
    }
  }
}
