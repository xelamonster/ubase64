use miette::Diagnostic;

/// Base64 result type alias with defaults.
pub type Result<T=(), E=Error> = std::result::Result<T, E>;

/// Base64 error type.
#[derive(thiserror::Error, Debug, Diagnostic)]
#[diagnostic(url(docsrs))]
pub enum Error {
  /// Utf8 string parse error.
  #[error(transparent)]
  Utf8(#[from] std::string::FromUtf8Error),

  /// Base64 encode remainder error.
  #[error("Unexpected number of bytes remaining after encode")]
  EncodeUnexpectedRemainderBytes {},

  /// Base64 decode input length error.
  #[error("Invalid input string length")]
  #[diagnostic(help("This error can only occur with padding option `Strict`."))]
  DecodeInvalidInputLength {},

  /// Base64 decode remainder error.
  #[error("Unexpected number of chars remaining after decode")]
  #[diagnostic(help("Ensure your input data is valid base64."))]
  DecodeUnexpectedRemainderChars {},
}
