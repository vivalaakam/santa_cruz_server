use sqlx::postgres::PgArguments;
use sqlx::{Arguments, Encode, Postgres, Type};

#[derive(Clone)]
pub struct Field {
    value: String,
    position: usize,
}

#[derive(Default)]
pub struct QueryBuilder {
    table: String,
    pointer: usize,
    fields: Vec<Field>,
    filters: Vec<Field>,
    arguments: PgArguments,
}

impl QueryBuilder {
    pub fn new<T>(table: T) -> Self
    where
        T: Into<String>,
    {
        QueryBuilder {
            table: table.into(),
            pointer: 1,
            ..QueryBuilder::default()
        }
    }

    pub fn fields<F>(&mut self, fields: Vec<F>)
    where
        F: Into<String>,
    {
        for field in fields {
            self.field(field);
        }
    }

    pub fn field<F>(&mut self, field: F)
    where
        F: Into<String>,
    {
        self.fields.push(Field {
            value: field.into(),
            position: 0,
        })
    }

    pub fn field_with_argument<'q, F, V>(&mut self, field: F, value: V)
    where
        F: Into<String>,
        V: Encode<'q, Postgres> + Send + Sync + Type<Postgres> + 'q,
    {
        self.fields.push(Field {
            value: field.into(),
            position: self.pointer,
        });
        self.arguments.add(value);
        self.pointer += 1;
    }

    pub fn where_raw<'q, F, V>(&mut self, field: F, value: V)
    where
        F: Into<String>,
        V: Encode<'q, Postgres> + Send + Sync + Type<Postgres> + 'q,
    {
        self.filters.push(Field {
            value: str::replace(
                field.into().as_str(),
                "{index}",
                self.pointer.to_string().as_str(),
            ),
            position: self.pointer,
        });
        self.arguments.add(value);

        self.pointer += 1;
    }

    pub fn where_eq<'q, F, V>(&mut self, field: F, value: V)
    where
        F: Into<String>,
        V: Encode<'q, Postgres> + Send + Sync + Type<Postgres> + 'q,
    {
        self.filters.push(Field {
            value: format!(
                "{field} = ${index}",
                field = field.into(),
                index = self.pointer
            ),
            position: self.pointer,
        });
        self.arguments.add(value);

        self.pointer += 1;
    }

    pub fn where_any<'q, F, V>(&mut self, field: F, value: V)
    where
        F: Into<String>,
        V: Encode<'q, Postgres> + Send + Sync + Type<Postgres> + 'q,
    {
        self.filters.push(Field {
            value: format!(
                "{field} = ANY(${index})",
                field = field.into(),
                index = self.pointer
            ),
            position: self.pointer,
        });
        self.arguments.add(value);

        self.pointer += 1;
    }

    pub fn get_where_query(filters: Vec<Field>) -> String {
        filters
            .into_iter()
            .map(|field| {
                format!(
                    "{field} = ${index}",
                    field = field.value,
                    index = field.position
                )
            })
            .collect::<Vec<String>>()
            .join(" AND ")
    }

    pub fn has_fields(&self) -> bool {
        self.pointer > 1
    }

    pub fn insert_query(self) -> (String, PgArguments) {
        let fields = self
            .fields
            .into_iter()
            .filter(|f| f.position > 0)
            .collect::<Vec<Field>>();

        let sql = format!(
            "INSERT INTO {table} SET ({insert_fields}) VALUES ( {insert_indexes} ) RETURNING id",
            table = self.table,
            insert_fields = fields
                .clone()
                .into_iter()
                .map(|f| format!("${}", f.value))
                .collect::<Vec<String>>()
                .join(", "),
            insert_indexes = fields
                .clone()
                .into_iter()
                .map(|f| format!("${}", f.position))
                .collect::<Vec<String>>()
                .join(", "),
        );

        (sql, self.arguments)
    }

    pub fn update_query(self) -> (String, PgArguments) {
        let fields = self
            .fields
            .into_iter()
            .filter(|f| f.position > 0)
            .collect::<Vec<Field>>();

        let sql = format!(
            "UPDATE {table} SET {update_fields} WHERE {filters}",
            table = self.table,
            update_fields = fields
                .clone()
                .into_iter()
                .map(|field| {
                    format!(
                        "{field} = ${index}",
                        field = field.value,
                        index = field.position
                    )
                })
                .collect::<Vec<String>>()
                .join(", "),
            filters = QueryBuilder::get_where_query(self.filters),
        );

        (sql, self.arguments)
    }

    pub fn select_query(self) -> (String, PgArguments) {
        let fields = self
            .fields
            .into_iter()
            .filter(|f| f.position == 0)
            .collect::<Vec<Field>>();

        let sql = format!(
            "SELECT {select_fields} FROM {table}  WHERE {filters}",
            table = self.table,
            select_fields = fields
                .into_iter()
                .map(|f| f.value)
                .collect::<Vec<String>>()
                .join(", "),
            filters = QueryBuilder::get_where_query(self.filters),
        );

        (sql, self.arguments)
    }

    pub fn delete_query(self) -> (String, PgArguments) {
        let sql = format!(
            r#"DELETE FROM {table} WHERE {filters}"#,
            table = self.table,
            filters = QueryBuilder::get_where_query(self.filters),
        );

        (sql, self.arguments)
    }
}
