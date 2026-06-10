pub use interfaces::*;
pub use models::*;

mod interfaces {
    use crate::service::{
        model::{Task, TaskID},
        port::repository::{AddError, GetTasksQuery, ReadError, UpdateError},
    };

    #[cfg(test)]
    use mockall::{automock, predicate::*};

    #[cfg_attr(test, automock)]
    #[async_trait::async_trait]
    pub trait TaskReader: Send + Sync {
        async fn get_tasks(&self, query: GetTasksQuery) -> Result<Vec<Task>, ReadError>;
        async fn get_task_by_id(&self, id: TaskID) -> Result<Option<Task>, ReadError>;
    }

    #[cfg_attr(test, automock)]
    #[async_trait::async_trait]
    pub trait TaskWriter: Send + Sync {
        async fn add_task(&self, task: Task) -> Result<Task, AddError>;
        async fn update_task(&self, task: Task) -> Result<Task, UpdateError>;
    }
}

mod models {
    use serde::Deserialize;

    use crate::service::model::Status;

    #[derive(Debug, Clone, Deserialize, Default, PartialEq, Eq)]
    pub struct GetTasksQuery {
        pub title:  Option<String>,
        pub status: Option<Vec<Status>>,

        pub sort_by: TaskSortField,
        pub order:   SortOrder,
    }

    #[derive(Debug, Clone, Copy, Deserialize, Default, PartialEq, Eq)]
    pub enum TaskSortField {
        #[default]
        Id,
        Title,
        Status,
    }

    #[derive(Debug, Clone, Copy, Deserialize, Default, PartialEq, Eq)]
    pub enum SortOrder {
        #[default]
        Ascending,
        Descending,
    }

    #[derive(Debug, thiserror::Error)]
    pub enum ReadError {
        #[error("database failure")]
        DatabaseFailure,
    }

    #[derive(Debug, thiserror::Error)]
    pub enum AddError {
        #[error("task already exists")]
        NotUnique,

        #[error("database failure")]
        DatabaseFailure,
    }

    #[derive(Debug, thiserror::Error)]
    pub enum UpdateError {
        #[error("task not found")]
        NotFound,

        #[error("task already exists")]
        NotUnique,

        #[error("database failure")]
        DatabaseFailure,
    }
}
