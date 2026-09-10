use crate::error::Result;
use rusqlite::Connection;

const MIGRATIONS: &[(&str, &str)] = &[(
    "001_initial",
    r#"
        CREATE TABLE IF NOT EXISTS projects (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            description TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS milestones (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            project_id INTEGER NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
            name TEXT NOT NULL,
            version TEXT,
            target_date DATE,
            status TEXT DEFAULT 'active' CHECK(status IN ('active', 'completed')),
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            project_id INTEGER NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
            milestone_id INTEGER REFERENCES milestones(id) ON DELETE SET NULL,
            parent_id INTEGER REFERENCES tasks(id) ON DELETE SET NULL,
            type TEXT NOT NULL CHECK(type IN ('requirement', 'task', 'bug')),
            title TEXT NOT NULL,
            description TEXT,
            status TEXT DEFAULT 'todo' CHECK(status IN ('todo', 'in_progress', 'review', 'done')),
            priority TEXT DEFAULT 'normal' CHECK(priority IN ('low', 'normal', 'high', 'urgent')),
            assignee TEXT,
            due_date DATE,
            actor TEXT DEFAULT 'human' CHECK(actor IN ('ai', 'human')),
            steps_to_reproduce TEXT,
            severity TEXT CHECK(severity IN ('minor', 'normal', 'critical', 'fatal')),
            environment TEXT,
            user_story TEXT,
            acceptance_criteria TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS task_links (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            source_id INTEGER NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
            target_id INTEGER NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
            type TEXT NOT NULL CHECK(type IN ('requires', 'blocks', 'related'))
        );

        CREATE TABLE IF NOT EXISTS time_entries (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            task_id INTEGER NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
            actor TEXT NOT NULL CHECK(actor IN ('ai', 'human')),
            duration INTEGER NOT NULL,
            start_time DATETIME,
            end_time DATETIME,
            note TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS documents (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            project_id INTEGER NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
            title TEXT NOT NULL,
            content TEXT,
            status TEXT DEFAULT 'draft' CHECK(status IN ('draft', 'review', 'published', 'archived')),
            tags TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS document_links (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            document_id INTEGER NOT NULL REFERENCES documents(id) ON DELETE CASCADE,
            task_id INTEGER NOT NULL REFERENCES tasks(id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS schema_migrations (
            version TEXT PRIMARY KEY,
            applied_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );
    "#,
)];

pub fn run_migrations(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS schema_migrations (
            version TEXT PRIMARY KEY,
            applied_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );",
    )?;

    for (version, sql) in MIGRATIONS {
        let already_applied: bool = conn.query_row(
            "SELECT COUNT(*) > 0 FROM schema_migrations WHERE version = ?1",
            [version],
            |row| row.get(0),
        )?;

        if !already_applied {
            conn.execute_batch(sql)?;
            conn.execute(
                "INSERT INTO schema_migrations (version) VALUES (?1)",
                [version],
            )?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::connection::get_memory_connection;

    #[test]
    fn test_migrations_run_on_empty_db() {
        let conn = get_memory_connection().unwrap();
        run_migrations(&conn).unwrap();

        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM schema_migrations", [], |row| {
                row.get(0)
            })
            .unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn test_migrations_are_idempotent() {
        let conn = get_memory_connection().unwrap();
        run_migrations(&conn).unwrap();
        run_migrations(&conn).unwrap();

        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM schema_migrations", [], |row| {
                row.get(0)
            })
            .unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn test_tables_exist_after_migration() {
        let conn = get_memory_connection().unwrap();
        run_migrations(&conn).unwrap();

        let tables: Vec<String> = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name")
            .unwrap()
            .query_map([], |row| row.get(0))
            .unwrap()
            .filter_map(|r| r.ok())
            .collect();

        assert!(tables.contains(&"projects".to_string()));
        assert!(tables.contains(&"milestones".to_string()));
        assert!(tables.contains(&"tasks".to_string()));
        assert!(tables.contains(&"task_links".to_string()));
        assert!(tables.contains(&"time_entries".to_string()));
        assert!(tables.contains(&"documents".to_string()));
        assert!(tables.contains(&"document_links".to_string()));
    }
}
