use std::collections::HashMap;

use chrono::{DateTime, Utc};
use sqlx::types::Json;
use sqlx::{postgres::PgArguments, Arguments, PgPool};
use tonic::{Request, Response, Status};

use crate::me_extension::MeExtension;
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

    pub async fn get_workout_by_id(pool: &PgPool, id: i32, user_id: i32) -> Option<Workout> {
        sqlx::query_as::<_, WorkoutRow>(
            r#"
                    SELECT id, status, day, created_at, updated_at, rate, comment
                    FROM workouts
                    WHERE id = $1 AND ((permissions ->> CAST($2 as text))::integer > 0 OR (permissions ->> '0')::integer > 0)
                "#,
        )
            .bind(id)
            .bind(user_id)
            .fetch_one(pool)
            .await
            .map(|r| r.into())
            .ok()
    }
}

#[tonic::async_trait]
impl proto::santa_cruz::workout_service_server::WorkoutService for WorkoutService {
    async fn get_workout(
        &self,
        request: Request<GetWorkoutRequest>,
    ) -> Result<Response<Workout>, Status> {
        let MeExtension { user_id } = &request.extensions().get::<MeExtension>().unwrap();
        let GetWorkoutRequest { id } = &request.get_ref();

        WorkoutService::get_workout_by_id(&self.pool, *id, *user_id)
            .await
            .map(|reply| Response::new(reply))
            .ok_or(Status::not_found(format!(
                "workout #{} not found",
                id.to_string()
            )))
    }

    async fn create_workout(
        &self,
        request: Request<CreateWorkoutRequest>,
    ) -> Result<Response<Workout>, Status> {
        let MeExtension { user_id } = &request.extensions().get::<MeExtension>().unwrap();

        let mut permissions = HashMap::new();
        permissions.insert(user_id, 2);

        let rec: (i32, ) = sqlx::query_as(
            r#"INSERT INTO workouts ( status, day, permissions ) VALUES ( $1 , $2, $3 ) RETURNING id"#,
        )
            .bind(0)
            .bind(Utc::now())
            .bind(Json(permissions))
            .fetch_one(&self.pool)
            .await
            .expect("create_workout error");

        match WorkoutService::get_workout_by_id(&self.pool, rec.0 as i32, *user_id).await {
            Some(reply) => Ok(Response::new(reply)),
            None => Err(Status::not_found(format!(
                "workout #{} not found",
                rec.0.to_string()
            ))),
        }
    }

    async fn update_workout(
        &self,
        request: Request<UpdateWorkoutRequest>,
    ) -> Result<Response<Workout>, Status> {
        let MeExtension { user_id } = &request.extensions().get::<MeExtension>().unwrap();
        let UpdateWorkoutRequest {
            id,
            status,
            day,
            rate,
            comment,
        } = request.get_ref();
        let original = WorkoutService::get_workout_by_id(&self.pool, *id, *user_id).await;

        if original.is_none() {
            return Err(Status::not_found(format!(
                "workout #{} not found",
                id.to_string()
            )));
        }

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
            return original
                .map(|reply| Response::new(reply))
                .ok_or(Status::not_found(format!(
                    "workout #{} not found",
                    id.to_string()
                )));
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

        match WorkoutService::get_workout_by_id(&self.pool, *id, *user_id).await {
            Some(reply) => Ok(Response::new(reply)),
            None => Err(Status::not_found(format!(
                "workout #{} not found",
                id.to_string()
            ))),
        }
    }

    async fn delete_workout(
        &self,
        request: Request<DeleteWorkoutRequest>,
    ) -> Result<Response<DeleteWorkoutResponse>, Status> {
        let MeExtension { user_id } = request.extensions().get::<MeExtension>().unwrap();
        let DeleteWorkoutRequest { id } = request.get_ref();

        sqlx::query(r#"DELETE FROM workouts WHERE id = $1 AND (permissions ->> CAST($2 as text))::integer > 1"#)
            .bind(id)
            .bind(user_id)
            .execute(&self.pool)
            .await
            .expect("update_workout error");

        Ok(Response::new(DeleteWorkoutResponse {}))
    }

    async fn get_workouts(
        &self,
        request: Request<GetWorkoutsRequest>,
    ) -> Result<Response<GetWorkoutsResponse>, Status> {
        let MeExtension { user_id } = &request.extensions().get::<MeExtension>().unwrap();

        let rows: Vec<WorkoutRow> = sqlx::query_as(
            r#"SELECT id, status, day, created_at, updated_at, rate, comment
                    FROM workouts
                    WHERE ((permissions ->> CAST($1 as text))::integer > 0 OR (permissions ->> '0')::integer > 0)
                    ORDER BY created_at DESC
                "#,
        )
            .bind(user_id)
            .fetch_all(&self.pool)
            .await
            .expect("get_workout_by_id error");

        let workouts = rows.into_iter().map(|row| row.into()).collect();

        Ok(Response::new(GetWorkoutsResponse { workouts }))
    }
}
