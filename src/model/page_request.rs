use rocket::FromForm;
use rocket_okapi::JsonSchema;

#[derive(FromForm, JsonSchema)]
pub struct PageRequest {
    #[field(default = Some(10))]
    size: Option<u8>,
    page: Option<u16>,
}

impl PageRequest {
    pub fn limit(&self) -> u8 {
        self.size.unwrap_or_default()
    }

    pub fn offset(&self) -> u32 {
        (self.page.unwrap_or_default() as u32) * (self.size.unwrap_or_default() as u32)
    }
}
