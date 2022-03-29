use std::env;
use std::sync::Arc;

use dotenv::dotenv;
use log::debug;
use sqlx::PgPool;
use tonic::Request;
use tonic::transport::Server;
use tonic_web;

use crate::auth_interception::AuthInterception;
use crate::auth_service::AuthService;
use crate::exercise_service::ExerciseService;
use crate::proto::proto::santa_cruz::auth_service_server::AuthServiceServer;
use crate::proto::proto::santa_cruz::exercise_service_server::ExerciseServiceServer;
use crate::proto::proto::santa_cruz::user_service_server::UserServiceServer;
use crate::proto::proto::santa_cruz::workout_repeat_service_server::WorkoutRepeatServiceServer;
use crate::proto::proto::santa_cruz::workout_service_server::WorkoutServiceServer;
use crate::proto::proto::santa_cruz::workout_set_service_server::WorkoutSetServiceServer;
use crate::sessions_cache::SessionsCache;
use crate::user_service::UserService;
use crate::workout_repeat_service::WorkoutRepeatService;
use crate::workout_service::WorkoutService;
use crate::workout_set_service::WorkoutSetService;

mod auth_interception;
mod auth_service;
mod exercise_service;
mod me_extension;
mod proto;
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

    let cache = Arc::new(SessionsCache::default());

    let auth_interception = AuthInterception::new(&pool, cache.clone());

    auth_interception.load_sessions().await;

    let auth = AuthService::new(&pool, cache.clone());
    let user = UserService::new(&pool);
    let exercise = ExerciseService::new(&pool);
    let workout = WorkoutService::new(&pool);
    let workout_repeat = WorkoutRepeatService::new(&pool);
    let workout_set = WorkoutSetService::new(&pool);

    let addr = "[::1]:50051";
    debug!("started: {}", addr);

    let auth = tonic_web::config().enable(AuthServiceServer::new(auth));
    let user = tonic_web::config().enable(UserServiceServer::with_interceptor(
        user,
        move |req: Request<()>| auth_interception.check(req),
    ));

    let exercise = tonic_web::config().enable(ExerciseServiceServer::new(exercise));
    let workout = tonic_web::config().enable(WorkoutServiceServer::new(workout));
    let workout_repeat =
        tonic_web::config().enable(WorkoutRepeatServiceServer::new(workout_repeat));
    let workout_set = tonic_web::config().enable(WorkoutSetServiceServer::new(workout_set));

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
