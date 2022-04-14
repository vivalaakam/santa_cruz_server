use crate::proto::proto::santa_cruz::Exercise;
use crate::Queryable;
use chrono::{DateTime, Utc};
use sqlx::postgres::PgRow;
use sqlx::Row;
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
}
