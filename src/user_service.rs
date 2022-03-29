use chrono::{DateTime, Utc};
use sqlx::PgPool;
use tonic::{Request, Response, Status};

use crate::me_extension::MeExtension;
use crate::proto::proto::santa_cruz;
use crate::proto::proto::santa_cruz::{MeRequest, User};

pub struct UserService {
    pool: PgPool,
}

type UserRow = (i32, DateTime<Utc>, DateTime<Utc>, String);

pub async fn get_user_by_id(pool: &PgPool, id: i32) -> santa_cruz::User {
    let row: UserRow =
        sqlx::query_as(r#"SELECT id, created_at, updated_at, email FROM users WHERE id = $1"#)
            .bind(id)
            .fetch_one(pool)
            .await
            .expect("get_user_by_id error");

    row.into()
}

impl Into<santa_cruz::User> for UserRow {
    fn into(self) -> santa_cruz::User {
        santa_cruz::User {
            id: self.0,
            created_at: self.1.to_rfc3339(),
            updated_at: self.2.to_rfc3339(),
            email: self.3,
        }
    }
}

impl UserService {
    pub fn new(pool: &PgPool) -> UserService {
        UserService { pool: pool.clone() }
    }
}

#[tonic::async_trait]
impl santa_cruz::user_service_server::UserService for UserService {
    async fn me(&self, request: Request<MeRequest>) -> Result<Response<User>, Status> {
        let extension = request.extensions().get::<MeExtension>().unwrap();

        let reply = get_user_by_id(&self.pool, extension.user_id).await;
        Ok(Response::new(reply))
    }
}
