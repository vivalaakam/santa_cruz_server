use crate::query_builder::QueryBuilder;
use crate::Queryable;
use chrono::{DateTime, Utc};
use sqlx::postgres::PgRow;
use sqlx::Row;
pub mod exercise {
    use super::*;
    use crate::proto::proto::santa_cruz::Exercise;
    impl From<PgRow> for Exercise {
        fn from(row: PgRow) -> Self {
            Exercise {
                id: row.get::<i32, _>("id"),
                created_at: row.get::<DateTime<Utc>, _>("created_at").to_rfc3339(),
                updated_at: row.get::<DateTime<Utc>, _>("updated_at").to_rfc3339(),
                name: row.get::<String, _>("name"),
                description: row.get::<String, _>("description"),
            }
        }
    }
    impl Queryable for Exercise {
        fn fields() -> Vec<&'static str> {
            vec!["id", "created_at", "updated_at", "name", "description"]
        }
        fn table() -> &'static str {
            "exercises"
        }
        fn query() -> QueryBuilder {
            let mut query = QueryBuilder::new(Exercise::table());
            query.fields(Exercise::fields());
            query
        }
    }
}
