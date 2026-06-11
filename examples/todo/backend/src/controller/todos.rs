use axum::Router;
use groom_macros::Controller;
use utoipa::openapi::OpenApiBuilder;

/// Sets up HTTP router.
pub fn setup_router(router: Router) -> Router {
    controller::merge_into_router(router)
}

/// Sets up OpenAPI definitions.
pub fn setup_spec(spec_builder: OpenApiBuilder) -> OpenApiBuilder {
    controller::merge_into_openapi_builder(spec_builder)
}

#[Controller()]
mod controller {
    use std::sync::Arc;

    use axum::{Extension, extract::{Path}, response::IntoResponse};
    use axum_extra::extract::Query;

    use groom::{
        // GroomExtractor is the trait that enables types to describe themselves into openapi spec.
        //
        // Importing it fixes this error: 
        //    "required methods no associated item named '__openapi_modify_operation' found 
        //     for struct `axum::extract::Query<GreetParams>` in the current scope".
        extract::GroomExtractor,

        // Response is the trait that enables enums and structs to turn themselves into HTTP responses
        // and into openapi spec.
        response::Response
    };
    
    use groom_macros::{
        // DTO macro generates implementations for DTOs.
        DTO, 

        // RequestBody macro generates implementations for structs to act as response bodies.
        RequestBody,

        // Response macro generates implementations for enums and structs as responses.
        Response
    };

    use crate::service::{
        error::StorageError,
        model::{self, Status, TaskID},
        task_service::{self, TaskService},
    };

    use super::model::{SortOrder, Task, TaskSortField};

    // region: list tasks
    //

    /// Lists tasks
    #[Route(method="get", path="/tasks")]
    pub async fn list_tasks(
        Extension(task_service): Extension<Arc<TaskService>>,
        Query(req): Query<ListTasksRequest>
    ) -> Result<ListTasksOk, ListTasksError> {
        match task_service.list_tasks(req.into()).await {
            Ok(l) => {
                let tasks: Result<Vec<Task>, ()> = l.iter()
                    .map(|t| Task::try_from(t))
                    .collect()
                ;

                match tasks {
                    Ok(v)  => Ok(ListTasksOk(TasksList(v))),
                    Err(_) => Err(ListTasksError::ServerError),
                }
            },
            Err(task_service::ListTasksError::StorageError(err)) => {
                log_storage_error(&err, "storage error when listing tasks");
                Err(ListTasksError::ServerError)
            },
        }
    }

    /// Query parameters for listing tasks.
    #[DTO(parameters)]
    #[derive(Debug)]
    pub struct ListTasksRequest {
        pub title:  Option<String>,
        pub status: Option<Vec<Status>>,

        #[serde(default)]
        pub sort_by: TaskSortField,

        #[serde(default)]
        pub order:   SortOrder,
    }

    impl From<ListTasksRequest> for task_service::ListTasksRequest {
        fn from(req: ListTasksRequest) -> Self {
            task_service::ListTasksRequest {
                title:   req.title,
                status:  req.status,
                sort_by: req.sort_by.into(),
                order:   req.order.into(),
            }
        }
    }

    #[Response(format(json), code = 200)]
    pub struct ListTasksOk(TasksList);

    #[Response(format(json))]
    pub enum ListTasksError {
        #[Response(code = 500)]
        ServerError,
    }

    #[DTO(response)]
    pub struct TasksList(Vec<Task>);

    //
    // endregion: list tasks

    // region: get task
    //

    /// Gets a single task.
    #[Route(method="get", path="/tasks/{task_id}")]
    pub async fn get_task(
        Extension(task_service): Extension<Arc<TaskService>>,
        Path(path): Path<TaskId>
    ) -> Result<Task, GetTaskError> {
        match task_service.get_task_by_id(TaskID::from(path.task_id)).await {
            Ok(maybe) => match maybe {
                None => Err(GetTaskError::NotFound),
                Some(t) => 
                    match Task::try_from(&t) {
                        Ok(v)  => Ok(v),
                        Err(_) => Err(GetTaskError::ServerError),
                    },
            },
            Err(task_service::GetTaskError::StorageError(err)) => {
                log_storage_error(&err, "storage error when getting task");
                Err(GetTaskError::ServerError)
            },
        }
    }

