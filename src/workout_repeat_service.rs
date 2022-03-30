use std::collections::HashMap;

use chrono::{DateTime, Utc};
use sqlx::postgres::PgRow;
use sqlx::types::Json;
use sqlx::{postgres::PgArguments, Arguments, PgPool, Row};
use tonic::{Request, Response, Status};

use crate::me_extension::MeExtension;
use crate::proto::proto;
use crate::proto::proto::santa_cruz::id_query::Value;
use crate::proto::proto::santa_cruz::{
    CreateWorkoutRepeatRequest, DeleteWorkoutRepeatRequest, DeleteWorkoutRepeatResponse,
    GetWorkoutRepeatRequest, GetWorkoutRepeatsRequest, GetWorkoutRepeatsResponse,
    UpdateWorkoutRepeatRequest, WorkoutRepeat,
};
use crate::WorkoutSetService;

pub struct WorkoutRepeatService {
    pool: PgPool,
}

impl From<PgRow> for WorkoutRepeat {
    fn from(row: PgRow) -> Self {
        WorkoutRepeat {
            id: row.get::<i32, _>("id"),
            created_at: row.get::<DateTime<Utc>, _>("created_at").to_rfc3339(),
            updated_at: row.get::<DateTime<Utc>, _>("updated_at").to_rfc3339(),
            workout_set_id: row.get::<i32, _>("workout_set_id"),
            exercise_id: row.get::<i32, _>("exercise_id"),
            repeats: row.get::<i32, _>("repeats"),
            weight: row.get::<Option<f64>, _>("weight"),
            time: row.get::<Option<f64>, _>("time"),
        }
    }
}

impl WorkoutRepeatService {
    pub fn new(pool: &PgPool) -> WorkoutRepeatService {
        WorkoutRepeatService { pool: pool.clone() }
    }

    pub async fn get_workout_repeat_by_id(
        pool: &PgPool,
        id: i32,
        user_id: i32,
    ) -> Option<WorkoutRepeat> {
        let mut arguments = PgArguments::default();
        arguments.add(id);
        arguments.add(user_id);
        sqlx::query_with(
            r#"
                SELECT id, created_at, updated_at, workout_set_id, exercise_id, repeats, weight, time
                FROM workout_repeats
                WHERE id = $1 AND ((permissions ->> CAST($2 as text))::integer > 0 OR (permissions ->> '0')::integer > 0)
            "#, arguments,
        )
            .fetch_one(pool)
            .await
            .map(|r| r.into())
            .ok()
    }

    pub async fn return_workout_repeat_by_id(
        &self,
        id: i32,
        user_id: i32,
    ) -> Result<Response<WorkoutRepeat>, Status> {
        WorkoutRepeatService::get_workout_repeat_by_id(&self.pool, id, user_id)
            .await
            .map(|reply| Response::new(reply))
            .ok_or(Status::not_found(format!(
                "workout_repeat #{} not found",
                id.to_string()
            )))
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
        let MeExtension { user_id } = request.extensions().get::<MeExtension>().unwrap();
        let GetWorkoutRepeatRequest { id } = request.get_ref();

        self.return_workout_repeat_by_id(*id, *user_id).await
    }

    async fn create_workout_repeat(
        &self,
        request: Request<CreateWorkoutRepeatRequest>,
    ) -> Result<Response<WorkoutRepeat>, Status> {
        let MeExtension { user_id } = request.extensions().get::<MeExtension>().unwrap();

        let CreateWorkoutRepeatRequest {
            workout_set_id,
            exercise_id,
            repeats,
            weight,
            time,
        } = request.get_ref();

        let workout_set =
            WorkoutSetService::get_workout_set_by_id(&self.pool, *workout_set_id, *user_id).await;

        if workout_set.is_none() {
            return Err(Status::permission_denied(format!(
                "permissions not found for workout_set #{}",
                workout_set_id.to_string()
            )));
        }

        let mut arguments = PgArguments::default();
        let mut params = vec![];

        params.push("workout_set_id");
        arguments.add(workout_set_id);

        params.push("exercise_id");
        arguments.add(exercise_id);

        params.push("permissions");

        let mut permissions = HashMap::new();
        permissions.insert(user_id, 2);
        arguments.add(Json(permissions));

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

        self.return_workout_repeat_by_id(rec.get::<i32, _>("id"), *user_id)
            .await
    }

