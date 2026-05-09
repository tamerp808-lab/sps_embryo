use rusqlite::{Connection, Result as SqlResult};
use std::collections::VecDeque;
use std::path::PathBuf;
use std::sync::Mutex;

pub struct ShortTermMemory {
    messages: VecDeque<(String, String)>,
    capacity: usize,
}

impl ShortTermMemory {
    pub fn new(capacity: usize) -> Self {
        Self {
            messages: VecDeque::with_capacity(capacity),
            capacity,
        }
    }
    pub fn add(&mut self, role: &str, content: &str) {
        if self.messages.len() >= self.capacity {
            self.messages.pop_front();
        }
        self.messages
            .push_back((role.to_string(), content.to_string()));
    }
    pub fn context(&self) -> Vec<(String, String)> {
        self.messages.iter().cloned().collect()
    }
    pub fn formatted(&self) -> String {
        self.messages
            .iter()
            .map(|(r, c)| format!("{}: {}", r, c))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

pub struct LongTermMemory {
    conn: Mutex<Connection>,
}

unsafe impl Send for LongTermMemory {}
unsafe impl Sync for LongTermMemory {}

impl LongTermMemory {
    pub fn open(path: PathBuf) -> SqlResult<Self> {
        let conn = Connection::open(path)?;
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS long_term_memory (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                role TEXT,
                content TEXT,
                embedding BLOB,
                importance REAL DEFAULT 0.5,
                created_at TEXT DEFAULT (datetime('now'))
            );
            CREATE TABLE IF NOT EXISTS episodic_memory (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                event_type TEXT,
                description TEXT,
                importance REAL DEFAULT 0.5,
                created_at TEXT DEFAULT (datetime('now'))
            );
            CREATE TABLE IF NOT EXISTS semantic_relations (
                subject TEXT,
                predicate TEXT,
                object TEXT,
                created_at TEXT DEFAULT (datetime('now'))
            );",
        )?;
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    pub fn store(
        &self,
        role: &str,
        content: &str,
        _embedding: Option<&[f32]>,
        importance: f64,
    ) -> SqlResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO long_term_memory (role, content, importance) VALUES (?1, ?2, ?3)",
            (role, content, importance),
        )?;
        Ok(())
    }

    pub fn retrieve_top(&self, limit: usize) -> Vec<(String, String)> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare("SELECT role, content FROM long_term_memory ORDER BY importance DESC LIMIT ?")
            .unwrap();
        let rows = stmt
            .query_map([limit as i64], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
            })
            .unwrap();
        rows.filter_map(|r| r.ok()).collect()
    }

    pub fn record_episode(
        &self,
        event_type: &str,
        description: &str,
        importance: f64,
    ) -> SqlResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO episodic_memory (event_type, description, importance) VALUES (?1, ?2, ?3)",
            (event_type, description, importance),
        )?;
        Ok(())
    }

    pub fn recall_episodes(&self, limit: usize) -> Vec<String> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare("SELECT description FROM episodic_memory ORDER BY importance DESC LIMIT ?")
            .unwrap();
        let rows = stmt
            .query_map([limit as i64], |row| row.get::<_, String>(0))
            .unwrap();
        rows.filter_map(|r| r.ok()).collect()
    }

    pub fn add_semantic_relation(
        &self,
        subject: &str,
        predicate: &str,
        object: &str,
    ) -> SqlResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO semantic_relations (subject, predicate, object) VALUES (?1, ?2, ?3)",
            (subject, predicate, object),
        )?;
        Ok(())
    }

    pub fn get_semantic_relations(&self, subject: &str) -> Vec<(String, String)> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare("SELECT predicate, object FROM semantic_relations WHERE subject = ?1")
            .unwrap();
        stmt.query_map([subject], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })
        .unwrap()
        .filter_map(|r| r.ok())
        .collect()
    }
}

pub struct VectorMemory {
    vectors: Vec<Vec<f32>>,
    texts: Vec<String>,
}

impl VectorMemory {
    pub fn new() -> Self {
        Self {
            vectors: Vec::new(),
            texts: Vec::new(),
        }
    }
    pub fn insert(&mut self, text: String, embedding: Vec<f32>) {
        self.vectors.push(embedding);
        self.texts.push(text);
    }
    pub fn search(&self, query_emb: &[f32], top_k: usize) -> Vec<String> {
        if self.texts.is_empty() {
            return vec![];
        }
        let mut scored: Vec<(f64, String)> = self
            .vectors
            .iter()
            .enumerate()
            .map(|(i, v)| (cosine_similarity(query_emb, v), self.texts[i].clone()))
            .collect();
        scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        scored.into_iter().take(top_k).map(|(_, s)| s).collect()
    }
}

fn cosine_similarity(a: &[f32], b: &[f32]) -> f64 {
    let dot: f64 = a.iter().zip(b).map(|(x, y)| *x as f64 * *y as f64).sum();
    let na: f64 = a.iter().map(|x| *x as f64 * *x as f64).sum::<f64>().sqrt();
    let nb: f64 = b.iter().map(|x| *x as f64 * *x as f64).sum::<f64>().sqrt();
    if na < 1e-8 || nb < 1e-8 {
        return 0.0;
    }
    dot / (na * nb)
}
