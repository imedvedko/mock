use crate::model::{Audit, Result};

repository!(AuditRepository);

impl AuditRepository {
    pub async fn get(&mut self) -> Result<Vec<Audit>> {
        let audit = sqlx::query_as!(
            Audit,
            "SELECT timestamp, token, name, response FROM audit ORDER BY timestamp DESC"
        )
        .fetch_all(&mut **self.db)
        .await?;

        Ok(audit)
    }

    pub async fn token_get(&mut self, token: String) -> Result<Vec<Audit>> {
        let audit = sqlx::query!(
            "SELECT timestamp, name, response FROM audit WHERE token = ? ORDER BY timestamp DESC",
            token
        )
        .fetch_all(&mut **self.db)
        .await?
        .into_iter()
        .map(|row| Audit {
            timestamp: row.timestamp,
            token: Some(token.clone()),
            name: row.name,
            response: row.response,
        })
        .collect();

        Ok(audit)
    }

    pub async fn mock_get(&mut self, name: String) -> Result<Vec<Audit>> {
        let audit = sqlx::query!(
            "SELECT timestamp, token, response FROM audit WHERE name = ? ORDER BY timestamp DESC",
            name
        )
        .fetch_all(&mut **self.db)
        .await?
        .into_iter()
        .map(|row| Audit {
            timestamp: row.timestamp,
            token: row.token,
            name: name.clone(),
            response: row.response,
        })
        .collect();

        Ok(audit)
    }
}
