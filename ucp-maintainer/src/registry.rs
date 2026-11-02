//! SQLite‑backed local spec registry (store, list, show, delete).

use rusqlite::{params, Connection};
use serde_json;
use ucp_core::Result;
use ucp_synthesizer::pipeline::SynthesisOutput;

/// A local SQLite-backed store for UCP specs.
pub struct SpecStore {
    conn: Connection,
}

impl SpecStore {
    /// Open (or create) a spec store at the given file path.
    /// Use ":memory:" for an in-memory store.
    /// # Errors
    ///
    /// Errors when the SQLite database file cannot be opened or its schema
    /// cannot be initialized.
    pub fn open(path: &str) -> Result<Self> {
        let conn =
            Connection::open(path).map_err(|e| ucp_core::UcpError::Io(std::io::Error::other(e)))?;

        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS specs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                content TEXT NOT NULL,
                created_at TEXT NOT NULL DEFAULT (datetime('now'))
            );",
        )
        .map_err(|e| ucp_core::UcpError::Io(std::io::Error::other(e)))?;

        Ok(Self { conn })
    }

    /// Store a spec and return its unique ID.
    /// # Errors
    ///
    /// Errors when serialization or the SQL insert fails.
    pub fn store(&self, name: &str, spec: &SynthesisOutput) -> Result<i64> {
        let json = serde_json::to_string(spec).map_err(ucp_core::UcpError::Json)?;

        self.conn
            .execute(
                "INSERT INTO specs (name, content) VALUES (?1, ?2)",
                params![name, json],
            )
            .map_err(|e| ucp_core::UcpError::Io(std::io::Error::other(e)))?;

        Ok(self.conn.last_insert_rowid())
    }

    /// Retrieve a spec by ID.
    /// # Errors
    ///
    /// Errors when the row cannot be read or its JSON payload is invalid.
    pub fn get(&self, id: i64) -> Result<Option<SynthesisOutput>> {
        let mut stmt = self
            .conn
            .prepare("SELECT content FROM specs WHERE id = ?1")
            .map_err(|e| ucp_core::UcpError::Io(std::io::Error::other(e)))?;

        let result = stmt.query_row(params![id], |row| {
            let content: String = row.get(0)?;
            Ok(content)
        });

        match result {
            Ok(content) => {
                let spec = serde_json::from_str(&content).map_err(ucp_core::UcpError::Json)?;
                Ok(Some(spec))
            }
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(ucp_core::UcpError::Io(std::io::Error::other(e))),
        }
    }

    /// List all stored specs (id and name only).
    /// # Errors
    ///
    /// Errors when the SQL query fails.
    pub fn list(&self) -> Result<Vec<(i64, String)>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name FROM specs ORDER BY created_at DESC")
            .map_err(|e| ucp_core::UcpError::Io(std::io::Error::other(e)))?;

        let rows = stmt
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
            .map_err(|e| ucp_core::UcpError::Io(std::io::Error::other(e)))?;

        let mut results = Vec::new();
        for row in rows {
            results.push(row.map_err(|e| ucp_core::UcpError::Io(std::io::Error::other(e)))?);
        }
        Ok(results)
    }

    /// Delete a spec by ID. Returns true if a row was deleted.
    /// # Errors
    ///
    /// Errors when the SQL delete fails.
    pub fn delete(&self, id: i64) -> Result<bool> {
        let affected = self
            .conn
            .execute("DELETE FROM specs WHERE id = ?1", params![id])
            .map_err(|e| ucp_core::UcpError::Io(std::io::Error::other(e)))?;
        Ok(affected > 0)
    }

    /// Return the number of stored specs.
    /// # Errors
    ///
    /// Errors when the SQL count fails.
    pub fn count(&self) -> Result<usize> {
        let count: i64 = self
            .conn
            .query_row("SELECT COUNT(*) FROM specs", [], |row| row.get(0))
            .map_err(|e| ucp_core::UcpError::Io(std::io::Error::other(e)))?;
        Ok(usize::try_from(count).unwrap_or(0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use ucp_synthesizer::pipeline::PipelineStats;

    fn empty_spec() -> SynthesisOutput {
        SynthesisOutput {
            ucp_version: "4.0.0".into(),
            components: vec![],
            stats: PipelineStats {
                files_scanned: 0,
                files_parsed: 0,
                components_found: 0,
                conflicts_detected: 0,
                llm_enriched: false,
            },
            provenance: None,
            curation_log: None,
        }
    }

    #[test]
    fn can_store_and_retrieve() {
        let store = SpecStore::open(":memory:").unwrap();
        let spec = empty_spec();
        let id = store.store("test-spec", &spec).unwrap();
        let retrieved = store.get(id).unwrap().unwrap();
        assert_eq!(retrieved.ucp_version, "4.0.0");
    }

    #[test]
    fn list_returns_stored_specs() {
        let store = SpecStore::open(":memory:").unwrap();
        store.store("a", &empty_spec()).unwrap();
        store.store("b", &empty_spec()).unwrap();
        let list = store.list().unwrap();
        assert_eq!(list.len(), 2);
        let names: Vec<&str> = list.iter().map(|(_, name)| name.as_str()).collect();
        assert!(names.contains(&"a"));
        assert!(names.contains(&"b"));
    }

    #[test]
    fn get_nonexistent_returns_none() {
        let store = SpecStore::open(":memory:").unwrap();
        assert!(store.get(999).unwrap().is_none());
    }

    #[test]
    fn delete_removes_record() {
        let store = SpecStore::open(":memory:").unwrap();
        let id = store.store("x", &empty_spec()).unwrap();
        assert!(store.delete(id).unwrap());
        assert!(store.get(id).unwrap().is_none());
    }

    #[test]
    fn count_reflects_rows() {
        let store = SpecStore::open(":memory:").unwrap();
        assert_eq!(store.count().unwrap(), 0);
        store.store("a", &empty_spec()).unwrap();
        assert_eq!(store.count().unwrap(), 1);
    }
}
