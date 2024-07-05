use crate::model::User;
use rocket::http::Status;
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::okapi;
use rocket_okapi::okapi::openapi3::{Object, RefOr, Responses, SecurityScheme, SecuritySchemeData};
use rocket_okapi::request::{OpenApiFromRequest, RequestHeaderInput};

impl<'a> OpenApiFromRequest<'a> for User {
    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<RequestHeaderInput> {
        Ok(RequestHeaderInput::Security(
            "bearerAuth".to_owned(),
            SecurityScheme {
                description: None,
                data: SecuritySchemeData::Http {
                    scheme: "bearer".to_owned(),
                    bearer_format: None,
                },
                extensions: Object::default(),
            },
            okapi::map! {
                "bearerAuth".to_owned() =>  Vec::new()
            },
        ))
    }

    fn get_responses(_gen: &mut OpenApiGenerator) -> rocket_okapi::Result<Responses> {
        // let schema = gen.json_schema_no_ref::<Mock>();
        Ok(Responses {
            responses: okapi::map! {
                Status::Forbidden.code.to_string() => RefOr::Object(okapi::openapi3::Response {
                    ..Default::default()
                }),
                Status::Unauthorized.code.to_string() => RefOr::Object(okapi::openapi3::Response {
                    ..Default::default()
                }),
            },
            ..Default::default()
        })
    }
}
