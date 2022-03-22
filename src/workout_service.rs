use chrono::{NaiveDateTime, Utc};
use sqlx::SqlitePool;
use tonic::{Request, Response, Status};

use crate::proto::proto;
use crate::proto::proto::santa_cruz::{
    CreateWorkoutRequest, DeleteWorkoutRequest, DeleteWorkoutResponse, GetWorkoutRequest,
    GetWorkoutsRequest, GetWorkoutsResponse, UpdateWorkoutRequest, Workout,
};

pub struct WorkoutService {
    pool: SqlitePool,
}

type WorkoutRow = (i32, i32, NaiveDateTime, NaiveDateTime, NaiveDateTime);

impl WorkoutService {
    pub fn new(pool: &SqlitePool) -> WorkoutService {
        WorkoutService { pool: pool.clone() }
    }

    pub async fn get_workout_by_id(&self, id: i32) -> Workout {
        let row: WorkoutRow = sqlx::query_as(
            r#"SELECT id, status, day, created_at, updated_at FROM workouts WHERE id = $1"#,
        )
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .expect("get_workout_by_id error");

        Workout {
            id: row.0,
            status: row.1,
            day: row.2.to_string(),
            created_at: row.3.to_string(),
            updated_at: row.4.to_string(),
        }
    }
}

#[tonic::async_trait]
impl proto::santa_cruz::workout_service_server::WorkoutService for WorkoutService {
    async fn get_workout(
        &self,
        request: Request<GetWorkoutRequest>,
    ) -> Result<Response<Workout>, Status> {
        let GetWorkoutRequest { id } = &request.into_inner();

        let reply = self.get_workout_by_id(*id).await;
        Ok(Response::new(reply))
    }

    async fn create_workout(
        &self,
        request: Request<CreateWorkoutRequest>,
    ) -> Result<Response<Workout>, Status> {
        let CreateWorkoutRequest {} = &request.into_inner();

        let id = sqlx::query(r#"INSERT INTO workouts ( status, day ) VALUES ( $1 , $2 )"#)
            .bind(0)
            .bind(
                Utc::now()
                    .naive_utc()
                    .format("%Y-%m-%d 00:00:00")
                    .to_string(),
            )
            .execute(&self.pool)
            .await
            .expect("create_workout error")
            .last_insert_rowid();

        let reply = self.get_workout_by_id(id as i32).await;
        Ok(Response::new(reply))
    }

    async fn update_workout(
        &self,
        request: Request<UpdateWorkoutRequest>,
    ) -> Result<Response<Workout>, Status> {
        let UpdateWorkoutRequest { id, status, day } = &request.into_inner();
        let original = self.get_workout_by_id(*id).await;

        sqlx::query(r#"UPDATE workouts SET status = $1, day = $2, updated_at = $3 WHERE id = $4 "#)
            .bind(match status {
                None => original.status,
                Some(val) => *val,
            })
            .bind(match day {
                None => original.day,
                Some(val) => NaiveDateTime::parse_from_str(val, "%Y-%m-%d %H:%M:%S")
                    .expect("update_workout invalid day")
                    .format("%Y-%m-%d 00:00:00")
                    .to_string(),
            })
            .bind(
                Utc::now()
                    .naive_utc()
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string(),
            )
            .bind(id)
            .execute(&self.pool)
            .await
            .expect("update_workout error");

        let reply = self.get_workout_by_id(*id).await;
        Ok(Response::new(reply))
    }

    async fn delete_workout(
        &self,
        request: Request<DeleteWorkoutRequest>,
    ) -> Result<Response<DeleteWorkoutResponse>, Status> {
        let DeleteWorkoutRequest { id } = &request.into_inner();

        sqlx::query(r#"DELETE FROM workouts WHERE id = $1 "#)
            .bind(id)
            .execute(&self.pool)
            .await
            .expect("update_workout error");

        Ok(Response::new(DeleteWorkoutResponse {}))
    }

    async fn get_workouts(
        &self,
        _request: Request<GetWorkoutsRequest>,
    ) -> Result<Response<GetWorkoutsResponse>, Status> {
        let rows: Vec<WorkoutRow> =
            sqlx::query_as(r#"SELECT id, status, day, created_at, updated_at FROM workouts"#)
                .fetch_all(&self.pool)
                .await
                .expect("get_workout_by_id error");

        let workouts = rows
            .into_iter()
            .map(|row| Workout {
                id: row.0,
                status: row.1,
                day: row.2.to_string(),
                created_at: row.3.to_string(),
                updated_at: row.4.to_string(),
            })
            .collect();

        Ok(Response::new(GetWorkoutsResponse { workouts }))
    }
}
