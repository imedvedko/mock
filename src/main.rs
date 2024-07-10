use model::{Config, Response};
use rocket::config::Ident;
use rocket::fairing::AdHoc;
use rocket::figment::providers::{Env, Format, Toml};
use rocket::figment::{Figment, Profile};
use rocket::http::Status;
use rocket::shield::Shield;
use rocket::{catch, catchers, launch, Request};
use rocket_db_pools::Database;
use rocket_okapi::openapi_get_routes;
use rocket_okapi::rapidoc::{make_rapidoc, GeneralConfig, HideShowConfig, RapiDocConfig};
use rocket_okapi::settings::UrlObject;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};

mod model;
mod repository;
mod routing;

#[launch]
fn rocket() -> _ {
    let default_config = rocket::Config {
        ident: Ident::none(),
        ip_header: None,
        // log_level: rocket::log::LogLevel::Debug,
        ..rocket::Config::default()
    };

    let figment = Figment::from(default_config)
        .merge(Toml::file(Env::var_or("MOCK_CONFIG", "Mock.toml")).nested())
        .merge(Env::prefixed("MOCK_").ignore(&["PROFILE"]).global())
        .select(Profile::from_env_or(
            "MOCK_PROFILE",
            rocket::Config::DEFAULT_PROFILE,
        ))
        .merge(("databases.sqlite_mock.url", "db.sqlite"));

    rocket::custom(figment)
        .attach(repository::MockData::init())
        .attach(AdHoc::try_on_ignite(
            "Migrations",
            repository::run_migrations,
        ))
        .attach(Shield::new())
        .attach(AdHoc::config::<Config>())
        .mount(
            "/",
            openapi_get_routes![
                routing::index::index,
                routing::mock::list,
                routing::mock::create,
                routing::mock::update,
                routing::mock::get,
                routing::mock::delete,
                routing::data::create,
                routing::data::update,
                routing::call::get,
                routing::call::post,
                routing::log::get,
                routing::log::mock_get,
                routing::audit::get,
                routing::audit::token_get,
                routing::audit::mock_get
            ],
        )
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                display_request_duration: true,
                ..Default::default()
            }),
        )
        .mount(
            "/rapidoc/",
            make_rapidoc(&RapiDocConfig {
                general: GeneralConfig {
                    spec_urls: vec![UrlObject::new("General", "../openapi.json")],
                    ..Default::default()
                },
                hide_show: HideShowConfig {
                    allow_spec_url_load: false,
                    allow_spec_file_load: false,
                    ..Default::default()
                },
                ..Default::default()
            }),
        )
        .register("/", catchers![catcher])
}

#[catch(default)]
fn catcher(status: Status, _request: &Request) -> Response {
    Response::http_error(status)
}
