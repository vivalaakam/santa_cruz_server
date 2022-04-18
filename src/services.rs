use crate::me_extension::MeExtension;
use crate::proto::proto;
use crate::query_builder::QueryBuilder;
use crate::Queryable;
use chrono::{DateTime, Utc};
use sqlx::postgres::PgRow;
use sqlx::types::Json;
use sqlx::{PgPool, Row};
use std::collections::HashMap;
use tonic::{Request, Response, Status};
pub mod exercise {
    use super::*;
    use crate::proto::proto::santa_cruz::{
        CreateExerciseRequest, DeleteExerciseRequest, DeleteExerciseResponse, Exercise,
        GetExerciseRequest, GetExercisesRequest, GetExercisesResponse, UpdateExerciseRequest,
    };
    impl From<PgRow> for Exercise {
        fn from(row: PgRow) -> Self {
            Exercise {
                id: row.get::<i32, _>("id"),
                created_at: row.get::<DateTime<Utc>, _>("created_at").to_rfc3339(),
                updated_at: row.get::<DateTime<Utc>, _>("updated_at").to_rfc3339(),
                name: row.get::<String, _>("name"),
                description: row.get::<String, _>("description"),
            }
        }
    }
    impl Queryable for Exercise {
        fn fields() -> Vec<&'static str> {
            vec!["id", "created_at", "updated_at", "name", "description"]
        }
        fn table() -> &'static str {
            "exercises"
        }
        fn query() -> QueryBuilder {
            let mut query = QueryBuilder::new(Exercise::table());
            query.fields(Exercise::fields());
            query
        }
    }
    pub struct ExerciseService {
        pool: PgPool,
    }
    impl ExerciseService {
        pub fn new(pool: &PgPool) -> Self {
            ExerciseService { pool: pool.clone() }
        }
        pub async fn get_exercise_by_id(pool: &PgPool, id: i32, user_id: i32) -> Option<Exercise> {
            let mut query_builder = Exercise::query();
            query_builder . where_raw ("((permissions ->> CAST(${index} as text))::integer > 0 OR (permissions ->> '0')::integer > 0)" , user_id) ;
            query_builder.where_eq("id", id);
            let sql = query_builder.select_query();
            sqlx::query_with(sql.0.as_str(), sql.1)
                .fetch_one(pool)
                .await
                .map(|r| r.into())
                .ok()
        }
        pub async fn return_exercise_by_id(
            &self,
            id: i32,
            user_id: i32,
        ) -> Result<Response<Exercise>, Status> {
            ExerciseService::get_exercise_by_id(&self.pool, id, user_id)
                .await
                .map(|reply| Response::new(reply))
                .ok_or(Status::not_found(format!(
                    "object #{} not found",
                    id.to_string()
                )))
        }
    }
    #[tonic::async_trait]
    impl proto::santa_cruz::exercise_service_server::ExerciseService for ExerciseService {
        async fn get_exercises(
            &self,
            request: Request<GetExercisesRequest>,
        ) -> Result<Response<GetExercisesResponse>, Status> {
            let MeExtension { user_id } = request.extensions().get::<MeExtension>().unwrap();
            let GetExercisesRequest {} = request.get_ref();
            let mut query_builder = Exercise::query();
            query_builder . where_raw ("((permissions ->> CAST(${index} as text))::integer > 0 OR (permissions ->> '0')::integer > 0)" , user_id) ;
            let sql = query_builder.select_query();
            let exercises = sqlx::query_with(sql.0.as_str(), sql.1)
                .fetch_all(&self.pool)
                .await
                .expect("error")
                .into_iter()
                .map(|row| row.into())
                .collect();
            Ok(Response::new(GetExercisesResponse { exercises }))
        }
        async fn get_exercise(
            &self,
            request: Request<GetExerciseRequest>,
        ) -> Result<Response<Exercise>, Status> {
            let MeExtension { user_id } = request.extensions().get::<MeExtension>().unwrap();
            let GetExerciseRequest { id } = request.get_ref();
            self.return_exercise_by_id(*id, *user_id).await
        }
        async fn create_exercise(
            &self,
            request: Request<CreateExerciseRequest>,
        ) -> Result<Response<Exercise>, Status> {
            let MeExtension { user_id } = request.extensions().get::<MeExtension>().unwrap();
            let CreateExerciseRequest { name, description } = request.get_ref();
            let mut query_builder = Exercise::query();
            let mut permissions = HashMap::new();
            permissions.insert(user_id, 2);
            query_builder.field_with_argument("permissions", Json(permissions));
            query_builder.field_with_argument("name", name);
            query_builder.field_with_argument("description", description);
            let sql = query_builder.insert_query();
            let rec = sqlx::query_with(sql.0.as_str(), sql.1)
                .fetch_one(&self.pool)
                .await
                .expect("create error");
            self.return_exercise_by_id(rec.get::<i32, _>("id"), *user_id)
                .await
        }
        async fn update_exercise(
            &self,
            request: Request<UpdateExerciseRequest>,
        ) -> Result<Response<Exercise>, Status> {
            let MeExtension { user_id } = request.extensions().get::<MeExtension>().unwrap();
            let UpdateExerciseRequest {
                id,
                name,
                description,
            } = request.get_ref();
            let original = ExerciseService::get_exercise_by_id(&self.pool, *id, *user_id).await;
            if original.is_none() {
                return Err(Status::not_found(format!(
                    "object #{} not found",
                    id.to_string()
                )));
            }
            let mut query_builder = Exercise::query();
            if let Some(name) = name {
                query_builder.field_with_argument("name", name);
            }
            if let Some(description) = description {
                query_builder.field_with_argument("description", description);
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
            self.return_exercise_by_id(*id, *user_id).await
        }
        async fn delete_exercise(
            &self,
            request: Request<DeleteExerciseRequest>,
        ) -> Result<Response<DeleteExerciseResponse>, Status> {
            let MeExtension { user_id } = &request.extensions().get::<MeExtension>().unwrap();
            let DeleteExerciseRequest { id } = request.get_ref();
            let mut query_builder = Exercise::query();
            query_builder.where_raw(
                "(permissions ->> CAST(${index} as text))::integer > 1",
                user_id,
            );
            query_builder.where_eq("id", id);
            let sql = query_builder.delete_query();
            sqlx::query_with(sql.0.as_str(), sql.1)
                .execute(&self.pool)
                .await
                .expect("delete error");
            Ok(Response::new(DeleteExerciseResponse {}))
        }
    }
}
