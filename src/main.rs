
#[macro_use]
extern crate graphql_client;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "query.graphql",
    response_derives = "Debug,Clone,Serialize"
)]
pub struct ExampleQuery;

fn main() {
    println!("Hello, world!");
}
