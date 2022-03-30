use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use sqlx::{postgres::PgArguments, Arguments, PgPool};
use tonic::{Request, Response, Status};

use crate::me_extension::MeExtension;
use crate::proto::proto;
use crate::proto::proto::santa_cruz;
use crate::proto::proto::santa_cruz::workout_set_type::Type;
use crate::proto::proto::santa_cruz::{
    CreateWorkoutSetRequest, DeleteWorkoutSetRequest, DeleteWorkoutSetResponse,
    GetWorkoutSetRequest, GetWorkoutSetsRequest, GetWorkoutSetsResponse, UpdateWorkoutSetRequest,
    WorkoutSet,
};
use crate::WorkoutService;

pub struct WorkoutSetService {
    pool: PgPool,
}

type WorkoutSetRow = (
    i32,
    i32,
    i32,
    Json<WorkoutSetType>,
    String,
    DateTime<Utc>,
    DateTime<Utc>,
);

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WorkoutSetType {
    Unknown {},
    Circle {},
    Exercise { exercise_id: i32 },
}

impl Into<santa_cruz::WorkoutSetType> for Json<WorkoutSetType> {
    fn into(self) -> santa_cruz::WorkoutSetType {
        let value = match self.0 {
            WorkoutSetType::Circle { .. } => {
                santa_cruz::workout_set_type::Type::Circle(santa_cruz::workout_set_type::Circle {})
            }
            WorkoutSetType::Exercise { exercise_id } => {
                santa_cruz::workout_set_type::Type::Exercise(
                    santa_cruz::workout_set_type::Exercise { exercise_id },
                )
            }
            WorkoutSetType::Unknown {} => santa_cruz::workout_set_type::Type::Unknown(
                santa_cruz::workout_set_type::Unknown {},
            ),
        };

        santa_cruz::WorkoutSetType {
            r#type: Some(value),
        }
    }
}

impl From<santa_cruz::WorkoutSetType> for Json<WorkoutSetType> {
    fn from(data: santa_cruz::WorkoutSetType) -> Self {
        let json = match data.r#type {
            None => WorkoutSetType::Unknown {},
            Some(value) => match value {
                Type::Circle(_) => WorkoutSetType::Circle {},
                Type::Exercise(exercise) => WorkoutSetType::Exercise {
                    exercise_id: exercise.exercise_id,
                },
                Type::Unknown(_) => WorkoutSetType::Unknown {},
            },
        };

        Json(json)
    }
}

impl Into<WorkoutSet> for WorkoutSetRow {
    fn into(self) -> WorkoutSet {
        WorkoutSet {
            id: self.0,
            workout_id: self.1,
            position: self.2,
            r#type: Some(self.3.into()),
            comment: self.4,
            created_at: self.5.to_rfc3339(),
            updated_at: self.6.to_rfc3339(),
        }
    }
}

impl WorkoutSetService {
    pub fn new(pool: &PgPool) -> WorkoutSetService {
        WorkoutSetService { pool: pool.clone() }
    }

    pub async fn get_workout_set_by_id(pool: &PgPool, id: i32, user_id: i32) -> Option<WorkoutSet> {
        sqlx::query_as::<_, WorkoutSetRow>(
            r#"
                    SELECT id, workout_id, position, type, comment, created_at, updated_at
                    FROM workout_sets
                    WHERE id = $1 AND ((permissions ->> CAST($2 as text))::integer > 0 OR (permissions ->> '0')::integer > 0)
                "#,
        )
            .bind(id)
            .bind(user_id)
            .fetch_one(pool)
            .await
            .map(|row| row.into())
            .ok()
    }

    pub async fn return_workout_set_by_id(
        &self,
        id: i32,
        user_id: i32,
    ) -> Result<Response<WorkoutSet>, Status> {
        WorkoutSetService::get_workout_set_by_id(&self.pool, id, user_id)
            .await
            .map(|reply| Response::new(reply))
            .ok_or(Status::not_found(format!(
                "workout_set #{} not found",
                id.to_string()
            )))
    }
}

#[tonic::async_trait]
impl proto::santa_cruz::workout_set_service_server::WorkoutSetService for WorkoutSetService {
    async fn get_workout_set(
        &self,
        request: Request<GetWorkoutSetRequest>,
    ) -> Result<Response<WorkoutSet>, Status> {
        let MeExtension { user_id } = request.extensions().get::<MeExtension>().unwrap();
        let GetWorkoutSetRequest { id } = request.get_ref();

        self.return_workout_set_by_id(*id, *user_id).await
    }