    /// Path parameter identifying a task.
    #[DTO(parameters)]
    pub struct TaskId {
        pub task_id: u64,
    }

    #[Response(format(json))]
    pub enum GetTaskError {
        #[Response(code = 404)]
        NotFound,

        #[Response(code = 500)]
        ServerError,
    }

    //
    // endregion: get task

    // region: add task
    //

    /// Adds a new task.
    #[Route(method="post", path="/tasks")]
    pub async fn add_task(
        Extension(task_service): Extension<Arc<TaskService>>,
        req: AddTaskRequest
    ) -> Result<AddTaskOk, AddTaskError> {
        let req = task_service::AddTaskRequest {
            title: req.title,
        };

        match task_service.add_task(req).await {
            Ok(t) => match Task::try_from(&t) {
                Ok(v)  => Ok(AddTaskOk(v)),
                Err(_) => Err(AddTaskError::ServerError),
            },

            Err(e) => match e {
                task_service::AddTaskError::Duplicate =>
                    Err(AddTaskError::AlreadyExists),

                task_service::AddTaskError::InvalidRequest(reason) => 
                    Err(AddTaskError::MalformedRequest(reason.into())),

                task_service::AddTaskError::StorageError(err) => {
                    log_storage_error(&err, "storage error when adding task");
                    Err(AddTaskError::ServerError)
                },
            },
        }
    }

    /// Request body
    #[RequestBody(format(json, url_encoded))]
    pub struct AddTaskRequest {
        pub title: String,
    }

    /// Task added successfully
    #[Response(format(json), code = 201)]
    pub struct AddTaskOk(Task);

    #[Response(format(json))]
    pub enum AddTaskError {
        /// Task already exists with the same title
        #[Response(code = 409)]
        AlreadyExists,

        /// Malformed request, e.g. missing title or title is too long.
        #[Response(code = 400)]
        MalformedRequest(String),

        /// Unexpected error when adding a task, e.g. database is down or serialization error.
        #[Response(code = 500)]
        ServerError,
    }

    //
    // endregion: add task

    // region: rename task
    //

    /// Renames a task.
    #[Route(method="put", path="/tasks/{task_id}/name")]
    pub async fn rename_task(
        Extension(task_service): Extension<Arc<TaskService>>,
        Path(task_id): Path<TaskId>,
        req: RenameTaskRequest
    ) -> Result<Task, RenameTaskError> {
        match task_service.rename_task(TaskID::from(task_id.task_id), req.title).await {
            Ok(t) => match Task::try_from(&t) {
                Ok(v)  => Ok(v),
                Err(_) => Err(RenameTaskError::ServerError),
            },

            Err(e) => Err(match e {
                task_service::RenameTaskError::InvalidRequest(d) =>
                    RenameTaskError::MalformedRequest(d.into()),

                task_service::RenameTaskError::NotFound => 
                    RenameTaskError::NotFound,

                task_service::RenameTaskError::Duplicate => 
                    RenameTaskError::AlreadyExists,

                task_service::RenameTaskError::StorageReadError(err)
                | task_service::RenameTaskError::StorageWriteError(err) => {
                    log_storage_error(&err, "storage error when renaming task");
                    RenameTaskError::ServerError
                },
            }),
        }
    }

    /// Request body
    #[RequestBody(format(json, url_encoded))]
    pub struct RenameTaskRequest {
        pub title: String,
    }

    #[Response(format(json))]
    pub enum RenameTaskError {
        #[Response(code = 404)]
        NotFound,

        #[Response(code = 400)]
        MalformedRequest(String),

        #[Response(code = 409)]
        AlreadyExists,

        #[Response(code = 500)]
        ServerError,
    }

    //
    // endregion: rename task

    // region: change status
    //

    /// Mark the task as done.
    #[Route(method="put", path="/tasks/{task_id}/status/done")]
    pub async fn set_done(
        Extension(task_service): Extension<Arc<TaskService>>,
        Path(task_id): Path<TaskId>
    ) -> Result<Task, ChangeStatusError> {
        let result = task_service.change_status(task_id.task_id.into(), Status::Done).await;
        map_change_status_result(result)
    }

