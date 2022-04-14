use crate::query_builder::QueryBuilder;

pub trait Queryable {
    fn fields() -> Vec<&'static str>;

    fn table() -> &'static str;

    fn query() -> QueryBuilder;
}
