use rocket::response::Debug;
use sqlx::Error;

pub type Result<T> = std::result::Result<T, Debug<Error>>;