    async fn update_workout_repeat(
        &self,
        request: Request<UpdateWorkoutRepeatRequest>,
    ) -> Result<Response<WorkoutRepeat>, Status> {
        let MeExtension { user_id } = request.extensions().get::<MeExtension>().unwrap();

        let UpdateWorkoutRepeatRequest {
            id,
            repeats,
            weight,
            time,
        } = &request.get_ref();

        let original =
            WorkoutRepeatService::get_workout_repeat_by_id(&self.pool, *id, *user_id).await;

        if original.is_none() {
            return Err(Status::not_found(format!(
                "workout_repeat #{} not found",
                id.to_string()
            )));
        }

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
            return Ok(Response::new(original.unwrap()));
        }

        params.push("updated_at");
        arguments.add(Utc::now());

        let fields = params
            .into_iter()
            .enumerate()
            .map(|row| format!("{key} = ${index}", key = row.1, index = row.0 + 1))
            .collect::<Vec<String>>();

        let query = format!(
            "UPDATE workout_repeats SET {fields} WHERE id = ${index}",
            fields = fields.join(", "),
            index = fields.len() + 1
        );

        arguments.add(id);

        sqlx::query_with(&*query, arguments)
            .execute(&self.pool)
            .await
            .expect("update_workout_repeat error");

        self.return_workout_repeat_by_id(*id, *user_id).await
    }

    async fn delete_workout_repeat(
        &self,
        request: Request<DeleteWorkoutRepeatRequest>,
    ) -> Result<Response<DeleteWorkoutRepeatResponse>, Status> {
        let MeExtension { user_id } = &request.extensions().get::<MeExtension>().unwrap();
        let DeleteWorkoutRepeatRequest { id } = &request.get_ref();

        let mut params = vec![
            "((permissions ->> CAST($1 as text))::integer > 0 OR (permissions ->> '0')::integer > 0)".to_string()
        ];
        let mut arguments = PgArguments::default();
        arguments.add(user_id);

        params.push(format!("id = ${index}", index = params.len() + 1));
        arguments.add(id);

        let query = format!(
            r#"DELETE FROM workout_repeats WHERE {}"#,
            params.join(" AND ")
        );

        sqlx::query_with(query.as_str(), arguments)
            .execute(&self.pool)
            .await
            .expect("update_workout_repeat error");

        Ok(Response::new(DeleteWorkoutRepeatResponse {}))
    }

    async fn get_workout_repeats(
        &self,
        request: Request<GetWorkoutRepeatsRequest>,
    ) -> Result<Response<GetWorkoutRepeatsResponse>, Status> {
        let MeExtension { user_id } = request.extensions().get::<MeExtension>().unwrap();
        let GetWorkoutRepeatsRequest { workout_set_id } = request.get_ref();

        let mut params = vec!["((permissions ->> CAST($1 as text))::integer > 0 OR (permissions ->> '0')::integer > 0)".to_string()];
        let mut arguments = PgArguments::default();
        arguments.add(user_id);

        if let Some(id_query) = workout_set_id {
            match id_query.clone().value.unwrap() {
                Value::Unknown(_) => {}
                Value::Eq(value) => {
                    params.push(format!("workout_set_id = ${}", params.len() + 1));
                    arguments.add(value.value)
                }
                Value::In(value) => {
                    params.push(format!("workout_set_id = ANY(${})", params.len() + 1));
                    arguments.add(value.value)
                }
            }
        }

        let query = format!(
            r#"SELECT id, created_at, updated_at, workout_set_id, exercise_id, repeats, weight, time FROM workout_repeats WHERE {}"#,
            params.join(" AND ")
        );

        let workout_repeats = sqlx::query_with(query.as_str(), arguments)
            .fetch_all(&self.pool)
            .await
            .expect("get_workout_repeat_by_id error")
            .into_iter()
            .map(|row| row.into())
            .collect();

        Ok(Response::new(GetWorkoutRepeatsResponse { workout_repeats }))
    }
}
