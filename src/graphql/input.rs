use diesel::prelude::*;
use crate::schema::{setting};

#[derive(GraphQLInputObject)]
#[graphql(description="A humanoid creature in the Star Wars universe")]
pub struct NewHuman {

    #[graphql(description="new name for new human")]
    name: String,
    home_planet: Option<String>,
}

#[derive(GraphQLInputObject)]
#[graphql(description="modify a setting")]
#[derive(Serialize, Insertable, AsChangeset)]
#[table_name = "setting"]
pub struct ModifiedSetting {

    #[graphql(description="the name of the setting")]
    pub name: String,

    #[graphql(description="new value of the setting")]
    pub value: String,
}