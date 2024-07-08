use crate::model::{Mock, Result, User};
use rocket::tokio::time::{sleep, Duration};
use sqlx::Connection;

repository!(MockRepository);

impl MockRepository {
    pub async fn list(&mut self) -> Result<Vec<String>> {
        let mocks = sqlx::query!("SELECT name FROM mocks")
            .fetch_all(&mut **self.db)
            .await?
            .into_iter()
            .map(|row| row.name)
            .collect();

        Ok(mocks)
    }

    pub async fn create(&mut self, user: User, mock: Mock) -> Result<Mock> {
        self.db
            .transaction(|transaction| {
                Box::pin(async move {
                    sqlx::query!(
                        "INSERT INTO mocks (name, response) VALUES (?, ?)",
                        mock.name,
                        mock.response
                    )
                    .execute(&mut **transaction)
                    .await?;

                    sqlx::query!(
                        "INSERT INTO audit (token, name, response) VALUES (?, ?, ?)",
                        user.token,
                        mock.name,
                        mock.response
                    )
                    .execute(&mut **transaction)
                    .await?;

                    Ok(mock) as Result<Mock>
                })
            })
            .await
    }

    pub async fn update(&mut self, user: User, name: String, mock: Mock) -> Result<Option<Mock>> {
        self.db
            .transaction(|transaction| {
                Box::pin(async move {
                    let result = sqlx::query!(
                        "UPDATE mocks SET name = ?, response = ? WHERE name = ?",
                        mock.name,
                        mock.response,
                        name
                    )
                    .execute(&mut **transaction)
                    .await?;

                    let response = if result.rows_affected() > 0 {
                        sqlx::query!(
                            "INSERT INTO audit (token, name, response) VALUES (?, ?, ?)",
                            user.token,
                            mock.name,
                            mock.response
                        )
                        .execute(&mut **transaction)
                        .await?;

                        Some(mock)
                    } else {
                        None
                    };

                    Ok(response)
                })
            })
            .await
    }

    pub async fn get(&mut self, name: String) -> Result<Option<Mock>> {
        let response = sqlx::query!("SELECT response FROM mocks WHERE name = ?", name)
            .fetch_optional(&mut **self.db)
            .await?
            .map(|row| Mock {
                name,
                response: row.response,
            });

        Ok(response)
    }

    pub async fn delete(&mut self, user: User, name: String) -> Result<Option<()>> {
        self.db
            .transaction(|transaction| {
                Box::pin(async move {
                    let result = sqlx::query!("DELETE FROM mocks WHERE name = ?", name)
                        .execute(&mut **transaction)
                        .await?;

                    let response = if result.rows_affected() > 0 {
                        sqlx::query!(
                            "INSERT INTO audit (token, name, response) VALUES (?, ?, ?)",
                            user.token,
                            name,
                            None::<String>
                        )
                        .execute(&mut **transaction)
                        .await?;

                        Some(())
                    } else {
                        None
                    };

                    Ok(response)
                })
            })
            .await
    }

    pub async fn call(
        &mut self,
        name: String,
        method: String,
        request: Option<String>,
        timeout: Option<u32>,
    ) -> Result<Option<String>> {
        let response = sqlx::query!("SELECT response FROM mocks WHERE name = ?", name)
            .fetch_optional(&mut **self.db)
            .await?
            .map(|row| row.response);

        if response.is_some() {
            sqlx::query!(
                "INSERT INTO logs (name, method, request, response, timeout) VALUES (?, ?, ?, ?, ?)",
                name,
                method,
                request,
                response,
                timeout
            )
                .execute(&mut **self.db)
                .await?;

            if let Some(timeout) = timeout {
                sleep(Duration::from_millis(timeout.into())).await;
            }
        }

        Ok(response)
    }
}
