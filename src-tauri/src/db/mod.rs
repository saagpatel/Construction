pub mod incidents;
pub mod jsa;
pub mod locations;
pub mod osha;
pub mod rca;
pub mod toolbox;

use anyhow::{Context, Result};
use rusqlite::Connection;
use std::path::Path;

const MIGRATION_FILES: &[(&str, &str)] = &[
    ("001_initial", include_str!("migrations/001_initial.sql")),
    (
        "002_toolbox_talks",
        include_str!("migrations/002_toolbox_talks.sql"),
    ),
    (
        "003_toolbox_seed",
        include_str!("migrations/003_toolbox_seed.sql"),
    ),
    ("004_jsa", include_str!("migrations/004_jsa.sql")),
    ("005_jsa_seed", include_str!("migrations/005_jsa_seed.sql")),
    (
        "006_inspections",
        include_str!("migrations/006_inspections.sql"),
    ),
    (
        "007_inspections_seed",
        include_str!("migrations/007_inspections_seed.sql"),
    ),
    (
        "008_near_miss",
        include_str!("migrations/008_near_miss.sql"),
    ),
    ("009_training", include_str!("migrations/009_training.sql")),
    (
        "010_training_seed",
        include_str!("migrations/010_training_seed.sql"),
    ),
    (
        "011_equipment",
        include_str!("migrations/011_equipment.sql"),
    ),
    (
        "012_sync_auth",
        include_str!("migrations/012_sync_auth.sql"),
    ),
    (
        "013_trade_hazards",
        include_str!("migrations/013_trade_hazards.sql"),
    ),
    (
        "014_trade_hazards_seed",
        include_str!("migrations/014_trade_hazards_seed.sql"),
    ),
];

pub fn open_db(db_path: &Path) -> Result<Connection> {
    let mut conn = Connection::open(db_path)
        .with_context(|| format!("Failed to open database at {}", db_path.display()))?;

    conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")
        .context("Failed to set PRAGMA")?;

    run_migrations(&mut conn)?;
    Ok(conn)
}

fn run_migrations(conn: &mut Connection) -> Result<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS _migrations (
            name TEXT PRIMARY KEY,
            applied_at TEXT NOT NULL DEFAULT (datetime('now'))
        );",
    )
    .context("Failed to create migrations table")?;

    for (name, sql) in MIGRATION_FILES {
        let already_applied: bool = conn
            .query_row(
                "SELECT EXISTS(SELECT 1 FROM _migrations WHERE name = ?1)",
                [name],
                |row| row.get(0),
            )
            .context("Failed to check migration status")?;

        if !already_applied {
            let tx = conn
                .transaction()
                .with_context(|| format!("Failed to open transaction for migration: {name}"))?;

            tx.execute_batch(sql)
                .with_context(|| format!("Failed to run migration: {name}"))?;

            tx.execute("INSERT INTO _migrations (name) VALUES (?1)", [name])
                .with_context(|| format!("Failed to record migration: {name}"))?;

            tx.commit()
                .with_context(|| format!("Failed to commit migration transaction: {name}"))?;
        }
    }

    Ok(())
}

#[cfg(test)]
pub fn open_test_db() -> Connection {
    let conn = Connection::open_in_memory().expect("open in-memory db");
    conn.execute_batch("PRAGMA foreign_keys=ON;").unwrap();
    for (_name, sql) in MIGRATION_FILES {
        conn.execute_batch(sql).unwrap();
    }
    conn
}
