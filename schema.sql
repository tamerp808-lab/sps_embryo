CREATE TABLE long_term_memory (id INTEGER PRIMARY KEY, role TEXT, content TEXT, importance REAL);
CREATE TABLE episodic_memory (id INTEGER PRIMARY KEY, event_type TEXT, description TEXT);
CREATE TABLE semantic_relations (subject TEXT, predicate TEXT, object TEXT);
