use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use rand::Rng;

pub struct LogEncryption {
    cipher: Aes256Gcm,
}

impl LogEncryption {
    pub fn new(key: &[u8; 32]) -> Self {
        let key = Key::<Aes256Gcm>::from_slice(key);
        Self {
            cipher: Aes256Gcm::new(key),
        }
    }

    pub fn encrypt(&self, plaintext: &str) -> Vec<u8> {
        let nonce_bytes: [u8; 12] = rand::thread_rng().gen();
        let nonce = Nonce::from_slice(&nonce_bytes);
        let ciphertext = self.cipher.encrypt(nonce, plaintext.as_bytes()).unwrap();
        [nonce_bytes.to_vec(), ciphertext].concat()
    }

    pub fn decrypt(&self, data: &[u8]) -> Option<String> {
        if data.len() < 12 {
            return None;
        }
        let (nonce_bytes, ciphertext) = data.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);
        self.cipher
            .decrypt(nonce, ciphertext)
            .ok()
            .map(|v| String::from_utf8(v).unwrap_or_default())
    }
}
