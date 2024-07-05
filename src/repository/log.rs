use super::MockData;
use crate::model::{Log, Result};
use rocket_db_pools::Connection;

pub async fn get(mut db: Connection<MockData>) -> Result<Vec<Log>> {
    let logs = sqlx::query_as!(
        Log,
        r#"SELECT timestamp, name, method, timeout as "timeout: u32", request, response FROM logs ORDER BY timestamp DESC"#,
    )
        .fetch_all(&mut **db)
        .await?;

    Ok(logs)
}

pub async fn mock_get(mut db: Connection<MockData>, name: &str) -> Result<Vec<Log>> {
    let logs = sqlx::query!(
        r#"SELECT timestamp, method, timeout as "timeout: u32", request, response FROM logs WHERE name = ? ORDER BY timestamp DESC"#,
        name
    )
        .fetch_all(&mut **db)
        .await?
        .into_iter()
        .map(|row| Log {
            timestamp: row.timestamp,
            name: name.to_owned(),
            method: row.method,
            timeout: row.timeout,
            request: row.request,
            response: row.response,
        })
        .collect();

    Ok(logs)
}
