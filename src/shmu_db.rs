use chrono::{DateTime, Utc};
use log::info;
use sqlite::{Connection, State};

pub struct AlertDatabase {
    connection: Connection,
}

impl AlertDatabase {
    pub fn new(path: &str) -> sqlite::Result<Self> {
        let connection = sqlite::open(path)?;

        let db = Self {
            connection,
        };

        db.create_database()?;

        Ok(db)
    }

    pub fn empty() -> sqlite::Result<Self> {
        Self::new(":memory:")
    }

    fn create_database(&self) -> sqlite::Result<()> {
        self.connection.execute(
            "
                CREATE TABLE IF NOT EXISTS alerts (
                    id TEXT PRIMARY KEY,
                    event_end INTEGER NOT NULL
                );
            ",
        )?;

        Ok(())
    }

    pub fn insert_alert(
        &self,
        id: &str,
        event_end: DateTime<Utc>,
    ) -> sqlite::Result<()> {
        let query = "
            INSERT OR REPLACE INTO alerts (id, event_end) VALUES (?, ?);
        ";

        info!("Executing query: {}", query);

        let mut statement = self.connection.prepare(query)?;

        statement.bind((1, id))?;
        statement.bind((2, event_end.timestamp()))?;
        statement.next()?;

        Ok(())
    }

    pub fn has_alert(&self, id: &str) -> sqlite::Result<bool> {
        let query = "
            SELECT 1 FROM alerts WHERE id = ? LIMIT 1;
        ";

        info!("Executing query: {}", query);

        let mut statement = self.connection.prepare(query)?;

        statement.bind((1, id))?;

        Ok(matches!(statement.next()?, State::Row))
    }

    pub fn clear_expired_alerts(&self) -> sqlite::Result<()> {
        let query = "
            DELETE FROM alerts WHERE event_end < ?;
        ";

        info!("Executing query: {}", query);

        let mut statement = self.connection.prepare(query)?;

        statement.bind((1, Utc::now().timestamp()))?;
        statement.next()?;

        Ok(())
    }
}
