#[cfg(feature = "aes-gcm")]
pub mod aes_gcm;
#[cfg(feature = "chacha20")]
pub mod chacha20_poly1305;
pub mod error;

use error::Result;

pub trait CryptoSerialize {
    fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>>;
}

pub trait CryptoDeserialize {
    fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>>;
}
