use chrono::NaiveDateTime;
use rocket::serde::Serialize;
use rocket_okapi::JsonSchema;

#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct Audit {
    pub timestamp: NaiveDateTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    pub name: String,
    pub response: Option<String>,
}