    async fn create_workout_set(
        &self,
        request: Request<CreateWorkoutSetRequest>,
    ) -> Result<Response<WorkoutSet>, Status> {
        let MeExtension { user_id } = &request.extensions().get::<MeExtension>().unwrap();

        let CreateWorkoutSetRequest {
            workout_id,
            position,
            r#type,
        } = &request.get_ref();

        let workout = WorkoutService::get_workout_by_id(&self.pool, *workout_id, *user_id).await;

        if workout.is_none() {
            return Err(Status::permission_denied(format!(
                "permissions not found for workout #{}",
                workout_id.to_string()
            )));
        }

        let mut permissions = HashMap::new();
        permissions.insert(user_id, 2);

        let (id, ): (i32, ) = sqlx::query_as(
            r#"INSERT INTO workout_sets ( workout_id, position, type, permissions ) VALUES ( $1 , $2, $3, $4 ) RETURNING id"#,
        )
            .bind(workout_id)
            .bind(position)
            .bind(Json::from(r#type.clone().unwrap()))
            .bind(Json(permissions))
            .fetch_one(&self.pool)
            .await
            .expect("create_workout_set error");

        self.return_workout_set_by_id(id, *user_id).await
    }

    async fn update_workout_set(
        &self,
        request: Request<UpdateWorkoutSetRequest>,
    ) -> Result<Response<WorkoutSet>, Status> {
        let MeExtension { user_id } = &request.extensions().get::<MeExtension>().unwrap();
        let UpdateWorkoutSetRequest {
            id,
            comment,
            position,
            r#type,
        } = &request.get_ref();

        let original = WorkoutSetService::get_workout_set_by_id(&self.pool, *id, *user_id).await;

        if original.is_none() {
            return Err(Status::not_found(format!(
                "workout #{} not found",
                id.to_string()
            )));
        }

        let mut arguments = PgArguments::default();

        let mut params = vec![];

        if let Some(position) = position {
            params.push("position");
            arguments.add(position)
        }

        if let Some(r#type) = r#type.clone() {
            params.push("type");
            arguments.add(Json::from(r#type));
        }

        if let Some(comment) = comment {
            params.push("comment");
            arguments.add(comment);
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
            "UPDATE workout_sets SET {fields} WHERE id = ${index}",
            fields = fields.join(", "),
            index = fields.len() + 1
        );

        arguments.add(id);

        sqlx::query_with(&*query, arguments)
            .execute(&self.pool)
            .await
            .expect("update_workout_set error");

        self.return_workout_set_by_id(*id, *user_id).await
    }

    async fn delete_workout_set(
        &self,
        request: Request<DeleteWorkoutSetRequest>,
    ) -> Result<Response<DeleteWorkoutSetResponse>, Status> {
        let MeExtension { user_id } = request.extensions().get::<MeExtension>().unwrap();
        let DeleteWorkoutSetRequest { id } = &request.get_ref();

        sqlx::query(r#"DELETE FROM workout_sets WHERE id = $1 AND (permissions ->> CAST($2 as text))::integer > 1"#)
            .bind(id)
            .bind(user_id)
            .execute(&self.pool)
            .await
            .expect("update_workout_set error");

        Ok(Response::new(DeleteWorkoutSetResponse {}))
    }

    async fn get_workout_sets(
        &self,
        request: Request<GetWorkoutSetsRequest>,
    ) -> Result<Response<GetWorkoutSetsResponse>, Status> {
        let MeExtension { user_id } = request.extensions().get::<MeExtension>().unwrap();
        let GetWorkoutSetsRequest { workout_id } = &request.get_ref();

        let rows = sqlx::query_as::<_, WorkoutSetRow>(
            r#"SELECT id, workout_id, position, type, comment, created_at, updated_at FROM workout_sets WHERE workout_id = $1 AND ((permissions ->> CAST($2 as text))::integer > 0 OR (permissions ->> '0')::integer > 0)"#,
        )
            .bind(workout_id)
            .bind(user_id)
            .fetch_all(&self.pool)
            .await
            .expect("get_workout_sets error");

        let workout_sets = rows.into_iter().map(|row| row.into()).collect();

        Ok(Response::new(GetWorkoutSetsResponse { workout_sets }))
    }
}
