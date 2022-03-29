use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use sqlx::{postgres::PgArguments, Arguments, PgPool};
use tonic::{Request, Response, Status};

use crate::proto::proto;
use crate::proto::proto::santa_cruz;
use crate::proto::proto::santa_cruz::workout_set_type::Type;
use crate::proto::proto::santa_cruz::{
    CreateWorkoutSetRequest, DeleteWorkoutSetRequest, DeleteWorkoutSetResponse,
    GetWorkoutSetRequest, GetWorkoutSetsRequest, GetWorkoutSetsResponse, UpdateWorkoutSetRequest,
    WorkoutSet,
};

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

    pub async fn get_workout_set_by_id(&self, id: i32) -> WorkoutSet {
        let row: WorkoutSetRow = sqlx::query_as(
            r#"SELECT id, workout_id, position, type, comment, created_at, updated_at FROM workout_sets WHERE id = $1"#,
        )
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .expect("get_workout_set_by_id error");

        row.into()
    }
}

#[tonic::async_trait]
impl proto::santa_cruz::workout_set_service_server::WorkoutSetService for WorkoutSetService {
    async fn get_workout_set(
        &self,
        request: Request<GetWorkoutSetRequest>,
    ) -> Result<Response<WorkoutSet>, Status> {
        let GetWorkoutSetRequest { id } = &request.into_inner();

        let reply = self.get_workout_set_by_id(*id).await;
        Ok(Response::new(reply))
    }

    async fn create_workout_set(
        &self,
        request: Request<CreateWorkoutSetRequest>,
    ) -> Result<Response<WorkoutSet>, Status> {
        let CreateWorkoutSetRequest {
            workout_id,
            position,
            r#type,
        } = &request.into_inner();

        let rec: (i32, ) = sqlx::query_as(
            r#"INSERT INTO workout_sets ( workout_id, position, type ) VALUES ( $1 , $2, $3 ) RETURNING id"#,
        )
            .bind(workout_id)
            .bind(position)
            .bind(Json::from(r#type.clone().unwrap()))
            .fetch_one(&self.pool)
            .await
            .expect("create_workout_set error");

        let reply = self.get_workout_set_by_id(rec.0 as i32).await;
        Ok(Response::new(reply))
    }

    async fn update_workout_set(
        &self,
        request: Request<UpdateWorkoutSetRequest>,
    ) -> Result<Response<WorkoutSet>, Status> {
        let UpdateWorkoutSetRequest {
            id,
            comment,
            position,
            r#type,
        } = &request.into_inner();
        let original = self.get_workout_set_by_id(*id).await;

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
            "UPDATE workout_sets SET {fields} WHERE id = ${index}",
            fields = fields.join(", "),
            index = params.len() + 1
        );

        arguments.add(id);

        sqlx::query_with(&*query, arguments)
            .execute(&self.pool)
            .await
            .expect("update_workout_set error");

        let reply = self.get_workout_set_by_id(*id).await;
        Ok(Response::new(reply))
    }

    async fn delete_workout_set(
        &self,
        request: Request<DeleteWorkoutSetRequest>,
    ) -> Result<Response<DeleteWorkoutSetResponse>, Status> {
        let DeleteWorkoutSetRequest { id } = &request.into_inner();

        sqlx::query(r#"DELETE FROM workout_sets WHERE id = $1 "#)
            .bind(id)
            .execute(&self.pool)
            .await
            .expect("update_workout_set error");

        Ok(Response::new(DeleteWorkoutSetResponse {}))
    }

    async fn get_workout_sets(
        &self,
        request: Request<GetWorkoutSetsRequest>,
    ) -> Result<Response<GetWorkoutSetsResponse>, Status> {
        let GetWorkoutSetsRequest { workout_id } = &request.into_inner();

        let rows: Vec<WorkoutSetRow> = sqlx::query_as(
            r#"SELECT id, workout_id, position, type, comment, created_at, updated_at FROM workout_sets WHERE workout_id = $1"#,
        )
            .bind(workout_id)
            .fetch_all(&self.pool)
            .await
            .expect("get_workout_sets error");

        let workout_sets = rows.into_iter().map(|row| row.into()).collect();

        Ok(Response::new(GetWorkoutSetsResponse { workout_sets }))
    }
}
