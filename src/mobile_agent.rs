use crate::outbox::{EffectOutbox, EffectRequest, EffectType};

pub struct MobileControlAgent;

impl MobileControlAgent {
    pub fn open_app(pkg: &str, outbox: &mut EffectOutbox) {
        let cmd = format!(
            "am start -n {}/.MainActivity 2>/dev/null || echo 'FAILED'",
            pkg
        );
        outbox.enqueue(EffectRequest {
            id: uuid::Uuid::now_v7().as_u64_pair().0,
            tick: 0,
            effect_type: EffectType::Subprocess,
            payload: cmd,
        });
    }
    pub fn input_text(text: &str, outbox: &mut EffectOutbox) {
        let cmd = format!(
            "input text '{}' 2>/dev/null || echo 'FAILED'",
            text.replace('\'', "'\\''")
        );
        outbox.enqueue(EffectRequest {
            id: uuid::Uuid::now_v7().as_u64_pair().0,
            tick: 0,
            effect_type: EffectType::Subprocess,
            payload: cmd,
        });
    }
    pub fn press_key(keycode: u32, outbox: &mut EffectOutbox) {
        let cmd = format!("input keyevent {} 2>/dev/null || echo 'FAILED'", keycode);
        outbox.enqueue(EffectRequest {
            id: uuid::Uuid::now_v7().as_u64_pair().0,
            tick: 0,
            effect_type: EffectType::Subprocess,
            payload: cmd,
        });
    }
    pub fn screenshot(path: &str, outbox: &mut EffectOutbox) {
        let cmd = format!("screencap {} 2>/dev/null || echo 'FAILED'", path);
        outbox.enqueue(EffectRequest {
            id: uuid::Uuid::now_v7().as_u64_pair().0,
            tick: 0,
            effect_type: EffectType::Subprocess,
            payload: cmd,
        });
    }
    pub fn swipe(x1: u32, y1: u32, x2: u32, y2: u32, outbox: &mut EffectOutbox) {
        if x1 > 3000 || y1 > 3000 || x2 > 3000 || y2 > 3000 {
            return;
        }
        let cmd = format!(
            "input swipe {} {} {} {} 2>/dev/null || echo 'FAILED'",
            x1, y1, x2, y2
        );
        outbox.enqueue(EffectRequest {
            id: uuid::Uuid::now_v7().as_u64_pair().0,
            tick: 0,
            effect_type: EffectType::Subprocess,
            payload: cmd,
        });
    }
    pub fn tap(x: u32, y: u32, outbox: &mut EffectOutbox) {
        if x > 3000 || y > 3000 {
            return;
        }
        let cmd = format!("input tap {} {} 2>/dev/null || echo 'FAILED'", x, y);
        outbox.enqueue(EffectRequest {
            id: uuid::Uuid::now_v7().as_u64_pair().0,
            tick: 0,
            effect_type: EffectType::Subprocess,
            payload: cmd,
        });
    }
    pub fn get_clipboard(outbox: &mut EffectOutbox) {
        let cmd = "termux-clipboard-get 2>/dev/null || echo 'FAILED'".to_string();
        outbox.enqueue(EffectRequest {
            id: uuid::Uuid::now_v7().as_u64_pair().0,
            tick: 0,
            effect_type: EffectType::Subprocess,
            payload: cmd,
        });
    }
    pub fn set_clipboard(text: &str, outbox: &mut EffectOutbox) {
        let cmd = format!(
            "echo '{}' | termux-clipboard-set 2>/dev/null || echo 'FAILED'",
            text
        );
        outbox.enqueue(EffectRequest {
            id: uuid::Uuid::now_v7().as_u64_pair().0,
            tick: 0,
            effect_type: EffectType::Subprocess,
            payload: cmd,
        });
    }
    pub fn notification(title: &str, msg: &str, outbox: &mut EffectOutbox) {
        let cmd = format!(
            "termux-notification --title '{}' --content '{}' 2>/dev/null || echo 'FAILED'",
            title, msg
        );
        outbox.enqueue(EffectRequest {
            id: uuid::Uuid::now_v7().as_u64_pair().0,
            tick: 0,
            effect_type: EffectType::Subprocess,
            payload: cmd,
        });
    }
    pub fn screenshot_now() {
        let _ = std::process::Command::new("screencap")
            .arg("/sdcard/sps_screenshot.png")
            .status();
    }
}
