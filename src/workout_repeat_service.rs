use chrono::{DateTime, Utc};
use sqlx::{postgres::PgArguments, Arguments, PgPool, Row};
use tonic::{Request, Response, Status};

use crate::proto::proto;
use crate::proto::proto::santa_cruz::{
    CreateWorkoutRepeatRequest, DeleteWorkoutRepeatRequest, DeleteWorkoutRepeatResponse,
    GetWorkoutRepeatRequest, GetWorkoutRepeatsRequest, GetWorkoutRepeatsResponse,
    UpdateWorkoutRepeatRequest, WorkoutRepeat,
};

pub struct WorkoutRepeatService {
    pool: PgPool,
}

type WorkoutRepeatRow = (
    i32,
    DateTime<Utc>,
    DateTime<Utc>,
    i32,
    i32,
    i32,
    Option<f64>,
    Option<f64>,
);

impl Into<WorkoutRepeat> for WorkoutRepeatRow {
    fn into(self) -> WorkoutRepeat {
        WorkoutRepeat {
            id: self.0,
            created_at: self.1.to_rfc3339(),
            updated_at: self.2.to_rfc3339(),
            workout_set_id: self.3,
            exercise_id: self.4,
            repeats: self.5,
            weight: self.6,
            time: self.7,
        }
    }
}

impl WorkoutRepeatService {
    pub fn new(pool: &PgPool) -> WorkoutRepeatService {
        WorkoutRepeatService { pool: pool.clone() }
    }

    pub async fn get_workout_repeat_by_id(&self, id: i32) -> WorkoutRepeat {
        let row: WorkoutRepeatRow = sqlx::query_as(
            r#"SELECT id, created_at, updated_at, workout_set_id, exercise_id, repeats, weight, time FROM workout_repeats WHERE id = $1"#,
        )
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .expect("get_workout_repeat_by_id error");

        row.into()
    }
}

#[tonic::async_trait]
impl proto::santa_cruz::workout_repeat_service_server::WorkoutRepeatService
    for WorkoutRepeatService
{
    async fn get_workout_repeat(
        &self,
        request: Request<GetWorkoutRepeatRequest>,
    ) -> Result<Response<WorkoutRepeat>, Status> {
        let GetWorkoutRepeatRequest { id } = &request.into_inner();

        let reply = self.get_workout_repeat_by_id(*id).await;
        Ok(Response::new(reply))
    }

    async fn create_workout_repeat(
        &self,
        request: Request<CreateWorkoutRepeatRequest>,
    ) -> Result<Response<WorkoutRepeat>, Status> {
        let CreateWorkoutRepeatRequest {
            workout_set_id,
            exercise_id,
            repeats,
            weight,
            time,
        } = &request.into_inner();

        let mut arguments = PgArguments::default();
        let mut params = vec![];

        params.push("workout_set_id");
        arguments.add(workout_set_id);

        params.push("exercise_id");
        arguments.add(exercise_id);

        if let Some(repeats) = repeats {
            params.push("repeats");
            arguments.add(repeats);
        }

        if let Some(weight) = weight {
            params.push("weight");
            arguments.add(weight);
        }

        if let Some(time) = time {
            params.push("time");
            arguments.add(time);
        }

        let query = format!(
            "INSERT INTO workout_repeats ( {fields}) VALUES ( {indexes} ) RETURNING id",
            fields = params.join(", "),
            indexes = params
                .iter()
                .enumerate()
                .map(|ind| format!("${}", ind.0 + 1))
                .collect::<Vec<String>>()
                .join(", "),
        );

        let rec = sqlx::query_with(&*query, arguments)
            .fetch_one(&self.pool)
            .await
            .expect("create_workout_repeat error");

        let reply = self.get_workout_repeat_by_id(rec.get::<i32, _>("id")).await;
        Ok(Response::new(reply))
    }

    async fn update_workout_repeat(
        &self,
        request: Request<UpdateWorkoutRepeatRequest>,
    ) -> Result<Response<WorkoutRepeat>, Status> {
        let UpdateWorkoutRepeatRequest {
            id,
            repeats,
            weight,
            time,
        } = &request.into_inner();
        let original = self.get_workout_repeat_by_id(*id).await;

        let mut arguments = PgArguments::default();

        let mut params = vec![];

        if let Some(repeats) = repeats {
            params.push("repeats");
            arguments.add(repeats)
        }

        if let Some(weight) = weight {
            params.push("weight");
            arguments.add(weight)
        }

        if let Some(time) = time {
            params.push("time");
            arguments.add(time)
        }

        if params.len() == 0 {
            return Ok(Response::new(original));
        }

        params.push("updated_at");
        arguments.add(Utc::now());

        let mut fields = vec![];

        for i in params.iter().enumerate() {
            fields.push(format!("{key} = ${index}", key = i.1, index = i.0 + 1));
        }

        let query = format!(
            "UPDATE workout_repeats SET {fields} WHERE id = ${index}",
            fields = fields.join(", "),
            index = params.len() + 1
        );

        arguments.add(id);

        sqlx::query_with(&*query, arguments)
            .execute(&self.pool)
            .await
            .expect("update_workout_repeat error");

        let reply = self.get_workout_repeat_by_id(*id).await;
        Ok(Response::new(reply))
    }

    async fn delete_workout_repeat(
        &self,
        request: Request<DeleteWorkoutRepeatRequest>,
    ) -> Result<Response<DeleteWorkoutRepeatResponse>, Status> {
        let DeleteWorkoutRepeatRequest { id } = &request.into_inner();

        sqlx::query(r#"DELETE FROM workout_repeats WHERE id = $1 "#)
            .bind(id)
            .execute(&self.pool)
            .await
            .expect("update_workout_repeat error");

        Ok(Response::new(DeleteWorkoutRepeatResponse {}))
    }

    async fn get_workout_repeats(
        &self,
        _request: Request<GetWorkoutRepeatsRequest>,
    ) -> Result<Response<GetWorkoutRepeatsResponse>, Status> {
        let rows: Vec<WorkoutRepeatRow> = sqlx::query_as(
            r#"SELECT id, created_at, updated_at, workout_set_id, exercise_id, repeats, weight, time FROM workout_repeats"#,
        )
            .fetch_all(&self.pool)
            .await
            .expect("get_workout_repeat_by_id error");

        let workout_repeats = rows.into_iter().map(|row| row.into()).collect();

        Ok(Response::new(GetWorkoutRepeatsResponse { workout_repeats }))
    }
}
