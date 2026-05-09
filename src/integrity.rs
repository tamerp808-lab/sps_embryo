use crate::event::Event;

// =====================================================
// HASH CHAIN
// =====================================================

/// cryptographic integrity chain.
///
/// كل event hash يعتمد على:
/// - previous hash
/// - current event
///
/// أي تعديل في history
/// يكسر السلسلة بالكامل.
pub fn compute_event_hash(
    previous_hash: &str,
    event: &Event,
) -> String {

    let serialized =
        serde_json::to_string(event)
            .expect(
                "event serialization failed"
            );

    let combined =
        format!(
            "{}{}",
            previous_hash,
            serialized
        );

    blake3::hash(
        combined.as_bytes()
    )
    .to_hex()
    .to_string()
}

// =====================================================
// GENESIS HASH
// =====================================================

pub fn genesis_hash()
    -> String
{

    blake3::hash(
        b"SPS_GENESIS"
    )
    .to_hex()
    .to_string()
}
