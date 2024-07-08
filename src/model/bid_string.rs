use rocket::data::{ByteUnit, FromData};
use rocket::http::{ContentType, Status};
use rocket::serde::{Deserialize, Serialize};
use rocket::{data, Data, Request};
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::okapi::openapi3::{MediaType, RequestBody};
use rocket_okapi::request::OpenApiFromData;
use rocket_okapi::{okapi, JsonSchema};
use std::io;

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(crate = "rocket::serde", transparent)]
pub struct BigString {
    pub value: String,
}

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

impl<'r> OpenApiFromData<'r> for BigString {
    fn request_body(gen: &mut OpenApiGenerator) -> rocket_okapi::Result<RequestBody> {
        let request_body = String::request_body(gen)?;
        let schema = gen.json_schema::<String>();
        let mut content = okapi::map! {
            ContentType::Text.to_string() => MediaType {
                schema: Some(schema),
                ..Default::default()
            }
        };
        request_body.content.into_iter().for_each(|(k, v)| {
            content.insert(k, v);
        });
        Ok(RequestBody {
            content,
            ..request_body
        })
    }
}
