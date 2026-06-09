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
    use tracing::debug;

    use crate::service::{
        model::{Status, Task, TaskID},
        repository::TaskFilter,
        task_service::{self, AddTaskError, ChangeStatusError, RenameTaskError, TaskService}
    };

    use super::model::{SortDirection, TaskViewModel, TasksSortBy};

    // region: list tasks
    //

    /// Lists tasks
    #[Route(method="get", path="/tasks")]
    pub async fn list_tasks(
        Extension(task_service): Extension<Arc<TaskService>>,
        Query(filters): Query<TaskListFilters>
    ) -> TaskListResponse {
        debug!("Filters parsed: {:?}", filters);
        match task_service.list_tasks(filters.into()).await {
            Ok(l) => {
                let tasks: Result<Vec<TaskViewModel>, ()> = l.iter()
                    .map(|t| TaskViewModel::try_from(t.clone()))
                    .collect()
                ;

                match tasks {
                    Ok(v)  => TaskListResponse::Ok(TaskList(v)),
                    Err(_) => TaskListResponse::ServerError,
                }
            },
            Err(_) => TaskListResponse::ServerError,
        }
    }

    /// Task list filters
    #[DTO(parameters)]
    #[derive(Debug)]
    pub struct TaskListFilters {
        pub title:  Option<String>,
        pub status: Option<Vec<Status>>,

        #[serde(default)]
        pub sort_by: TasksSortBy,

        #[serde(default)]
        pub order:   SortDirection,
    }

    impl From<TaskListFilters> for TaskFilter {
        fn from(val: TaskListFilters) -> Self {
            TaskFilter {
                title:   val.title,
                status:  val.status,
                sort_by: val.sort_by.into(),
                order:   val.order.into(),
            }
        }
    }

    #[DTO(response)]
    pub struct TaskList(Vec::<TaskViewModel>);

    /// List of tasks.
    #[Response(format(json))]
    pub enum TaskListResponse {
        #[Response(code = 200)]
        Ok(TaskList),

        #[Response(code = 500)]
        ServerError,
    }

    //
    // endregion: list tasks

    // region: get task
    //

    /// Gets a single task.
    #[Route(method="get", path="/tasks/{task_id}")]
    pub async fn get_task(
        Extension(task_service): Extension<Arc<TaskService>>,
        Path(path): Path<TaskIdentifier>
    ) -> GetTaskResponse {
        match task_service.get_task_by_id(TaskID::from(path.task_id)).await {
            Ok(maybe) => match maybe {
                None => GetTaskResponse::NotFound,
                Some(t) => 
                    match t.try_into() {
                        Ok(v)  => GetTaskResponse::Ok(v),
                        Err(_) => GetTaskResponse::ServerError,
                    },
            },
            Err(_) => todo!(),
        }
    }

    /// Params to get a single task
    #[DTO(parameters)]
    pub struct TaskIdentifier {
        pub task_id: u64,
    }

    /// Single task.
    #[Response(format(json))]
    pub enum GetTaskResponse {
        #[Response(code = 200)]
        Ok(TaskViewModel),

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
    ) -> AddTaskResponse {
        let req = task_service::AddTaskRequest{
            title: req.title,
        };

        match task_service.add_task(req).await {
            Ok(t) => match t.try_into() {
                Ok(v)  => AddTaskResponse::Ok(v),
                Err(_) => AddTaskResponse::ServerError,
            },

            Err(e) => match e {
                AddTaskError::Duplicate =>
                    AddTaskResponse::AlreadyExists,

                AddTaskError::InvalidRequest(reason) => 
                    AddTaskResponse::MalformedRequest(reason.into()),

                AddTaskError::StorareError(task_add_error) => {
                    tracing::error!(err = task_add_error.to_string(), "storage error when adding task");
                    AddTaskResponse::ServerError
                },
            },
        }
    }

    /// Request body
    #[RequestBody(format(json, url_encoded))]
    pub struct AddTaskRequest {
        pub title: String,
    }

    /// Result of adding a task
    #[Response(format(json))]
    pub enum AddTaskResponse {
        /// Task added successfully
        #[Response(code = 200)]
        Ok(TaskViewModel),

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
        Path(task_id): Path<TaskIdentifier>,
        req: RenameTaskRequest
    ) -> RenameTaskResponse {
        let result = task_service.rename_task(TaskID::from(task_id.task_id), req.title).await;
        match result {
            Ok(t) => 
                match TaskViewModel::try_from(t) {
                    Ok(d)  => RenameTaskResponse::Ok(d),
                    Err(_) => RenameTaskResponse::ServerError,
                }
            ,

            Err(e) => match e {
                RenameTaskError::InvalidRequest(d) =>
                    RenameTaskResponse::MalformedRequest(d.into()),

                RenameTaskError::NotFound => 
                    RenameTaskResponse::NotFound,

                RenameTaskError::Duplicate => 
                    RenameTaskResponse::AlreadyExists,

                RenameTaskError::StorageReadError(_) | RenameTaskError::StorageWriteError(_) => 
                    RenameTaskResponse::ServerError,
            },
        }
    }

    /// Request body
    #[RequestBody(format(json, url_encoded))]
    pub struct RenameTaskRequest {
        pub title: String,
    }

    /// Result of renaming a task
    #[Response(format(json))]
    pub enum RenameTaskResponse {
        #[Response(code = 200)]
        Ok(TaskViewModel),

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
        Path(task_id): Path<TaskIdentifier>
    ) -> ChangeStatusResponse {
        let result = task_service.change_status(task_id.task_id.into(), Status::Done).await;
        map_change_status_result(result)
    }

    /// Mark the task as pending.
    #[Route(method="put", path="/tasks/{task_id}/status/pending")]
    pub async fn set_pending(
        Extension(task_service): Extension<Arc<TaskService>>,
        Path(task_id): Path<TaskIdentifier>
    ) -> ChangeStatusResponse {
        let result = task_service.change_status(task_id.task_id.into(), Status::Pending).await;
        map_change_status_result(result)
    }

    /// Mark the task as cancelled.
    #[Route(method="put", path="/tasks/{task_id}/status/cancel")]
    pub async fn set_cancelled(
        Extension(task_service): Extension<Arc<TaskService>>,
        Path(task_id): Path<TaskIdentifier>
    ) -> ChangeStatusResponse {
        let result = task_service.change_status(task_id.task_id.into(), Status::Cancelled).await;
        map_change_status_result(result)
    }

    /// Result of changing the status of task
    #[Response(format(json))]
    pub enum ChangeStatusResponse {
        #[Response(code = 200)]
        Ok(TaskViewModel),

        #[Response(code = 404)]
        NotFound,

        #[Response(code = 409)]
        Duplicate,

        #[Response(code = 500)]
        ServerError,
    }

    fn map_change_status_result(r: Result<Task, ChangeStatusError>) -> ChangeStatusResponse {
        match r {
            Ok(t) => 
                match TaskViewModel::try_from(t) {
                    Ok(d)  => ChangeStatusResponse::Ok(d),
                    Err(_) => ChangeStatusResponse::ServerError,
                }
            ,

            Err(e) => match e {
                ChangeStatusError::NotFound => 
                    ChangeStatusResponse::NotFound,

                ChangeStatusError::Duplicate => 
                    ChangeStatusResponse::Duplicate,

                ChangeStatusError::StorageReadError(_) | ChangeStatusError::StorageWriteError(_) => 
                    ChangeStatusResponse::ServerError,
            },
        }
    }

    //
    // endregion: change status
}

