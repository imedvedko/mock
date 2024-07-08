use crate::model::{Log, Result};

repository!(LogRepository);

impl LogRepository {
    pub async fn get(&mut self) -> Result<Vec<Log>> {
        let logs = sqlx::query_as!(
            Log,
            r#"SELECT timestamp, name, method, timeout as "timeout: u32", request, response FROM logs ORDER BY timestamp DESC"#,
        )
            .fetch_all(&mut **self.db)
            .await?;

        Ok(logs)
    }

    pub async fn mock_get(&mut self, name: String) -> Result<Vec<Log>> {
        let logs = sqlx::query!(
            r#"SELECT timestamp, method, timeout as "timeout: u32", request, response FROM logs WHERE name = ? ORDER BY timestamp DESC"#,
            name
        )
            .fetch_all(&mut **self.db)
            .await?
            .into_iter()
            .map(|row| Log {
                timestamp: row.timestamp,
                name: name.clone(),
                method: row.method,
                timeout: row.timeout,
                request: row.request,
                response: row.response,
            })
            .collect();

        Ok(logs)
    }
}
