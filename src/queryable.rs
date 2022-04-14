pub trait Queryable {
    fn fields() -> Vec<&'static str>;
}
