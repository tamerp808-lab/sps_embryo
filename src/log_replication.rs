use crate::event::Event;
use crate::event_log;

pub struct LogReplication;

impl LogReplication {
    /// يُرسل الأحداث المفقودة إلى نظير
    pub fn sync_with_peer(
        peer_address: &str,
        last_tick: u64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let all_events = event_log::read_all_events()?;
        let missing: Vec<&Event> = all_events.iter().filter(|e| e.tick > last_tick).collect();

        if missing.is_empty() {
            return Ok(());
        }

        let body = serde_json::to_string(&missing)?;
        let output = std::process::Command::new("curl")
            .args([
                "-s",
                "-X",
                "POST",
                &format!("http://{}/sync_events", peer_address),
                "-H",
                "Content-Type: application/json",
                "-d",
                &body,
            ])
            .output()?;

        if output.status.success() {
            println!("📤 Synced {} events to {}", missing.len(), peer_address);
        }
        Ok(())
    }
}
