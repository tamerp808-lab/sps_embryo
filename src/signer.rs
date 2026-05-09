use ed25519_dalek::{SecretKey, Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand_core::{OsRng, RngCore};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SignedEvent {

    pub event_json: String,
    pub signature: Vec<u8>,
    pub public_key: Vec<u8>,
}

pub struct EventSigner {
    signing_key: SigningKey,
}

impl EventSigner {
    pub fn new() -> Self {
        let mut csprng = OsRng;
        let mut secret = SecretKey::default();
        csprng.fill_bytes(&mut secret);
        let signing_key = SigningKey::from_bytes(&secret);
        Self { signing_key }
    }

    pub fn sign<T: Serialize>(&self, data: &T) -> Result<SignedEvent, Box<dyn std::error::Error>> {
        let event_json = serde_json::to_string(data)?;
        let signature = self
            .signing_key
            .sign(event_json.as_bytes())
            .to_bytes()
            .to_vec();
        let public_key = self.signing_key.verifying_key().to_bytes().to_vec();
        Ok(SignedEvent {

            event_json,
            signature,
            public_key,
        })
    }

    pub fn verify(signed: &SignedEvent) -> bool {
        let public_key_bytes: &[u8; 32] = match signed.public_key.as_slice().try_into() {
            Ok(bytes) => bytes,
            Err(_) => return false,
        };
        let Ok(public_key) = VerifyingKey::from_bytes(public_key_bytes) else {
            return false;
        };
        let Ok(signature) = Signature::from_slice(&signed.signature) else {
            return false;
        };
        public_key
            .verify(signed.event_json.as_bytes(), &signature)
            .is_ok()
    }

    pub fn verifying_key_bytes(&self) -> Vec<u8> {
        self.signing_key.verifying_key().to_bytes().to_vec()
    }
}
