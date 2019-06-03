use crate::models::CRUD;
use crate::schema::setting;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Queryable, Debug, Serialize, Deserialize, Insertable, AsChangeset)]
#[table_name = "setting"]
pub struct Setting {
    pub name: String,
    pub value: Option<String>,
}

#[derive(Serialize)]
pub struct SettingMap {
    pub title: String,
    pub description: String,
    pub url: String,
    pub analysis: String,
}
#[derive(Queryable, Debug, Serialize, Deserialize, AsChangeset)]
#[table_name = "setting"]
pub struct UpdateSetting {
    pub value: Option<String>,
}

impl Setting {
    // TODO refactor this method
    pub fn load(conn: &PgConnection) -> SettingMap {
        let settings = setting::table.load::<Setting>(conn).unwrap();

        let mut settings_map: HashMap<String, String> = HashMap::new();

        for one_setting in settings {
            settings_map.insert(
                one_setting.name,
                one_setting.value.unwrap_or("".to_string()),
            );
        }

        SettingMap {
            title: settings_map.get("title").unwrap_or(&"".to_string()).clone(),
            description: settings_map
                .get("description")
                .unwrap_or(&"".to_string())
                .clone(),

            url: settings_map.get("url").unwrap_or(&"".to_string()).clone(),
            analysis: settings_map
                .get("analysis")
                .unwrap_or(&"".to_string())
                .clone(),
        }
    }
}

impl CRUD<(), UpdateSetting, String> for Setting {
    fn create(conn: &PgConnection, from: &()) -> Result<Self, Error> {
        unimplemented!()
    }

    fn read(conn: &PgConnection) -> Vec<Self> {
        unimplemented!()
    }

    fn update(conn: &PgConnection, pk: String, value: &UpdateSetting) -> Result<Self, Error> {
        diesel::update(setting::table.find(&pk))
            .set(value)
            .get_result(conn)
    }

    fn delete(conn: &PgConnection, pk: String) -> Result<usize, Error> {
        unimplemented!()
    }

    fn get_by_pk(conn: &PgConnection, pk: String) -> Result<Self, Error> {
        unimplemented!()
    }
}
