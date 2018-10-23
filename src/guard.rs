use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use models::Setting;
use pg_pool::DbConn;
use r2d2::Pool;
use rocket::Outcome;
use rocket::Request;
use rocket::request;
use rocket::request::FromRequest;
use rocket::State;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct SettingMap {
    pub title: String,
    pub description: String,
}

impl <'a, 'r> FromRequest<'a, 'r> for SettingMap {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, ()> {

        use schema::{setting, setting::dsl::*};
        use diesel::prelude::*;

        let state = request.guard::<State<Pool<ConnectionManager<PgConnection>>>>().unwrap().get().unwrap();
        let conn = DbConn(state);

        let settings = setting::table.load::<Setting>(&*conn).unwrap();

        let mut settings_map: HashMap<String, String> = HashMap::new();

        for one_setting in settings {
            settings_map.insert(one_setting.name, one_setting.value.unwrap_or("".to_string()));
        }

        Outcome::Success(SettingMap{
            title: settings_map.get("title").unwrap_or(&"".to_string()).clone(),
            description: settings_map.get("description").unwrap_or(&"".to_string()).clone(),
        })

    }
}
