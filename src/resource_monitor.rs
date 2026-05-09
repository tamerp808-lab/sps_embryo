pub struct ResourceMonitor;

impl ResourceMonitor {
    pub fn cpu_percent() -> f64 {
        let mut total = 0.0;
        let mut idle = 0.0;
        if let Ok(data) = std::fs::read_to_string("/proc/stat") {
            for line in data.lines() {
                if line.starts_with("cpu ") {
                    let fields: Vec<&str> = line.split_whitespace().collect();
                    for (i, val) in fields.iter().enumerate() {
                        if let Ok(num) = val.parse::<f64>() {
                            total += num;
                            if i == 4 {
                                idle = num;
                            }
                        }
                    }
                    break;
                }
            }
        }
        if total > 0.0 {
            100.0 * (1.0 - idle / total)
        } else {
            0.0
        }
    }

    pub fn ram_free_mb() -> f64 {
        if let Ok(data) = std::fs::read_to_string("/proc/meminfo") {
            for line in data.lines() {
                if line.starts_with("MemAvailable:") {
                    if let Some(val) = line.split_whitespace().nth(1) {
                        if let Ok(kb) = val.parse::<f64>() {
                            return kb / 1024.0;
                        }
                    }
                }
            }
        }
        0.0
    }

    pub fn battery_pct() -> u32 {
        if let Ok(output) = std::process::Command::new("termux-battery-status").output() {
            if let Ok(parsed) =
                serde_json::from_str::<serde_json::Value>(&String::from_utf8_lossy(&output.stdout))
            {
                if let Some(pct) = parsed["percentage"].as_u64() {
                    return pct as u32;
                }
            }
        }
        100
    }

    pub fn stats() -> String {
        format!(
            "CPU: {:.1}%\nRAM free: {:.0} MB\nBattery: {}%",
            Self::cpu_percent(),
            Self::ram_free_mb(),
            Self::battery_pct()
        )
    }

    pub fn is_safe() -> bool {
        Self::cpu_percent() < 80.0 && Self::battery_pct() > 15
    }
}
