use dotenv::dotenv;
use std::env;
use log::debug;
use sqlx::PgPool;
use tonic::transport::Server;
use crate::proto::proto::santa_cruz::workout_service_server::WorkoutServiceServer;
use crate::workout_service::WorkoutService;

mod proto;
mod workout_service;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPool::connect(&*database_url).await.expect("postgresql fails");

    let workout = WorkoutService::new(&pool);

    let addr = "[::1]:50051";
    debug!("started: {}", addr);

    Server::builder()
        .add_service(WorkoutServiceServer::new(workout))
        .serve(addr.parse().expect("cannot parse addr"))
        .await.expect("some fails");
}