/// View models for HTTP layer
mod model {
    use groom_macros::DTO;
    use serde::Deserialize;
    use utoipa::ToSchema;

    use crate::service::{model::{Status, Task}, repository::{Order, TaskOrderField}};

    //
    // TaskViewModel
    //

    #[DTO(response)]
    pub struct TaskViewModel {
        pub id:     u64,
        pub title:  String,
        pub status: Status,
    }

    impl TryFrom<Task> for TaskViewModel {
        type Error = ();
        
        fn try_from(t: Task) -> Result<Self, Self::Error> {
            Ok(TaskViewModel {
                id: if let Some(id) = t.id() {
                        id.value()
                    } else {
                        tracing::error!("task_id is expected to be set");
                        return Err(())
                    },
                title: t.title(),
                status: t.status(),
            })
        }
    }

    //
    // TasksSortBy
    //

    #[derive(Default, Debug, Deserialize, ToSchema)]
    #[serde(rename_all = "lowercase")]
    pub enum TasksSortBy {
        #[default]
        Id,
        Title,
        Status
    }

    impl From<TasksSortBy> for TaskOrderField {
        fn from(val: TasksSortBy) -> Self {
            match val {
                TasksSortBy::Id     => TaskOrderField::Id,
                TasksSortBy::Title  => TaskOrderField::Title,
                TasksSortBy::Status => TaskOrderField::Status,
            }
        }
    }

    //
    // SortDirection
    //

    #[derive(Default, Debug, Deserialize, ToSchema)]
    #[serde(rename_all = "lowercase")]
    pub enum SortDirection {
        #[default]
        Asc,
        Desc
    }

    impl From<SortDirection> for Order {
        fn from(val: SortDirection) -> Self {
            match val {
                SortDirection::Asc  => Order::Ascending,
                SortDirection::Desc => Order::Descending,
            }
        }
    }
}
