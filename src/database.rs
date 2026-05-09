use rusqlite::{Connection, Result};
use std::path::PathBuf;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn open(path: PathBuf) -> Result<Self> {
        let conn = Connection::open(path)?;
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS tasks (
                id TEXT PRIMARY KEY, desc TEXT, status TEXT, result TEXT, reward REAL
            );
            CREATE TABLE IF NOT EXISTS memories (
                key TEXT PRIMARY KEY, code TEXT, task TEXT, reward REAL, usage INTEGER DEFAULT 1
            );
            CREATE TABLE IF NOT EXISTS agents (
                role TEXT PRIMARY KEY, code TEXT, success_count INTEGER DEFAULT 0, fail_count INTEGER DEFAULT 0
            );
            CREATE TABLE IF NOT EXISTS goals (
                id TEXT PRIMARY KEY, description TEXT, status TEXT, priority INTEGER
            );
            CREATE TABLE IF NOT EXISTS plans (
                id TEXT PRIMARY KEY, goal TEXT, steps TEXT, current_step INTEGER, status TEXT
            );"
        )?;
        Ok(Self { conn })
    }

    pub fn save_task(
        &self,
        id: &str,
        desc: &str,
        status: &str,
        result: &str,
        reward: f64,
    ) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO tasks VALUES (?1,?2,?3,?4,?5)",
            (id, desc, status, result, reward),
        )?;
        Ok(())
    }

    pub fn get_top_memories(&self, limit: i64) -> Vec<String> {
        let mut stmt = self
            .conn
            .prepare("SELECT task FROM memories WHERE reward > 0.3 ORDER BY usage DESC LIMIT ?")
            .unwrap();
        let rows = stmt.query_map([limit], |row| row.get(0)).unwrap();
        rows.filter_map(|r| r.ok()).collect()
    }

    pub fn add_memory(&self, key: &str, code: &str, task: &str, reward: f64) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO memories (key, code, task, reward) VALUES (?1,?2,?3,?4)",
            (key, code, task, reward),
        )?;
        Ok(())
    }

    pub fn save_agent(&self, role: &str, code: &str) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO agents (role,code) VALUES (?1,?2)",
            (role, code),
        )?;
        Ok(())
    }

    pub fn add_goal(&self, id: &str, desc: &str, priority: i32) -> Result<()> {
        self.conn.execute(
            "INSERT INTO goals (id,description,status,priority) VALUES (?1,?2,'pending',?3)",
            (id, desc, priority),
        )?;
        Ok(())
    }

    pub fn save_plan(
        &self,
        id: &str,
        goal: &str,
        steps: &str,
        current: i32,
        status: &str,
    ) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO plans VALUES (?1,?2,?3,?4,?5)",
            (id, goal, steps, current, status),
        )?;
        Ok(())
    }
}
