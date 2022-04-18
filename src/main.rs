use std::env;

use dotenv::dotenv;
use log::debug;
use sqlx::PgPool;
use tonic::transport::Server;
use tonic_web;

use crate::auth_interceptor::{load_sessions, AuthInterceptor};
use crate::auth_service::AuthService;
use crate::proto::proto::santa_cruz::auth_service_server::AuthServiceServer;
use crate::proto::proto::santa_cruz::exercise_service_server::ExerciseServiceServer;
use crate::proto::proto::santa_cruz::user_service_server::UserServiceServer;
use crate::proto::proto::santa_cruz::workout_repeat_service_server::WorkoutRepeatServiceServer;
use crate::proto::proto::santa_cruz::workout_service_server::WorkoutServiceServer;
use crate::proto::proto::santa_cruz::workout_set_service_server::WorkoutSetServiceServer;
use crate::queryable::Queryable;
use crate::services::exercise::ExerciseService;
use crate::sessions_cache::SessionsCache;
use crate::user_service::UserService;
use crate::workout_repeat_service::WorkoutRepeatService;
use crate::workout_service::WorkoutService;
use crate::workout_set_service::WorkoutSetService;

mod auth_interceptor;
mod auth_service;
mod me_extension;
mod proto;
mod query_builder;
mod queryable;
mod services;
mod session_service;
mod sessions_cache;
mod user_service;
mod workout_repeat_service;
mod workout_service;
mod workout_set_service;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPool::connect(&*database_url)
        .await
        .expect("postgresql fails");

    let cache = load_sessions(&pool).await;

    let addr = "[::1]:50051";
    debug!("started: {}", addr);

    let auth = tonic_web::config().enable(AuthServiceServer::new(AuthService::new(
        &pool,
        cache.clone(),
    )));

    let user = tonic_web::config().enable(UserServiceServer::with_interceptor(
        UserService::new(&pool),
        AuthInterceptor::new(cache.clone()),
    ));

    let exercise = tonic_web::config().enable(ExerciseServiceServer::with_interceptor(
        ExerciseService::new(&pool),
        AuthInterceptor::new(cache.clone()),
    ));

    let workout = tonic_web::config().enable(WorkoutServiceServer::with_interceptor(
        WorkoutService::new(&pool),
        AuthInterceptor::new(cache.clone()),
    ));

    let workout_repeat = tonic_web::config().enable(WorkoutRepeatServiceServer::with_interceptor(
        WorkoutRepeatService::new(&pool),
        AuthInterceptor::new(cache.clone()),
    ));

    let workout_set = tonic_web::config().enable(WorkoutSetServiceServer::with_interceptor(
        WorkoutSetService::new(&pool),
        AuthInterceptor::new(cache.clone()),
    ));

    Server::builder()
        .accept_http1(true)
        .add_service(auth)
        .add_service(user)
        .add_service(exercise)
        .add_service(workout)
        .add_service(workout_repeat)
        .add_service(workout_set)
        .serve(addr.parse().expect("cannot parse addr"))
        .await
        .expect("some fails");
}
