use chrono::{DateTime, Utc};
use sqlx::{Arguments, PgPool, postgres::PgArguments};
use tonic::{Request, Response, Status};

use crate::proto::proto;
use crate::proto::proto::santa_cruz::{
    CreateWorkoutRequest, DeleteWorkoutRequest, DeleteWorkoutResponse, GetWorkoutRequest,
    GetWorkoutsRequest, GetWorkoutsResponse, UpdateWorkoutRequest, Workout,
};

pub struct WorkoutService {
    pool: PgPool,
}

type WorkoutRow = (
    i32,
    i32,
    DateTime<Utc>,
    DateTime<Utc>,
    DateTime<Utc>,
    i32,
    String,
);

impl Into<Workout> for WorkoutRow {
    fn into(self) -> Workout {
        Workout {
            id: self.0,
            status: self.1,
            day: self.2.to_rfc3339(),
            created_at: self.3.to_rfc3339(),
            updated_at: self.4.to_rfc3339(),
            rate: self.5,
            comment: self.6,
        }
    }
}

impl WorkoutService {
    pub fn new(pool: &PgPool) -> WorkoutService {
        WorkoutService { pool: pool.clone() }
    }

    pub async fn get_workout_by_id(&self, id: i32) -> Workout {
        let row: WorkoutRow = sqlx::query_as(
            r#"SELECT id, status, day, created_at, updated_at, rate, comment FROM workouts WHERE id = $1"#,
        )
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .expect("get_workout_by_id error");

        row.into()
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

        let rec: (i32, ) = sqlx::query_as(
            r#"INSERT INTO workouts ( status, day ) VALUES ( $1 , $2 ) RETURNING id"#,
        )
            .bind(0)
            .bind(Utc::now())
            .fetch_one(&self.pool)
            .await
            .expect("create_workout error");

        let reply = self.get_workout_by_id(rec.0 as i32).await;
        Ok(Response::new(reply))
    }

    async fn update_workout(
        &self,
        request: Request<UpdateWorkoutRequest>,
    ) -> Result<Response<Workout>, Status> {
        let UpdateWorkoutRequest {
            id,
            status,
            day,
            rate,
            comment,
        } = &request.into_inner();
        let original = self.get_workout_by_id(*id).await;

        let mut arguments = PgArguments::default();

        let mut params = vec![];

        if let Some(status) = status {
            params.push("status");
            arguments.add(status)
        }

        if let Some(day) = day {
            params.push("day");
            arguments.add(day)
        }

        if let Some(rate) = rate {
            params.push("rate");
            arguments.add(rate);
        }

        if let Some(comment) = comment {
            params.push("comment");
            arguments.add(comment);
        }

        if params.len() == 0 {
            return Ok(Response::new(original));
        }

        params.push("updated_at");
        arguments.add(Utc::now());

        let mut fields = vec![];

        for i in 0..params.len() {
            let item = params.get(i).expect("item should be there");
            fields.push(format!("{key} = ${index}", key = item, index = i + 1));
        }

        let query = format!(
            "UPDATE workouts SET {fields} WHERE id = ${index}",
            fields = fields.join(", "),
            index = params.len() + 1
        );

        arguments.add(id);

        sqlx::query_with(&*query, arguments)
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
        let rows: Vec<WorkoutRow> = sqlx::query_as(
            r#"SELECT id, status, day, created_at, updated_at, rate, comment FROM workouts"#,
        )
            .fetch_all(&self.pool)
            .await
            .expect("get_workout_by_id error");

        let workouts = rows.into_iter().map(|row| row.into()).collect();

        Ok(Response::new(GetWorkoutsResponse { workouts }))
    }
}
