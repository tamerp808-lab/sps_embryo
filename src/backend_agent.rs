use crate::llm_chain;

pub struct BackendAgent;

impl BackendAgent {
    pub fn build_backend(description: &str) -> String {
        let prompt = format!(
            r#"You are a senior backend developer. Build a complete Python FastAPI backend for this description:
{}

Include:
- Database models (SQLAlchemy)
- CRUD API endpoints
- Authentication (JWT)
- Swagger documentation
- Dockerfile
- Requirements.txt

Output ALL files as a single ZIP-ready structure with file paths and contents."#,
            description
        );
        llm_chain::chat_sync(&prompt)
    }
}
