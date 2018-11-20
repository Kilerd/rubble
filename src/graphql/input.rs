
#[derive(GraphQLInputObject)]
#[graphql(description="A humanoid creature in the Star Wars universe")]
pub struct NewHuman {

    #[graphql(description="new name for new human")]
    name: String,
    home_planet: Option<String>,
}