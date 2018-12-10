use crate::graphql::Mutation;
use crate::pg_pool::DbConn;

use crate::graphql::input::*;
use crate::models::Setting;

graphql_object!(Mutation: DbConn |&self| {

    field modifySetting(&executor, input: SettingInput) -> Option<Setting> {
        let conn = executor.context();
        Setting::modify(&input, &conn)
    }
});