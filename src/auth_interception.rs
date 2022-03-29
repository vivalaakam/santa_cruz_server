use std::sync::Arc;

use sqlx::PgPool;
use tonic::{Request, Status};

use crate::me_extension::MeExtension;
use crate::SessionsCache;

#[derive(Clone)]
pub struct AuthInterception {
    pool: PgPool,
    cache: Arc<SessionsCache>,
}

fn get_token(token: &str) -> Option<&str> {
    let (bearer, token) = token.split_at(token.find(' ').unwrap());

    if bearer.to_lowercase() != "bearer" {
        return None;
    }

    return Some(token.trim());
}

impl AuthInterception {
    pub fn new(pool: &PgPool, cache: Arc<SessionsCache>) -> AuthInterception {
        AuthInterception {
            pool: pool.clone(),
            cache,
        }
    }

    pub async fn load_sessions(&self) {
        let rows: Vec<(String, i32)> = sqlx::query_as(r#"SELECT token, user_id FROM sessions"#)
            .fetch_all(&self.pool)
            .await
            .expect("load_sessions error");

        for row in rows {
            self.cache.insert(row.0, row.1);
        }
    }

    pub fn check(&self, mut req: Request<()>) -> Result<Request<()>, Status> {
        match req.metadata().get("authorization") {
            Some(t) => {
                let token = get_token(t.to_str().expect("token should be there"));

                if token.is_none() {
                    return Err(Status::unauthenticated("No valid auth token"));
                }

                let user_id = self.cache.get(token.unwrap().to_string());

                if user_id.is_none() {
                    return Err(Status::unauthenticated("Session not found"));
                }

                req.extensions_mut().insert(MeExtension {
                    user_id: user_id.unwrap(),
                });

                Ok(req)
            }
            _ => Err(Status::unauthenticated("No valid auth token")),
        }
    }
}
