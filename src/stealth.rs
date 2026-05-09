use std::fs;
use std::time::{Duration, SystemTime};

pub struct StealthMode;

impl StealthMode {
    pub fn run_cycle() {
        let now = SystemTime::now();
        let cutoff = now - Duration::from_secs(3600 * 24); // 24 hours
        for entry in fs::read_dir("/tmp").unwrap_or_else(|_| fs::read_dir(".").unwrap()) {
            if let Ok(entry) = entry {
                let path = entry.path();
                if let Ok(metadata) = entry.metadata() {
                    if let Ok(modified) = metadata.modified() {
                        if modified < cutoff && path.is_file() {
                            let _ = fs::remove_file(path);
                        }
                    }
                }
            }
        }
    }
}
