use crate::schema::{setting};

#[derive(GraphQLInputObject)]
#[graphql(description="modify a setting")]
#[derive(Serialize, Insertable, AsChangeset)]
#[table_name = "setting"]
pub struct SettingInput {

    #[graphql(description="the name of the setting")]
    pub name: String,

    #[graphql(description="new value of the setting")]
    pub value: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description="delete article")]
pub struct DeleteArticleInput {

    #[graphql(description="id of article wanna delete")]
    pub id: i32,
}