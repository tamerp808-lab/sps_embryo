use crate::hash::hash_state;
use crate::state::AppState;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Snapshot {
    pub tick: u64,
    pub state: AppState,
    pub state_hash: String,
    pub event_count: usize,
}

/// لقطة تزايدية: تحفظ فقط التغييرات منذ آخر لقطة
#[derive(Debug, Serialize, Deserialize)]
pub struct IncrementalSnapshot {
    pub base_tick: u64,      // آخر tick كامل
    pub new_tick: u64,       // tick الحالي
    pub delta_events: usize, // عدد الأحداث الجديدة
    pub state_delta: String, // JSON للاختلاف (مبسط)
    pub state_hash: String,
}

const SNAPSHOT_FILE: &str = "snapshot.json";
const INCREMENTAL_DIR: &str = "snapshots_delta";

/// حفظ لقطة كاملة
pub fn save_snapshot(state: &AppState, tick: u64, event_count: usize) -> std::io::Result<()> {
    let hash = hash_state(state);
    let snap = Snapshot {
        tick,
        state: state.clone(),
        state_hash: hash,
        event_count,
    };
    let json = serde_json::to_string_pretty(&snap)?;
    fs::write(SNAPSHOT_FILE, json)?;
    Ok(())
}

/// حفظ لقطة تزايدية
pub fn save_incremental_snapshot(
    state: &AppState,
    base_tick: u64,
    new_tick: u64,
    delta_events: usize,
) -> std::io::Result<()> {
    fs::create_dir_all(INCREMENTAL_DIR)?;

    let hash = hash_state(state);
    let state_json = serde_json::to_string(state)?;
    let inc = IncrementalSnapshot {
        base_tick,
        new_tick,
        delta_events,
        state_delta: state_json,
        state_hash: hash,
    };

    let file_name = format!("{}/inc_{}_{}.json", INCREMENTAL_DIR, base_tick, new_tick);
    let json = serde_json::to_string_pretty(&inc)?;
    fs::write(file_name, json)?;
    Ok(())
}

/// تحميل آخر لقطة كاملة
pub fn load_snapshot() -> std::io::Result<Option<Snapshot>> {
    if !Path::new(SNAPSHOT_FILE).exists() {
        return Ok(None);
    }
    let data = fs::read_to_string(SNAPSHOT_FILE)?;
    let snap: Snapshot = serde_json::from_str(&data)?;
    Ok(Some(snap))
}

/// تحميل أحدث لقطة تزايدية بعد tick معين
pub fn load_incremental_after(base_tick: u64) -> std::io::Result<Option<IncrementalSnapshot>> {
    if !Path::new(INCREMENTAL_DIR).exists() {
        return Ok(None);
    }

    let mut latest: Option<IncrementalSnapshot> = None;
    for entry in fs::read_dir(INCREMENTAL_DIR)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().map(|e| e == "json").unwrap_or(false) {
            let data = fs::read_to_string(&path)?;
            if let Ok(inc) = serde_json::from_str::<IncrementalSnapshot>(&data) {
                if inc.base_tick == base_tick {
                    if latest
                        .as_ref()
                        .map(|l: &IncrementalSnapshot| inc.new_tick > l.new_tick)
                        .unwrap_or(true)
                    {
                        latest = Some(inc);
                    }
                }
            }
        }
    }
    Ok(latest)
}
