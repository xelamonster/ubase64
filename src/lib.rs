pub mod alpha;
pub mod config;
pub mod decode;
pub mod encode;
mod error;

pub use error::{Error, Result};

pub fn encode_std(bytes: &[u8]) -> Result<String> {
  encode::STD_ENCODER.encode(bytes)
}

pub fn decode_std(s: &str) -> Result<Vec<u8>> {
  decode::STD_DECODER.decode(s)
}
