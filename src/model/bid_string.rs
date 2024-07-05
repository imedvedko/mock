use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::JsonSchema;

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(crate = "rocket::serde", transparent)]
pub struct BigString {
    pub value: String
}
