use rocket::get;
use rocket::response::content::RawHtml;
use rocket_okapi::openapi;

#[openapi(skip)]
#[get("/")]
pub fn index() -> RawHtml<String> {
    let html = format!(
        r#"<h1>Hello, world!</h1><ul><li><a href="{}">swagger</a></li><li><a href="{}">rapidoc</a></li></ul>"#,
        rocket::uri!("/swagger-ui/"),
        rocket::uri!("/rapidoc/")
    );

    RawHtml(html)
}
