extern crate juniper;

#[derive(GraphQLObject)]
#[graphql(description = "Information about a person")]
struct Person {
    #[graphql(description = "The person's full name, including both first and last names")]
    name: String,
    #[graphql(description = "The person's age in years, rounded down")]
    age: i32,
    #[graphql(skip)]
    _password_hash: String, // This cannot be queried or modified from GraphQL
}

#[derive(GraphQLObject)]
#[graphql(description = "Information about a house")]
struct House {
    #[graphql(description = "House address, optional")]
    address: Option<String>,
    #[graphql(description = "Vector of Person struct, persons who live in the house")]
    inhabitants: Vec<Person>,
}
