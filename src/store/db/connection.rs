use std::path::Path;

use rusqlite::{Connection, Transaction};

use crate::error::{AcriResult, AcrithisError, ResultExt};

const PRAGMAS : &str = "
    PRAGMA journal_mode = WAL;
    PRAGMA foreign_keys = ON;
    PRAGMA synchronous = NORMAL;
    PRAGMA busy_timeout = 5000;
";

const MIGRATION_0001 : &str = include_str!("../sql/migrations/0001_initial_schema.sql");

pub struct DbConnection {
    conn : Connection,
}

impl DbConnection {
    pub fn open<P : AsRef<Path>>(path : P) -> AcriResult<Self> {
        let mut conn : Connection =
            Connection::open(path).map_database("Failed to open SQLite database file")?;

        conn.execute_batch(PRAGMAS)
            .map_database("Failed to apply SQLite Pragmas")?;

        Self::apply_migrations(&mut conn)?;

        Ok(Self { conn })
    }

    fn apply_migrations(conn : &mut Connection) -> AcriResult<()> {
        let tx : Transaction<'_> = conn.transaction()
                                       .map_database("Failed to begin migration transaction")?;

        let user_version : i32 = tx.query_row("PRAGMA user_version", [], |row| row.get(0))
                                   .map_database("Failed to read user_version PRAGMA")?;

        if user_version == 0 {
            tx.execute_batch(MIGRATION_0001)
              .map_database("Failed to apply 0001_initial_schema migration")?;

            tx.execute_batch("PRAGMA user_version = 1")
              .map_database("Failed to update user_version PRAGMA")?;
        }
        else if user_version > 1 {
            return Err(AcrithisError::database(format!(
                "Database schema version {} is newer than supported version 1",
                user_version
            ),));
        }

        tx.commit()
          .map_database("Failed to commit migration transaction")?;
        Ok(())
    }

    pub fn transaction<F, R>(&mut self, f : F) -> AcriResult<R>
        where F : FnOnce(&rusqlite::Transaction) -> AcriResult<R>,
    {
        let tx : Transaction<'_> =
            self.conn
                .transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)
                .map_database("Failed to begin IMMEDIATE transaction")?;

        let result : R = f(&tx)?;
        tx.commit().map_database("Failed to commit transaction")?;

        Ok(result)
    }

    pub fn read_transaction<F, R>(&mut self, f : F) -> AcriResult<R>
        where F : FnOnce(&rusqlite::Transaction) -> AcriResult<R>,
    {
        let tx : Transaction<'_> = self.conn
                                       .transaction()
                                       .map_database("Failed to begin read transaction")?;
        f(&tx)
    }
}
