use crate::alpha::{
  Base64Alphabet, Base64ReverseAlphabet, STD_ALPHABET, STD_REV_ALPHABET, URL_ALPHABET,
  URL_REV_ALPHABET,
};

/// Standard Base64 config defined by [RFC 4648](https://datatracker.ietf.org/doc/html/rfc4648#section-4).
pub const STD_CONFIG: Base64Config =
  Base64Config::new(STD_ALPHABET, STD_REV_ALPHABET, Base64Padding::Auto);
/// URL-safe Base64 config defined by [RFC 4648](https://datatracker.ietf.org/doc/html/rfc4648#section-5).
pub const URL_CONFIG: Base64Config =
  Base64Config::new(URL_ALPHABET, URL_REV_ALPHABET, Base64Padding::No);

/// Base64 padding options.
pub enum Base64Padding {
  /// Decodes padded or unpadded values, and encodes with padding.
  Auto,
  /// Requires padding to decode values, and encodes with padding.
  Strict,
  /// Requires no padding to decode values, and encodes without padding.
  No,
}

/// Base64 config options.
pub struct Base64Config<'a> {
  /// Base64 alphabet.
  pub alphabet: &'a Base64Alphabet,
  /// Base64 reverse alphabet lookup table.
  pub rev_alphabet: &'a Base64ReverseAlphabet,
  /// Base64 padding.
  pub padding: Base64Padding,
}

impl<'a> Base64Config<'a> {
  /// Creates a new Base64 config.
  pub const fn new(
    alphabet: &'a Base64Alphabet,
    rev_alphabet: &'a Base64ReverseAlphabet,
    padding: Base64Padding,
  ) -> Self {
    Self {
      alphabet,
      rev_alphabet,
      padding,
    }
  }
}
