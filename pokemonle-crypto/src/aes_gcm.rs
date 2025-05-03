use aes_gcm::AeadCore;
use base64::Engine;

use super::{CryptoDeserialize, CryptoSerialize};
use crate::error::{Error, Result};

#[derive(Debug, Clone)]
pub struct AesGcmCrypto {
    key: [u8; 32],
}

impl AesGcmCrypto {
    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }

    pub fn random() -> Self {
        let mut key = [0u8; 32];
        rand::random::<[u8; 32]>().copy_from_slice(&mut key);
        Self { key }
    }
}

impl CryptoSerialize for AesGcmCrypto {
    fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        use aes_gcm::{
            Aes256Gcm,
            aead::{Aead, KeyInit, OsRng},
        };

        let plaintext = data.to_vec();

        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

        let cipher =
            Aes256Gcm::new_from_slice(&self.key).map_err(|e| Error::Encryption(e.to_string()))?;
        let ciphertext = cipher
            .encrypt(&nonce, plaintext.as_ref())
            .map_err(|e| Error::Encryption(e.to_string()))?;

        let mut output = Vec::with_capacity(nonce.len() + ciphertext.len());
        output.extend_from_slice(&nonce);
        output.extend_from_slice(&ciphertext);

        let engine = base64::engine::general_purpose::STANDARD;
        Ok(engine.encode(output).into_bytes())
    }
}

impl CryptoDeserialize for AesGcmCrypto {
    fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        use aes_gcm::{Aes256Gcm, KeyInit, Nonce, aead::Aead};

        let engine = base64::engine::general_purpose::STANDARD;
        let data = engine.decode(data)?;

        if data.len() < 12 {
            return Err(Error::Format(format!(
                "Invalid data length: {}",
                data.len()
            )));
        }

        let (nonce_bytes, encrypted_data) = data.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);

        let cipher =
            Aes256Gcm::new_from_slice(&self.key).map_err(|e| Error::Decryption(e.to_string()))?;

        let plaintext = cipher
            .decrypt(nonce, encrypted_data)
            .map_err(|e| Error::Decryption(e.to_string()))?;

        Ok(plaintext)
    }
}
