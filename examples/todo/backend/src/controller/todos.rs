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
    ) -> ListTasksResponse {
        debug!("List tasks request: {:?}", req);
        match task_service.list_tasks(req.into()).await {
            Ok(l) => {
                let tasks: Result<Vec<Task>, ()> = l.iter()
                    .map(|t| Task::try_from(t.clone()))
                    .collect()
                ;

                match tasks {
                    Ok(v)  => ListTasksResponse::Ok(TasksList(v)),
                    Err(_) => ListTasksResponse::ServerError,
                }
            },
            Err(_) => ListTasksResponse::ServerError,
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

    #[DTO(response)]
    pub struct TasksList(Vec<Task>);

    /// List of tasks.
    #[Response(format(json))]
    pub enum ListTasksResponse {
        #[Response(code = 200)]
        Ok(TasksList),

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
        Path(path): Path<TaskId>
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

    /// Path parameter identifying a task.
    #[DTO(parameters)]
    pub struct TaskId {
        pub task_id: u64,
    }

    /// Single task.
    #[Response(format(json))]
    pub enum GetTaskResponse {
        #[Response(code = 200)]
        Ok(Task),

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
        let req = task_service::AddTaskRequest {
            title: req.title,
        };

        match task_service.add_task(req).await {
            Ok(t) => match t.try_into() {
                Ok(v)  => AddTaskResponse::Ok(v),
                Err(_) => AddTaskResponse::ServerError,
            },

            Err(e) => match e {
                task_service::AddTaskError::Duplicate =>
                    AddTaskResponse::AlreadyExists,

                task_service::AddTaskError::InvalidRequest(reason) => 
                    AddTaskResponse::MalformedRequest(reason.into()),

                task_service::AddTaskError::StorageError(err) => {
                    tracing::error!(err = %err, "storage error when adding task");
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
        Ok(Task),

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
    ) -> RenameTaskResponse {
        let result = task_service.rename_task(TaskID::from(task_id.task_id), req.title).await;
        match result {
            Ok(t) => 
                match Task::try_from(t) {
                    Ok(d)  => RenameTaskResponse::Ok(d),
                    Err(_) => RenameTaskResponse::ServerError,
                }
            ,

            Err(e) => match e {
                task_service::RenameTaskError::InvalidRequest(d) =>
                    RenameTaskResponse::MalformedRequest(d.into()),

                task_service::RenameTaskError::NotFound => 
                    RenameTaskResponse::NotFound,

                task_service::RenameTaskError::Duplicate => 
                    RenameTaskResponse::AlreadyExists,

                task_service::RenameTaskError::StorageReadError(_)
                | task_service::RenameTaskError::StorageWriteError(_) => 
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
        Ok(Task),

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
    ) -> ChangeStatusResponse {
        let result = task_service.change_status(task_id.task_id.into(), Status::Done).await;
        map_change_status_result(result)
    }

    /// Mark the task as pending.
    #[Route(method="put", path="/tasks/{task_id}/status/pending")]
    pub async fn set_pending(
        Extension(task_service): Extension<Arc<TaskService>>,
        Path(task_id): Path<TaskId>
    ) -> ChangeStatusResponse {
        let result = task_service.change_status(task_id.task_id.into(), Status::Pending).await;
        map_change_status_result(result)
    }

    /// Mark the task as cancelled.
    #[Route(method="put", path="/tasks/{task_id}/status/cancel")]
    pub async fn set_cancelled(
        Extension(task_service): Extension<Arc<TaskService>>,
        Path(task_id): Path<TaskId>
    ) -> ChangeStatusResponse {
        let result = task_service.change_status(task_id.task_id.into(), Status::Cancelled).await;
        map_change_status_result(result)
    }

    /// Result of changing the status of task
    #[Response(format(json))]
    pub enum ChangeStatusResponse {
        #[Response(code = 200)]
        Ok(Task),

        #[Response(code = 404)]
        NotFound,

        #[Response(code = 409)]
        Duplicate,

        #[Response(code = 500)]
        ServerError,
    }

    fn map_change_status_result(r: Result<model::Task, task_service::ChangeStatusError>) -> ChangeStatusResponse {
        match r {
            Ok(t) => 
                match Task::try_from(t) {
                    Ok(d)  => ChangeStatusResponse::Ok(d),
                    Err(_) => ChangeStatusResponse::ServerError,
                }
            ,

            Err(e) => match e {
                task_service::ChangeStatusError::NotFound => 
                    ChangeStatusResponse::NotFound,

                task_service::ChangeStatusError::Duplicate => 
                    ChangeStatusResponse::Duplicate,

                task_service::ChangeStatusError::StorageReadError(_)
                | task_service::ChangeStatusError::StorageWriteError(_) => 
                    ChangeStatusResponse::ServerError,
            },
        }
    }

    //
    // endregion: change status
}

/// HTTP-layer models.
mod model {
    use groom_macros::DTO;
    use serde::Deserialize;
    use utoipa::ToSchema;

    use crate::service::{model, task_service};

    //
    // Task
    //

    #[DTO(response)]
    pub struct Task {
        pub id:     u64,
        pub title:  String,
        pub status: model::Status,
    }

    impl TryFrom<model::Task> for Task {
        type Error = ();
        
        fn try_from(t: model::Task) -> Result<Self, Self::Error> {
            Ok(Task {
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
