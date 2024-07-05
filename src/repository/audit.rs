use super::MockData;
use crate::model::{Audit, Result};
use rocket_db_pools::Connection;

pub async fn get(mut db: Connection<MockData>) -> Result<Vec<Audit>> {
    let audit = sqlx::query_as!(
        Audit,
        "SELECT timestamp, token, name, response FROM audit ORDER BY timestamp DESC"
    )
    .fetch_all(&mut **db)
    .await?;

    Ok(audit)
}

pub async fn token_get(mut db: Connection<MockData>, token: &str) -> Result<Vec<Audit>> {
    let audit = sqlx::query!(
        "SELECT timestamp, name, response FROM audit WHERE token = ? ORDER BY timestamp DESC",
        token
    )
    .fetch_all(&mut **db)
    .await?
    .into_iter()
    .map(|row| Audit {
        timestamp: row.timestamp,
        token: Some(token.to_owned()),
        name: row.name,
        response: row.response,
    })
    .collect();

    Ok(audit)
}

pub async fn mock_get(mut db: Connection<MockData>, name: &str) -> Result<Vec<Audit>> {
    let audit = sqlx::query!(
        "SELECT timestamp, token, response FROM audit WHERE name = ? ORDER BY timestamp DESC",
        name
    )
    .fetch_all(&mut **db)
    .await?
    .into_iter()
    .map(|row| Audit {
        timestamp: row.timestamp,
        token: row.token,
        name: name.to_owned(),
        response: row.response,
    })
    .collect();

    Ok(audit)
}
