//! Database utilities using sqlx and sqlite.

#[cfg(feature = "db")]
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
#[cfg(feature = "db")]
use std::path::Path;

#[cfg(feature = "db")]
pub struct Database {
    pub pool: SqlitePool,
}

#[cfg(feature = "db")]
impl Database {
    /// Initialize database connection.
    /// Creates the database file if it doesn't exist.
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let db_path = database_url.strip_prefix("sqlite:").unwrap_or(database_url);
        
        // Ensure directory exists if path contains separators
        if let Some(parent) = Path::new(db_path).parent() {
            if !parent.exists() && !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent).ok();
            }
        }

        if !Path::new(db_path).exists() {
            println!("Creating new database: {}", db_path);
            std::fs::File::create(db_path).ok();
        }

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;

        println!("Database connected: {}", database_url);

        Ok(Self { pool })
    }

    /// Helper to execute a batch of raw SQL queries (e.g. initial schema setup)
    pub async fn run_raw_migrations(&self, queries: &[&str]) -> Result<(), sqlx::Error> {
        for query in queries {
            sqlx::query(query).execute(&self.pool).await?;
        }
        Ok(())
    }
}
