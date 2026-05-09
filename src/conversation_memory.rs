use rusqlite::{Connection, Result};
use std::path::PathBuf;
use std::sync::Mutex;

pub struct ConversationMemory {
    conn: Mutex<Connection>,
}

// نُخبر المُصرّف بأن النوع آمن للإرسال والمشاركة بين الخيوط
unsafe impl Send for ConversationMemory {}
unsafe impl Sync for ConversationMemory {}

impl ConversationMemory {
    pub fn open(path: PathBuf) -> Result<Self> {
        let conn = Connection::open(path)?;
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS conversations (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                role TEXT NOT NULL,
                content TEXT NOT NULL,
                timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
            );",
        )?;
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    pub fn add_message(&self, role: &str, content: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO conversations (role, content) VALUES (?1, ?2)",
            (role, content),
        )?;
        Ok(())
    }

    pub fn get_context(&self, limit: usize) -> Vec<(String, String)> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare("SELECT role, content FROM conversations ORDER BY id DESC LIMIT ?")
            .unwrap();
        let rows = stmt
            .query_map([limit as i64], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
            })
            .unwrap();
        let mut messages: Vec<(String, String)> = rows.filter_map(|r| r.ok()).collect();
        messages.reverse();
        messages
    }
}
