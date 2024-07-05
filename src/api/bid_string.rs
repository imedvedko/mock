use crate::model::BigString;
use rocket::data::{ByteUnit, FromData};
use rocket::http::Status;
use rocket::{data, Data, Request};
use std::io;

#[rocket::async_trait]
impl<'r> FromData<'r> for BigString {
    type Error = io::Error;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> data::Outcome<'r, Self> {
        let limit = req
            .limits()
            .get("big-string")
            .unwrap_or(ByteUnit::Megabyte(1));
        match data.open(limit).into_string().await {
            Ok(capped) if capped.is_complete() => {
                let string = capped.into_inner();
                let result = BigString { value: string };

                data::Outcome::Success(result)
            }
            Ok(_) => {
                let error = io::Error::new(io::ErrorKind::UnexpectedEof, "data limit exceeded");
                data::Outcome::Error((Status::PayloadTooLarge, error))
            }
            Err(e) => data::Outcome::Error((Status::BadRequest, e)),
        }
    }
}
