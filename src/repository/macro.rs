macro_rules! repository {
    ($name:ident) => {
        #[derive(rocket_okapi::request::OpenApiFromRequest)]
        pub struct $name {
            db: rocket_db_pools::Connection<crate::repository::MockData>
        }

        #[rocket::async_trait]
        impl<'a> rocket::request::FromRequest<'a> for $name {
            type Error = <rocket_db_pools::Connection<crate::repository::MockData> as rocket::request::FromRequest<'a>>::Error;

            async fn from_request(request: &'a rocket::Request<'_>) -> rocket::request::Outcome<Self, Self::Error> {
                let db =
                    rocket::outcome::try_outcome!(request.guard::<rocket_db_pools::Connection<crate::repository::MockData>>().await);

                rocket::request::Outcome::Success($name { db })
            }
        }
    }
}
