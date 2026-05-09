pub struct AutoAudit;

impl AutoAudit {
    pub fn log(action: &str, details: &str) {
        println!("📝 AUDIT: {} - {}", action, details);
    }
}
