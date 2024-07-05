use crate::model::Config;
use crate::model::User;
use rocket::http::Status;
use rocket::outcome::IntoOutcome;
use rocket::request::FromRequest;
use rocket::{request, Request};

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
