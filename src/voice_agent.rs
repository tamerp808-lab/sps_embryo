use crate::outbox::{EffectOutbox, EffectRequest, EffectType};

pub struct VoiceAgent;

impl VoiceAgent {
    pub fn speak(text: &str, lang: &str, outbox: &mut EffectOutbox) {
        let cmd = if lang == "ar" {
            format!("espeak-ng -v ar '{}' 2>/dev/null || echo 'FAILED'", text)
        } else {
            format!("espeak-ng '{}' 2>/dev/null || echo 'FAILED'", text)
        };
        outbox.enqueue(EffectRequest {
            id: uuid::Uuid::now_v7().as_u64_pair().0,
            tick: 0,
            effect_type: EffectType::Subprocess,
            payload: cmd,
        });
    }
    pub fn listen(duration: usize, outbox: &mut EffectOutbox) {
        let cmd = format!("arecord -d {} -f cd -t wav /tmp/sps_rec.wav 2>/dev/null && whisper /tmp/sps_rec.wav --model base --language ar --output_format txt --output_dir /tmp/ 2>/dev/null || echo 'FAILED'", duration);
        outbox.enqueue(EffectRequest {
            id: uuid::Uuid::now_v7().as_u64_pair().0,
            tick: 0,
            effect_type: EffectType::Subprocess,
            payload: cmd,
        });
    }
    pub fn list_audio(outbox: &mut EffectOutbox) {
        let cmd = "ls /tmp/sps_tts* 2>/dev/null || echo 'None'".to_string();
        outbox.enqueue(EffectRequest {
            id: uuid::Uuid::now_v7().as_u64_pair().0,
            tick: 0,
            effect_type: EffectType::Subprocess,
            payload: cmd,
        });
    }
    pub fn stop_speaking(outbox: &mut EffectOutbox) {
        let cmd =
            "killall espeak-ng 2>/dev/null; killall mpv 2>/dev/null || echo 'Done'".to_string();
        outbox.enqueue(EffectRequest {
            id: uuid::Uuid::now_v7().as_u64_pair().0,
            tick: 0,
            effect_type: EffectType::Subprocess,
            payload: cmd,
        });
    }
}
