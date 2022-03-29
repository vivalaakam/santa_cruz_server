use std::env;

use dotenv::dotenv;
use log::debug;
use sqlx::PgPool;
use tonic::transport::Server;
use tonic_web;

use crate::proto::proto::santa_cruz::workout_service_server::WorkoutServiceServer;
use crate::proto::proto::santa_cruz::workout_set_service_server::WorkoutSetServiceServer;
use crate::workout_service::WorkoutService;
use crate::workout_set::WorkoutSetService;

mod proto;
mod workout_service;
mod workout_set;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPool::connect(&*database_url)
        .await
        .expect("postgresql fails");

    let workout = WorkoutService::new(&pool);
    let workout_set = WorkoutSetService::new(&pool);

    let addr = "[::1]:50051";
    debug!("started: {}", addr);

    let workout = tonic_web::config().enable(WorkoutServiceServer::new(workout));
    let workout_set = tonic_web::config().enable(WorkoutSetServiceServer::new(workout_set));

    Server::builder()
        .accept_http1(true)
        .add_service(workout)
        .add_service(workout_set)
        .serve(addr.parse().expect("cannot parse addr"))
        .await
        .expect("some fails");
}