    /// Mark the task as pending.
    #[Route(method="put", path="/tasks/{task_id}/status/pending")]
    pub async fn set_pending(
        Extension(task_service): Extension<Arc<TaskService>>,
        Path(task_id): Path<TaskId>
    ) -> Result<Task, ChangeStatusError> {
        let result = task_service.change_status(task_id.task_id.into(), Status::Pending).await;
        map_change_status_result(result)
    }

    /// Mark the task as cancelled.
    #[Route(method="put", path="/tasks/{task_id}/status/cancel")]
    pub async fn set_cancelled(
        Extension(task_service): Extension<Arc<TaskService>>,
        Path(task_id): Path<TaskId>
    ) -> Result<Task, ChangeStatusError> {
        let result = task_service.change_status(task_id.task_id.into(), Status::Cancelled).await;
        map_change_status_result(result)
    }

    #[Response(format(json))]
    pub enum ChangeStatusError {
        #[Response(code = 404)]
        NotFound,

        /// Returned if the operation would result in a duplicated task in Pending status.
        #[Response(code = 409)]
        Duplicate,

        #[Response(code = 500)]
        ServerError,
    }

    fn map_change_status_result(r: Result<model::Task, task_service::ChangeStatusError>) -> Result<Task, ChangeStatusError> {
        match r {
            Ok(t) => 
                match Task::try_from(&t) {
                    Ok(d)  => Ok(d),
                    Err(_) => Err(ChangeStatusError::ServerError),
                }
            ,

            Err(e) => match e {
                task_service::ChangeStatusError::NotFound => 
                    Err(ChangeStatusError::NotFound),

                task_service::ChangeStatusError::Duplicate => 
                    Err(ChangeStatusError::Duplicate),

                task_service::ChangeStatusError::StorageReadError(err)
                | task_service::ChangeStatusError::StorageWriteError(err) => {
                    log_storage_error(&err, "storage error when changing task status");
                    Err(ChangeStatusError::ServerError)
                },
            },
        }
    }

    fn log_storage_error(err: &StorageError, message: &'static str) {
        tracing::error!(err = %err, message);
    }

    //
    // endregion: change status
}

/// HTTP-layer models.
mod model {
    use axum::response::IntoResponse;
    use groom_macros::{DTO, Response};
    use serde::Deserialize;
    use static_assertions::assert_impl_any;
    use utoipa::ToSchema;

    use crate::service::{model, task_service};

    //
    // Task
    //

    #[Response(format(json), code = 200)]
    pub struct Task {
        pub id:     u64,
        pub title:  String,
        pub status: model::Status,
    }

    impl TryFrom<&model::Task> for Task {
        type Error = ();
        
        /// Converts Task domain model into HTTP viewmodel. If conversion is unsuccessful, logs the error.
        /// 
        /// The conversion may fail if domain model has null id. This should never happen in production
        fn try_from(t: &model::Task) -> Result<Self, Self::Error> {
            Ok(Task {
                id: if let Some(id) = t.id() {
                        id.value()
                    } else {
                        tracing::error!("task_id is expected to be set");
                        return Err(())
                    },
                title: t.title().to_owned(),
                status: t.status(),
            })
        }
    }

    //
    // TaskSortField
    //

    #[derive(Default, Debug, Deserialize, ToSchema)]
    #[serde(rename_all = "lowercase")]
    pub enum TaskSortField {
        #[default]
        Id,
        Title,
        Status,
    }

    impl From<TaskSortField> for task_service::TaskSortField {
        fn from(field: TaskSortField) -> Self {
            match field {
                TaskSortField::Id     => task_service::TaskSortField::Id,
                TaskSortField::Title  => task_service::TaskSortField::Title,
                TaskSortField::Status => task_service::TaskSortField::Status,
            }
        }
    }

    //
    // SortOrder
    //

    #[derive(Default, Debug, Deserialize, ToSchema)]
    #[serde(rename_all = "lowercase")]
    pub enum SortOrder {
        #[default]
        #[serde(rename = "asc")]
        Ascending,

        #[serde(rename = "desc")]
        Descending,
    }

    impl From<SortOrder> for task_service::SortOrder {
        fn from(order: SortOrder) -> Self {
            match order {
                SortOrder::Ascending  => task_service::SortOrder::Ascending,
                SortOrder::Descending => task_service::SortOrder::Descending,
            }
        }
    }
}
