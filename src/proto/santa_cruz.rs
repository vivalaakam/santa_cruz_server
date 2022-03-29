#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetExerciseRequest {
    #[prost(int32, tag = "1")]
    pub id: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Exercise {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
}
#[doc = r" Generated server implementations."]
pub mod exercise_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ExerciseServiceServer."]
    #[async_trait]
    pub trait ExerciseService: Send + Sync + 'static {
        async fn get_exercise(
            &self,
            request: tonic::Request<super::GetExerciseRequest>,
        ) -> Result<tonic::Response<super::Exercise>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct ExerciseServiceServer<T: ExerciseService> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ExerciseService> ExerciseServiceServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ExerciseServiceServer<T>
    where
        T: ExerciseService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/santa_cruz.ExerciseService/GetExercise" => {
                    #[allow(non_camel_case_types)]
                    struct GetExerciseSvc<T: ExerciseService>(pub Arc<T>);
                    impl<T: ExerciseService> tonic::server::UnaryService<super::GetExerciseRequest>
                        for GetExerciseSvc<T>
                    {
                        type Response = super::Exercise;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetExerciseRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_exercise(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetExerciseSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: ExerciseService> Clone for ExerciseServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: ExerciseService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ExerciseService> tonic::transport::NamedService for ExerciseServiceServer<T> {
        const NAME: &'static str = "santa_cruz.ExerciseService";
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetWorkoutRequest {
    #[prost(int32, tag = "1")]
    pub id: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetWorkoutsRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetWorkoutsResponse {
    #[prost(message, repeated, tag = "1")]
    pub workouts: ::prost::alloc::vec::Vec<Workout>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateWorkoutRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateWorkoutRequest {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(enumeration = "WorkoutStatus", optional, tag = "2")]
    pub status: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "3")]
    pub day: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub comment: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "7")]
    pub rate: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteWorkoutRequest {
    #[prost(int32, tag = "1")]
    pub id: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteWorkoutResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Workout {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(enumeration = "WorkoutStatus", tag = "2")]
    pub status: i32,
    #[prost(string, tag = "3")]
    pub day: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub created_at: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub updated_at: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub comment: ::prost::alloc::string::String,
    #[prost(int32, tag = "7")]
    pub rate: i32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum WorkoutStatus {
    Unknown = 0,
    InProgress = 1,
    Finished = 2,
}
#[doc = r" Generated server implementations."]
pub mod workout_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with WorkoutServiceServer."]
    #[async_trait]
    pub trait WorkoutService: Send + Sync + 'static {
        async fn get_workout(
            &self,
            request: tonic::Request<super::GetWorkoutRequest>,
        ) -> Result<tonic::Response<super::Workout>, tonic::Status>;
        async fn create_workout(
            &self,
            request: tonic::Request<super::CreateWorkoutRequest>,
        ) -> Result<tonic::Response<super::Workout>, tonic::Status>;
        async fn update_workout(
            &self,
            request: tonic::Request<super::UpdateWorkoutRequest>,
        ) -> Result<tonic::Response<super::Workout>, tonic::Status>;
        async fn delete_workout(
            &self,
            request: tonic::Request<super::DeleteWorkoutRequest>,
        ) -> Result<tonic::Response<super::DeleteWorkoutResponse>, tonic::Status>;
        async fn get_workouts(
            &self,
            request: tonic::Request<super::GetWorkoutsRequest>,
        ) -> Result<tonic::Response<super::GetWorkoutsResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct WorkoutServiceServer<T: WorkoutService> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: WorkoutService> WorkoutServiceServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for WorkoutServiceServer<T>
    where
        T: WorkoutService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/santa_cruz.WorkoutService/GetWorkout" => {
                    #[allow(non_camel_case_types)]
                    struct GetWorkoutSvc<T: WorkoutService>(pub Arc<T>);
                    impl<T: WorkoutService> tonic::server::UnaryService<super::GetWorkoutRequest> for GetWorkoutSvc<T> {
                        type Response = super::Workout;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetWorkoutRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_workout(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetWorkoutSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/santa_cruz.WorkoutService/CreateWorkout" => {
                    #[allow(non_camel_case_types)]
                    struct CreateWorkoutSvc<T: WorkoutService>(pub Arc<T>);
                    impl<T: WorkoutService> tonic::server::UnaryService<super::CreateWorkoutRequest>
                        for CreateWorkoutSvc<T>
                    {
                        type Response = super::Workout;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateWorkoutRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_workout(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateWorkoutSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/santa_cruz.WorkoutService/UpdateWorkout" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateWorkoutSvc<T: WorkoutService>(pub Arc<T>);
                    impl<T: WorkoutService> tonic::server::UnaryService<super::UpdateWorkoutRequest>
                        for UpdateWorkoutSvc<T>
                    {
                        type Response = super::Workout;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateWorkoutRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).update_workout(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateWorkoutSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/santa_cruz.WorkoutService/DeleteWorkout" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteWorkoutSvc<T: WorkoutService>(pub Arc<T>);
                    impl<T: WorkoutService> tonic::server::UnaryService<super::DeleteWorkoutRequest>
                        for DeleteWorkoutSvc<T>
                    {
                        type Response = super::DeleteWorkoutResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteWorkoutRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete_workout(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteWorkoutSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/santa_cruz.WorkoutService/GetWorkouts" => {
                    #[allow(non_camel_case_types)]
                    struct GetWorkoutsSvc<T: WorkoutService>(pub Arc<T>);
                    impl<T: WorkoutService> tonic::server::UnaryService<super::GetWorkoutsRequest>
                        for GetWorkoutsSvc<T>
                    {
                        type Response = super::GetWorkoutsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetWorkoutsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_workouts(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetWorkoutsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: WorkoutService> Clone for WorkoutServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: WorkoutService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: WorkoutService> tonic::transport::NamedService for WorkoutServiceServer<T> {
        const NAME: &'static str = "santa_cruz.WorkoutService";
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetWorkoutSetRequest {
    #[prost(int32, tag = "1")]
    pub id: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetWorkoutSetsRequest {
    #[prost(int32, tag = "1")]
    pub workout_id: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetWorkoutSetsResponse {
    #[prost(message, repeated, tag = "1")]
    pub workout_sets: ::prost::alloc::vec::Vec<WorkoutSet>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateWorkoutSetRequest {
    #[prost(int32, tag = "1")]
    pub workout_id: i32,
    #[prost(int32, tag = "2")]
    pub position: i32,
    #[prost(message, optional, tag = "3")]
    pub r#type: ::core::option::Option<WorkoutSetType>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateWorkoutSetRequest {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(string, optional, tag = "2")]
    pub comment: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "3")]
    pub position: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "4")]
    pub r#type: ::core::option::Option<WorkoutSetType>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteWorkoutSetRequest {
    #[prost(int32, tag = "1")]
    pub id: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteWorkoutSetResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkoutSetType {
    #[prost(oneof = "workout_set_type::Type", tags = "1, 2, 3")]
    pub r#type: ::core::option::Option<workout_set_type::Type>,
}
/// Nested message and enum types in `WorkoutSetType`.
pub mod workout_set_type {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Unknown {}
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Circle {}
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Exercise {
        #[prost(int32, tag = "1")]
        pub exercise_id: i32,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        #[prost(message, tag = "1")]
        Unknown(Unknown),
        #[prost(message, tag = "2")]
        Circle(Circle),
        #[prost(message, tag = "3")]
        Exercise(Exercise),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkoutSet {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(int32, tag = "2")]
    pub workout_id: i32,
    #[prost(int32, tag = "3")]
    pub position: i32,
    #[prost(message, optional, tag = "4")]
    pub r#type: ::core::option::Option<WorkoutSetType>,
    #[prost(string, tag = "5")]
    pub comment: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub created_at: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub updated_at: ::prost::alloc::string::String,
}
#[doc = r" Generated server implementations."]
pub mod workout_set_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with WorkoutSetServiceServer."]
    #[async_trait]
    pub trait WorkoutSetService: Send + Sync + 'static {
        async fn get_workout_set(
            &self,
            request: tonic::Request<super::GetWorkoutSetRequest>,
        ) -> Result<tonic::Response<super::WorkoutSet>, tonic::Status>;
        async fn create_workout_set(
            &self,
            request: tonic::Request<super::CreateWorkoutSetRequest>,
        ) -> Result<tonic::Response<super::WorkoutSet>, tonic::Status>;
        async fn update_workout_set(
            &self,
            request: tonic::Request<super::UpdateWorkoutSetRequest>,
        ) -> Result<tonic::Response<super::WorkoutSet>, tonic::Status>;
        async fn delete_workout_set(
            &self,
            request: tonic::Request<super::DeleteWorkoutSetRequest>,
        ) -> Result<tonic::Response<super::DeleteWorkoutSetResponse>, tonic::Status>;
        async fn get_workout_sets(
            &self,
            request: tonic::Request<super::GetWorkoutSetsRequest>,
        ) -> Result<tonic::Response<super::GetWorkoutSetsResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct WorkoutSetServiceServer<T: WorkoutSetService> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: WorkoutSetService> WorkoutSetServiceServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for WorkoutSetServiceServer<T>
    where
        T: WorkoutSetService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/santa_cruz.WorkoutSetService/GetWorkoutSet" => {
                    #[allow(non_camel_case_types)]
                    struct GetWorkoutSetSvc<T: WorkoutSetService>(pub Arc<T>);
                    impl<T: WorkoutSetService>
                        tonic::server::UnaryService<super::GetWorkoutSetRequest>
                        for GetWorkoutSetSvc<T>
                    {
                        type Response = super::WorkoutSet;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetWorkoutSetRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_workout_set(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetWorkoutSetSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/santa_cruz.WorkoutSetService/CreateWorkoutSet" => {
                    #[allow(non_camel_case_types)]
                    struct CreateWorkoutSetSvc<T: WorkoutSetService>(pub Arc<T>);
                    impl<T: WorkoutSetService>
                        tonic::server::UnaryService<super::CreateWorkoutSetRequest>
                        for CreateWorkoutSetSvc<T>
                    {
                        type Response = super::WorkoutSet;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateWorkoutSetRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_workout_set(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateWorkoutSetSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/santa_cruz.WorkoutSetService/UpdateWorkoutSet" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateWorkoutSetSvc<T: WorkoutSetService>(pub Arc<T>);
                    impl<T: WorkoutSetService>
                        tonic::server::UnaryService<super::UpdateWorkoutSetRequest>
                        for UpdateWorkoutSetSvc<T>
                    {
                        type Response = super::WorkoutSet;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateWorkoutSetRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).update_workout_set(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateWorkoutSetSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/santa_cruz.WorkoutSetService/DeleteWorkoutSet" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteWorkoutSetSvc<T: WorkoutSetService>(pub Arc<T>);
                    impl<T: WorkoutSetService>
                        tonic::server::UnaryService<super::DeleteWorkoutSetRequest>
                        for DeleteWorkoutSetSvc<T>
                    {
                        type Response = super::DeleteWorkoutSetResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteWorkoutSetRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete_workout_set(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteWorkoutSetSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/santa_cruz.WorkoutSetService/GetWorkoutSets" => {
                    #[allow(non_camel_case_types)]
                    struct GetWorkoutSetsSvc<T: WorkoutSetService>(pub Arc<T>);
                    impl<T: WorkoutSetService>
                        tonic::server::UnaryService<super::GetWorkoutSetsRequest>
                        for GetWorkoutSetsSvc<T>
                    {
                        type Response = super::GetWorkoutSetsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetWorkoutSetsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_workout_sets(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetWorkoutSetsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: WorkoutSetService> Clone for WorkoutSetServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: WorkoutSetService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: WorkoutSetService> tonic::transport::NamedService for WorkoutSetServiceServer<T> {
        const NAME: &'static str = "santa_cruz.WorkoutSetService";
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkoutRepeat {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(int32, tag = "2")]
    pub workout_set_id: i32,
    #[prost(int32, tag = "3")]
    pub exercise_id: i32,
    #[prost(int32, tag = "4")]
    pub repeats: i32,
    #[prost(double, optional, tag = "5")]
    pub weight: ::core::option::Option<f64>,
}
