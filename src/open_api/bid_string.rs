use crate::model::BigString;
use rocket::http::ContentType;
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::okapi;
use rocket_okapi::okapi::openapi3::{MediaType, RequestBody};
use rocket_okapi::request::OpenApiFromData;

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
