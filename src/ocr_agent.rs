//! استخراج النص من الصور باستخدام tesseract

use crate::outbox::{EffectOutbox, EffectRequest, EffectType};

pub struct OCRAgent;

impl OCRAgent {
    pub fn extract_text(image_path: &str, lang: &str, outbox: &mut EffectOutbox) {
        let cmd = format!(
            "tesseract '{}' stdout -l {} 2>/dev/null || echo 'OCR_FAILED'",
            image_path, lang
        );
        let effect = EffectRequest {
            id: uuid::Uuid::now_v7().as_u64_pair().0,
            tick: 0,
            effect_type: EffectType::Subprocess,
            payload: cmd,
        };
        outbox.enqueue(effect);
    }
}
