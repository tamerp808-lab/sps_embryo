use crate::event_log::{read_all_events, Event};
use std::fs;
use std::path::PathBuf;

const COMPACTED_DIR: &str = "event_log_archive";

/// يضغط السجل: يحفظ الأحداث القديمة في مجلد أرشيف ويحذفها من السجل الحي
pub fn compact_log(keep_recent: usize) -> Result<(), Box<dyn std::error::Error>> {
    let events = read_all_events()?;
    if events.len() <= keep_recent {
        return Ok(()); // لا حاجة للضغط
    }

    let split_idx = events.len() - keep_recent;
    let old_events = &events[..split_idx];
    let recent_events = &events[split_idx..];

    // إنشاء مجلد الأرشيف إذا لم يكن موجودًا
    fs::create_dir_all(COMPACTED_DIR)?;

    // كتابة الأحداث القديمة في ملف أرشيف
    let archive_path = PathBuf::from(COMPACTED_DIR).join(format!(
        "archived_{}_{}.jsonl",
        old_events.first().map(|e| e.tick).unwrap_or(0),
        old_events.last().map(|e| e.tick).unwrap_or(0)
    ));
    let mut archive_content = String::new();
    for event in old_events {
        archive_content.push_str(&serde_json::to_string(event)?);
        archive_content.push('\n');
    }
    fs::write(&archive_path, archive_content)?;

    // إعادة كتابة السجل الحي بالأحداث الحديثة فقط
    let live_path = "event_log.jsonl";
    let mut live_content = String::new();
    for event in recent_events {
        live_content.push_str(&serde_json::to_string(event)?);
        live_content.push('\n');
    }
    fs::write(live_path, live_content)?;

    println!(
        "📦 Log compacted: {} old events archived, {} kept.",
        old_events.len(),
        recent_events.len()
    );

    Ok(())
}

/// يعيد بناء الحالة من الأرشيف + السجل الحي
pub fn restore_from_archive() -> Result<Vec<Event>, Box<dyn std::error::Error>> {
    let mut all_events = Vec::new();

    // قراءة جميع ملفات الأرشيف
    if let Ok(dir) = fs::read_dir(COMPACTED_DIR) {
        for entry in dir {
            let entry = entry?;
            let path = entry.path();
            if path.extension().map(|e| e == "jsonl").unwrap_or(false) {
                let content = fs::read_to_string(&path)?;
                for line in content.lines() {
                    if line.trim().is_empty() {
                        continue;
                    }
                    let event: Event = serde_json::from_str(line)?;
                    all_events.push(event);
                }
            }
        }
    }

    // إضافة الأحداث الحية
    let live_events = read_all_events()?;
    all_events.extend(live_events);

    // ترتيب حتمي
    all_events.sort_by(|a, b| a.tick.cmp(&b.tick).then(a.id.cmp(&b.id)));
    Ok(all_events)
}
