/// طبقة التنفيذ – تفصل الأوامر عن التأثيرات الجانبية
pub struct ExecutionRuntime;

impl ExecutionRuntime {
    /// يُعالج الأحداث الحية ويُقرر ما إذا كان يجب إنشاء تأثير
    pub fn process_live_event(
        event_id: u64,
        tick: u64,
        payload: &str,
        outbox: &mut crate::outbox::EffectOutbox,
    ) {
        if payload.starts_with("EXEC:") {
            let cmd = &payload[5..];
            match cmd {
                "spawn_subprocess" => {
                    let effect = crate::outbox::EffectRequest {
                        id: event_id,
                        tick,
                        effect_type: crate::outbox::EffectType::Subprocess,
                        payload: "echo 'Execution Runtime: Hello!'".to_string(),
                    };
                    outbox.enqueue(effect);
                }
                _ => {}
            }
        }
    }

    /// يُنفذ جميع الآثار المعلقة في الـ Outbox
    pub fn run_effects(outbox: &mut crate::outbox::EffectOutbox) -> Vec<String> {
        crate::sandbox::process_pending_effects(outbox)
    }
}
