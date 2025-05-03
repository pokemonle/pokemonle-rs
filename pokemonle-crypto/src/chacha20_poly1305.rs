use super::CryptoSerialize;

pub struct Chacha20Poly1305 {
    key: [u8; 32],
}

impl Chacha20Poly1305 {
    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }

    pub fn random() -> Self {
        let mut key = [0u8; 32];
        rand::random::<[u8; 32]>().copy_from_slice(&mut key);
        Self { key }
    }
}

impl CryptoSerialize for Chacha20Poly1305 {
    fn encrypt<T: serde::Serialize>(&self, data: &T) -> String {}
}
