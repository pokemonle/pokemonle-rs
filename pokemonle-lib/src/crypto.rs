pub use pokemonle_crypto::aes_gcm::AesGcmCrypto;
use pokemonle_crypto::{CryptoDeserialize, CryptoSerialize};

pub use pokemonle_crypto::error::Error;
pub trait Crypto: CryptoSerialize + CryptoDeserialize + Send + Sync {}

impl<T: CryptoSerialize + CryptoDeserialize + Send + Sync> Crypto for T {}

fn vec_to_32_bytes(vec: &[u8]) -> [u8; 32] {
    let mut array = [0u8; 32];
    let len = vec.len().min(32);
    array[..len].copy_from_slice(&vec[..len]);
    array
}

pub fn new() -> impl Crypto {
    let config = crate::config::Config::new().unwrap();

    let crypto_key = config.crypto_key.as_bytes().to_vec();
    let crypto_key = vec_to_32_bytes(&crypto_key)
        .try_into()
        .expect("Invalid key length");

    AesGcmCrypto::new(crypto_key)
}
