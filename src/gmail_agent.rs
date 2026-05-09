//! إرسال بريد عبر sendmail

use crate::outbox::{EffectOutbox, EffectRequest, EffectType};

pub struct GmailAgent;

impl GmailAgent {
    pub fn send_email(to: &str, subject: &str, body: &str, outbox: &mut EffectOutbox) {
        let cmd = format!(
            "echo -e 'Subject: {}\nTo: {}\n\n{}' | sendmail {} 2>/dev/null || echo 'SENDMAIL_FAILED'",
            subject, to, body, to
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
