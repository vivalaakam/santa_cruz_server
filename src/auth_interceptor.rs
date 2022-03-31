use std::sync::Arc;

use sqlx::PgPool;
use tonic::service::Interceptor;
use tonic::Status;

use crate::me_extension::MeExtension;
use crate::SessionsCache;

#[derive(Clone)]
pub struct AuthInterceptor {
    cache: Arc<SessionsCache>,
}

impl AuthInterceptor {
    pub fn new(cache: Arc<SessionsCache>) -> AuthInterceptor {
        AuthInterceptor { cache }
    }
}

fn get_token(token: &str) -> Option<&str> {
    let (bearer, token) = token.split_at(token.find(' ').unwrap());

    if bearer.to_lowercase() != "bearer" {
        return None;
    }

    return Some(token.trim());
}

pub async fn load_sessions(pool: &PgPool) -> Arc<SessionsCache> {
    let rows: Vec<(String, i32)> = sqlx::query_as(r#"SELECT token, user_id FROM sessions"#)
        .fetch_all(pool)
        .await
        .expect("load_sessions error");

    let cache = Arc::new(SessionsCache::default());

    for row in rows {
        cache.insert(row.0, row.1);
    }

    cache
}

impl Interceptor for AuthInterceptor {
    fn call(&mut self, request: tonic::Request<()>) -> Result<tonic::Request<()>, Status> {
        let mut req = request;
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
