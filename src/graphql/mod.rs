mod types;
mod schema;

use async_graphql::{EmptySubscription, Schema};
use sea_orm::DatabaseConnection;

pub type AppSchema = Schema<schema::Query, schema::Mutation, EmptySubscription>;

pub fn build_schema(
    db: DatabaseConnection,
) -> AppSchema {
    Schema::build(schema::Query, schema::Mutation, EmptySubscription)
        .data(db)
        .finish()
}
