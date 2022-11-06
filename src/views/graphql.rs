use std::pin::Pin;

use futures::Stream;
use juniper::{FieldError, FieldResult, FromContext, RootNode};
use rocket::{Route, State};

use crate::{config::Config, models::users::User};

/// The GraphQL context
pub struct Context {
    pub user: User,
    pub config: Config,
}

/// Common wrapper for contexts
impl juniper::Context for Context {}

/// Root Query
pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    fn api_version() -> &'static str {
        #[allow(unused_braces)]
        "1.0"
    }

    /// Get user
    async fn user(context: &Context) -> FieldResult<&User> {
        todo!()
    }
}

/// Root mutation
pub struct Mutation;

#[graphql_object(context = Context)]
impl Mutation {
    fn user() -> &'static str {
        "todo"
    }
}

/// Root subscription
pub struct Subscription;

pub type GQLStream<S> = Pin<Box<dyn Stream<Item = Result<S, FieldError>> + Send>>;

#[graphql_subscription(context = Context)]
impl Subscription {
    async fn updates(&self, _context: &Context) -> GQLStream<String> {
        let stream =
            futures::stream::iter(vec![Ok(String::from("Hello")), Ok(String::from("World!"))]);
        Box::pin(stream)
    }
}

/// The entire GraphQL schema
pub type Schema = RootNode<'static, Query, Mutation, Subscription>;

#[options("/graphql")]
pub async fn graphql_options() {}

#[get("/graphql?<request>", format = "json")]
pub async fn get_graphql_handler(
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
    user: &User,
    config: &State<Config>,
) -> juniper_rocket::GraphQLResponse {
    let context = Context {
        user: user.clone(),
        config: (**config).clone(),
    };
    request.execute(&*schema, &context).await
}

#[post("/graphql", data = "<request>", format = "json")]
pub async fn post_graphql_handler(
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
    user: &User,
    config: &State<Config>,
) -> juniper_rocket::GraphQLResponse {
    let context = Context {
        user: user.clone(),
        config: (**config).clone(),
    };
    request.execute(&*schema, &context).await
}

/// All routes associated with GraphQL
pub fn routes() -> Vec<Route> {
    routes![get_graphql_handler, post_graphql_handler, graphql_options]
}
impl Query {
    pub(crate) fn init() -> Query {
        Query
    }
}
impl Mutation {
    pub(crate) fn init() -> Mutation {
        Mutation
    }
}
impl Subscription {
    pub(crate) fn init() -> Subscription {
        Subscription
    }
}
