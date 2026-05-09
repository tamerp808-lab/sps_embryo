use crate::llm_chain;

pub struct APIBuilderAgent;

impl APIBuilderAgent {
    /// يُنشئ REST API بناءً على وصف قاعدة البيانات
    pub fn build_api(database_schema: &str) -> String {
        let prompt = format!(
            r#"You are a senior backend architect. Build a complete REST API in Python (FastAPI) for the following database schema:

Schema:
{}

Requirements:
- Full CRUD operations for every table
- Input validation using Pydantic
- JWT authentication
- Rate limiting
- Database migrations (Alembic)
- Swagger documentation
- Environment variable configuration
- Dockerfile for containerization
- Unit tests

Output ONLY the complete Python code. No explanations."#,
            database_schema
        );
        llm_chain::chat_sync(&prompt)
    }

    /// يُصمم مخطط قاعدة بيانات من وصف العمل
    pub fn design_schema(business_description: &str) -> String {
        let prompt = format!(
            r#"You are a database architect. Design a complete PostgreSQL schema for:
{}

Output:
1. CREATE TABLE statements with proper constraints, indexes, and foreign keys
2. An Entity Relationship Diagram in text format (Mermaid.js)
3. Seed data for testing

Output ONLY the SQL code and diagram."#,
            business_description
        );
        llm_chain::chat_sync(&prompt)
    }
}
