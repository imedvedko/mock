use crate::model::Config;
use rocket::http::Status;
use rocket::outcome::IntoOutcome;
use rocket::request::FromRequest;
use rocket::{request, Request};
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::okapi;
use rocket_okapi::okapi::openapi3::{Object, RefOr, Responses, SecurityScheme, SecuritySchemeData};
use rocket_okapi::request::{OpenApiFromRequest, RequestHeaderInput};

pub struct User {
    pub token: String,
}

#[rocket::async_trait]
impl<'a> FromRequest<'a> for User {
    type Error = String;

    async fn from_request(request: &'a Request<'_>) -> request::Outcome<Self, Self::Error> {
        let auth_type = "Bearer";
        let auth_type_token = &*(auth_type.to_owned() + " ");
        match request.headers().get_one("Authorization") {
            Some(authorization) if authorization.starts_with(auth_type_token) => {
                let token = &authorization[auth_type_token.len()..];
                let config = rocket::outcome::try_outcome!(request
                    .rocket()
                    .state::<Config>()
                    .or_error((Status::InternalServerError, "Can not get config".to_owned())));
                if token == config.token {
                    request::Outcome::Success(User {
                        token: token.to_owned(),
                    })
                } else {
                    request::Outcome::Error((
                        Status::Forbidden,
                        format!("Token is invalid: '{}'", token),
                    ))
                }
            }
            Some(authorization) => request::Outcome::Error((
                Status::Forbidden,
                format!("Unknown auth type: '{}'", authorization),
            )),
            None => request::Outcome::Error((
                Status::Unauthorized,
                "Missing 'Authorization' header".to_owned(),
            )),
        }
    }
}

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
