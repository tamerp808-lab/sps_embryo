use crate::outbox::{EffectOutbox, EffectRequest, EffectType};

pub struct BrowserAgent;

impl BrowserAgent {
    /// يُرجع true إذا كان الرابط غير آمن (شبكة داخلية أو بروتوكول خطير)
    fn is_url_unsafe(url: &str) -> bool {
        let url_lower = url.to_lowercase();
        url_lower.starts_with("http://127.")
            || url_lower.starts_with("http://10.")
            || url_lower.starts_with("http://192.168.")
            || url_lower.starts_with("http://172.")
            || url_lower.starts_with("http://169.254.")
            || url_lower.starts_with("http://0.0.0.0")
            || url_lower.starts_with("http://[::1]")
            || url_lower.starts_with("http://localhost")
            || url_lower.starts_with("file://")
            || url_lower.starts_with("ftp://")
            || url_lower.starts_with("gopher://")
            || url_lower.starts_with("dict://")
            || url_lower.starts_with("ldap://")
            || url_lower.starts_with("tftp://")
    }

    pub fn fetch_page(url: &str, outbox: &mut EffectOutbox) {
        if Self::is_url_unsafe(url) {
            return; // رفض الطلب
        }
        let cmd = format!("curl -sL '{}' 2>/dev/null | head -c 5000", url);
        outbox.enqueue(EffectRequest {
            id: uuid::Uuid::now_v7().as_u64_pair().0,
            tick: 0,
            effect_type: EffectType::Subprocess,
            payload: cmd,
        });
    }

    pub fn search_web(query: &str, outbox: &mut EffectOutbox) {
        let q = query.replace(' ', "+");
        Self::fetch_page(
            &format!("https://lite.duckduckgo.com/lite/?q={}", q),
            outbox,
        );
    }

    pub fn screenshot(url: &str, outbox: &mut EffectOutbox) {
        let path = format!("/tmp/sps_browser_{}.png", uuid::Uuid::now_v7());
        if Self::is_url_unsafe(url) {
            return;
        }
        let cmd = format!(
            "curl -sL '{}' | wkhtmltoimage - {} 2>/dev/null || echo 'FAILED'",
            url, path
        );
        outbox.enqueue(EffectRequest {
            id: uuid::Uuid::now_v7().as_u64_pair().0,
            tick: 0,
            effect_type: EffectType::Subprocess,
            payload: cmd,
        });
    }
}
