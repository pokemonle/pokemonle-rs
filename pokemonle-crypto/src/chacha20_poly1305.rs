// use super::CryptoSerialize;
// use crate::error::{Error, Result};

// pub struct Chacha20Poly1305 {
//     key: [u8; 32],
// }

// impl Chacha20Poly1305 {
//     pub fn new(key: [u8; 32]) -> Self {
//         Self { key }
//     }

//     pub fn random() -> Self {
//         let key = [0u8; 32];
//         rand::random::<[u8; 32]>().copy_from_slice(&key);
//         Self { key }
//     }
// }

// impl CryptoSerialize for Chacha20Poly1305 {
//     fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
//         Ok(data.to_vec())
//     }
// }
