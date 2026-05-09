//! إنشاء أحداث iCal

use crate::outbox::{EffectOutbox, EffectRequest, EffectType};
use std::fs;

pub struct CalendarAgent;

impl CalendarAgent {
    pub fn add_event(
        title: &str,
        start: &str, // YYYYMMDDTHHMMSS
        end: &str,
        description: &str,
        location: &str,
        outbox: &mut EffectOutbox,
    ) {
        let uid = uuid::Uuid::now_v7();
        let ics = format!(
            "BEGIN:VCALENDAR\r\nVERSION:2.0\r\nPRODID:-//SPS Embryo//EN\r\nBEGIN:VEVENT\r\nUID:{}\r\nDTSTART:{}\r\nDTEND:{}\r\nSUMMARY:{}\r\nLOCATION:{}\r\nDESCRIPTION:{}\r\nEND:VEVENT\r\nEND:VCALENDAR\r\n",
            uid, start, end, title, location, description
        );
        let path = format!("/tmp/sps_event_{}.ics", uid);
        let _ = fs::write(&path, ics);
        let effect = EffectRequest {
            id: uuid::Uuid::now_v7().as_u64_pair().0,
            tick: 0,
            effect_type: EffectType::Subprocess,
            payload: format!("echo 'ICS: {}'", path),
        };
        outbox.enqueue(effect);
    }
}
