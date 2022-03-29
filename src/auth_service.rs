use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use rand_core::OsRng;
use sqlx::PgPool;
use std::sync::Arc;
use tonic::{Request, Response, Status};

use crate::proto::proto::santa_cruz;
use crate::proto::proto::santa_cruz::{LoginRequest, LogoutRequest, LogoutResponse};
use crate::user_service::get_user_by_id;
use crate::SessionsCache;

pub struct AuthService {
    pool: PgPool,
    cache: Arc<SessionsCache>,
}

impl AuthService {
    pub fn new(pool: &PgPool, cache: Arc<SessionsCache>) -> AuthService {
        AuthService {
            pool: pool.clone(),
            cache,
        }
    }
}

pub fn hash_password(password: String) -> Result<String, String> {
    let salt = SaltString::generate(&mut OsRng);
    let pass = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .expect("oops")
        .to_string();

    Ok(pass)
}

pub fn verify_password(
    password: String,
    password_hash: String,
) -> argon2::password_hash::Result<()> {
    let parsed_hash = PasswordHash::new(&password_hash)?;
    Argon2::default().verify_password(password.as_bytes(), &parsed_hash)
}

#[tonic::async_trait]
impl santa_cruz::auth_service_server::AuthService for AuthService {
    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<santa_cruz::User>, Status> {
        let LoginRequest {
            email,
            password,
            token,
            device_name,
        } = &request.into_inner();

        let row = sqlx::query_as::<_, (i32, String, String)>(
            r#"SELECT id, email, password FROM users WHERE email = $1"#,
        )
        .bind(email)
        .fetch_one(&self.pool)
        .await;

        let mut user_id = None;

        if row.is_err() {
            let rec: (i32,) = sqlx::query_as(
                r#"INSERT INTO users ( email, password ) VALUES ( $1 , $2 ) RETURNING id"#,
            )
            .bind(email)
            .bind(hash_password(password.to_string()).unwrap())
            .fetch_one(&self.pool)
            .await
            .expect("create_workout error");

            user_id = Some(rec.0)
        } else {
            let row = row.unwrap();
            let check_password = verify_password(password.to_string(), row.2);

            if check_password.is_ok() {
                user_id = Some(row.0);
            }
        }

        if user_id.is_none() {
            return Err(Status::not_found("user not found"));
        }

        let user_id = user_id.unwrap();

        self.cache.insert(token.to_string(), user_id);

        sqlx::query(
            r#"INSERT INTO sessions ( user_id, token, device_name ) VALUES ( $1 , $2, $3 ) RETURNING id"#,
        )
            .bind(user_id)
            .bind(token)
            .bind(device_name)
            .execute(&self.pool)
            .await
            .expect("create session error");

        let reply = get_user_by_id(&self.pool, user_id).await;
        Ok(Response::new(reply))
    }

    async fn logout(
        &self,
        request: Request<LogoutRequest>,
    ) -> Result<Response<LogoutResponse>, Status> {
        let LogoutRequest { user_id, token } = request.into_inner();

        self.cache.remove(token.to_string());

        sqlx::query(r#"DELETE FROM sessions WHERE user_id = $1 AND token = $2 "#)
            .bind(user_id)
            .bind(token)
            .execute(&self.pool)
            .await
            .expect("logout error");

        Ok(Response::new(LogoutResponse {}))
    }
}
