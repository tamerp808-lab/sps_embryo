use crate::state::AppState;

pub fn hash_state(state: &AppState) -> String {
    let serialized = serde_json::to_string(state).expect("Serialization must succeed");
    let hash = blake3::hash(serialized.as_bytes());
    hash.to_hex().to_string()
}
