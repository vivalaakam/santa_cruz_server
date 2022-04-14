use std::collections::HashMap;

use chrono::{DateTime, Utc};
use sqlx::postgres::PgRow;
use sqlx::types::Json;
use sqlx::{PgPool, Row};
use tonic::{Request, Response, Status};

use crate::me_extension::MeExtension;
use crate::proto::proto;
use crate::proto::proto::santa_cruz::id_query::Value;
use crate::proto::proto::santa_cruz::{
    CreateWorkoutRepeatRequest, DeleteWorkoutRepeatRequest, DeleteWorkoutRepeatResponse,
    GetWorkoutRepeatRequest, GetWorkoutRepeatsRequest, GetWorkoutRepeatsResponse,
    UpdateWorkoutRepeatRequest, WorkoutRepeat,
};
use crate::query_builder::QueryBuilder;
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
        let mut query_builder = QueryBuilder::new("workout_repeats");
        query_builder.fields(vec![
            "id",
            "created_at",
            "updated_at",
            "workout_set_id",
            "exercise_id",
            "repeats",
            "weight",
            "time",
        ]);
        query_builder.where_raw("((permissions ->> CAST(${index} as text))::integer > 0 OR (permissions ->> '0')::integer > 0)", user_id);
        query_builder.where_eq("id", id);

        let sql = query_builder.select_query();

        sqlx::query_with(sql.0.as_str(), sql.1)
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

        let mut query_builder = QueryBuilder::new("workout_repeats");

        query_builder.field_with_argument("workout_set_id", workout_set_id);
        query_builder.field_with_argument("exercise_id", exercise_id);

        let mut permissions = HashMap::new();
        permissions.insert(user_id, 2);
        query_builder.field_with_argument("permissions", Json(permissions));

        if let Some(repeats) = repeats {
            query_builder.field_with_argument("repeats", repeats);
        }

        if let Some(weight) = weight {
            query_builder.field_with_argument("weight", weight);
        }

        if let Some(time) = time {
            query_builder.field_with_argument("time", time);
        }

        let sql = query_builder.insert_query();

        let rec = sqlx::query_with(sql.0.as_str(), sql.1)
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

        let mut query_builder = QueryBuilder::new("workout_repeats");

        if let Some(repeats) = repeats {
            query_builder.field_with_argument("repeats", repeats);
        }

        if let Some(weight) = weight {
            query_builder.field_with_argument("weight", weight);
        }

        if let Some(time) = time {
            query_builder.field_with_argument("time", time);
        }

        if !query_builder.has_fields() {
            return Ok(Response::new(original.unwrap()));
        }

        query_builder.field_with_argument("updated_at", Utc::now());

        query_builder.where_eq("id", id);

        let sql = query_builder.update_query();

        sqlx::query_with(sql.0.as_str(), sql.1)
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

        let mut query_builder = QueryBuilder::new("workout_repeats");
        query_builder.where_raw("((permissions ->> CAST(${index} as text))::integer > 0 OR (permissions ->> '0')::integer > 0)", user_id);
        query_builder.where_eq("id", id);

        let sql = query_builder.delete_query();

        sqlx::query_with(sql.0.as_str(), sql.1)
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

        let mut query_builder = QueryBuilder::new("workout_repeats");
        query_builder.fields(vec![
            "id",
            "created_at",
            "updated_at",
            "workout_set_id",
            "exercise_id",
            "repeats",
            "weight",
            "time",
        ]);
        query_builder.where_raw("((permissions ->> CAST(${index} as text))::integer > 0 OR (permissions ->> '0')::integer > 0)", user_id);

        if let Some(id_query) = workout_set_id {
            match id_query.clone().value.unwrap() {
                Value::Unknown(_) => {}
                Value::Eq(value) => {
                    query_builder.where_eq("workout_set_id", value.value);
                }
                Value::In(value) => {
                    query_builder.where_any("workout_set_id", value.value);
                }
            }
        }

        let sql = query_builder.select_query();

        let workout_repeats = sqlx::query_with(sql.0.as_str(), sql.1)
            .fetch_all(&self.pool)
            .await
            .expect("get_workout_repeat_by_id error")
            .into_iter()
            .map(|row| row.into())
            .collect();

        Ok(Response::new(GetWorkoutRepeatsResponse { workout_repeats }))
    }
}
