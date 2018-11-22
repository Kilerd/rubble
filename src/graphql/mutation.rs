use crate::graphql::Mutation;
use crate::pg_pool::DbConn;

use crate::graphql::input::*;
use crate::models::Setting;

graphql_object!(Mutation: DbConn |&self| {
    field create(&executor, new: NewHuman) -> i32 {
        2
    }

    field modifySetting(&executor, setting: ModifiedSetting) -> Option<Setting> {
        let conn = executor.context();
        Setting::modify(&setting, &conn)
    }
});