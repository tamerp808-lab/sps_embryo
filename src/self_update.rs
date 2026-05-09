//! التحديث الذاتي – فحص الإصدار وتنزيل التحديث باستخدام curl

use std::path::PathBuf;
use std::process::Command;

pub fn check_and_update(
    current_version: &str,
    update_url: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    // 1. فحص الإصدار عبر curl
    let output = Command::new("curl").args(["-sI", update_url]).output()?;
    let headers = String::from_utf8_lossy(&output.stdout);
    let latest_version = headers
        .lines()
        .find(|l| l.to_lowercase().starts_with("x-version:"))
        .and_then(|l| l.split(':').nth(1))
        .map(|s| s.trim())
        .unwrap_or("unknown");

    if latest_version == current_version {
        return Ok(false);
    }

    // 2. تنزيل الملف الثنائي الجديد
    let tmp_path = "/tmp/sps_new_binary";
    let status = Command::new("curl")
        .args(["-sL", "-o", tmp_path, update_url])
        .status()?;

    if !status.success() || !PathBuf::from(tmp_path).exists() {
        return Err("Download failed".into());
    }

    // 3. استبدال الملف الثنائي الحالي
    let current_exe = std::env::current_exe()?;
    std::fs::rename(tmp_path, &current_exe)?;
    Ok(true)
}
