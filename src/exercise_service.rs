use chrono::{DateTime, Utc};
use sqlx::{postgres::PgArguments, Arguments, PgPool};
use tonic::{Request, Response, Status};

use crate::proto::proto;
use crate::proto::proto::santa_cruz::{
    CreateExerciseRequest, DeleteExerciseRequest, DeleteExerciseResponse, Exercise,
    GetExerciseRequest, GetExercisesRequest, GetExercisesResponse, UpdateExerciseRequest,
};

pub struct ExerciseService {
    pool: PgPool,
}

type ExerciseRow = (i32, DateTime<Utc>, DateTime<Utc>, String, String);

impl Into<Exercise> for ExerciseRow {
    fn into(self) -> Exercise {
        Exercise {
            id: self.0,
            created_at: self.1.to_rfc3339(),
            updated_at: self.2.to_rfc3339(),
            name: self.3,
            description: self.4,
        }
    }
}

impl ExerciseService {
    pub fn new(pool: &PgPool) -> ExerciseService {
        ExerciseService { pool: pool.clone() }
    }

    pub async fn get_exercise_by_id(&self, id: i32) -> Exercise {
        let row: ExerciseRow = sqlx::query_as(
            r#"SELECT id, created_at, updated_at, name, description FROM exercises WHERE id = $1"#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .expect("get_exercise_by_id error");

        row.into()
    }
}

#[tonic::async_trait]
impl proto::santa_cruz::exercise_service_server::ExerciseService for ExerciseService {
    async fn get_exercise(
        &self,
        request: Request<GetExerciseRequest>,
    ) -> Result<Response<Exercise>, Status> {
        let GetExerciseRequest { id } = &request.into_inner();

        let reply = self.get_exercise_by_id(*id).await;
        Ok(Response::new(reply))
    }

    async fn create_exercise(
        &self,
        request: Request<CreateExerciseRequest>,
    ) -> Result<Response<Exercise>, Status> {
        let CreateExerciseRequest { name, description } = &request.into_inner();

        let rec: (i32,) = sqlx::query_as(
            r#"INSERT INTO exercises ( name, description ) VALUES ( $1 , $2 ) RETURNING id"#,
        )
        .bind(name)
        .bind(description)
        .fetch_one(&self.pool)
        .await
        .expect("create_exercise error");

        let reply = self.get_exercise_by_id(rec.0 as i32).await;
        Ok(Response::new(reply))
    }

    async fn update_exercise(
        &self,
        request: Request<UpdateExerciseRequest>,
    ) -> Result<Response<Exercise>, Status> {
        let UpdateExerciseRequest {
            id,
            name,
            description,
        } = &request.into_inner();
        let original = self.get_exercise_by_id(*id).await;

        let mut arguments = PgArguments::default();

        let mut params = vec![];

        if let Some(name) = name {
            params.push("name");
            arguments.add(name)
        }

        if let Some(description) = description {
            params.push("description");
            arguments.add(description)
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
            "UPDATE exercises SET {fields} WHERE id = ${index}",
            fields = fields.join(", "),
            index = params.len() + 1
        );

        arguments.add(id);

        sqlx::query_with(&*query, arguments)
            .execute(&self.pool)
            .await
            .expect("update_exercise error");

        let reply = self.get_exercise_by_id(*id).await;
        Ok(Response::new(reply))
    }

    async fn delete_exercise(
        &self,
        request: Request<DeleteExerciseRequest>,
    ) -> Result<Response<DeleteExerciseResponse>, Status> {
        let DeleteExerciseRequest { id } = &request.into_inner();

        sqlx::query(r#"DELETE FROM exercises WHERE id = $1 "#)
            .bind(id)
            .execute(&self.pool)
            .await
            .expect("update_exercise error");

        Ok(Response::new(DeleteExerciseResponse {}))
    }

    async fn get_exercises(
        &self,
        _request: Request<GetExercisesRequest>,
    ) -> Result<Response<GetExercisesResponse>, Status> {
        let rows: Vec<ExerciseRow> = sqlx::query_as(
            r#"SELECT id, created_at, updated_at, name, description FROM exercises"#,
        )
        .fetch_all(&self.pool)
        .await
        .expect("get_exercise_by_id error");

        let exercises = rows.into_iter().map(|row| row.into()).collect();

        Ok(Response::new(GetExercisesResponse { exercises }))
    }
}
