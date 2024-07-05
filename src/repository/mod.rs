pub mod audit;
pub mod log;

use rocket::log::private::error;
use rocket::{fairing, Build, Rocket};
use rocket_db_pools::Database;

#[derive(Database)]
#[database("sqlite_mock")]
pub struct MockData(sqlx::SqlitePool);

pub async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    match MockData::fetch(&rocket) {
        Some(db) => match sqlx::migrate!().run(&**db).await {
            Ok(_) => Ok(rocket),
            Err(e) => {
                error!("Failed to initialize SQLx database: {}", e);
                Err(rocket)
            }
        },
        None => Err(rocket),
    }
}