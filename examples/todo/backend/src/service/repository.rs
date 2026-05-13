pub use interfaces::*;
pub use models::*;

mod interfaces {
    use crate::service::{
        model::{Task, TaskID},
        repository::{TaskAddRepositoryError, TaskFilter, TaskReadRepositoryError, TaskUpdateRepositoryError}
    };

    #[cfg(test)]
    use mockall::{automock, predicate::*};

    #[cfg_attr(test, automock)]
    #[async_trait::async_trait]
    pub trait TaskReader: Send + Sync {
        async fn get_tasks(&self, filter: TaskFilter) -> Result<Vec<Task>, TaskReadRepositoryError>;
        async fn get_task_by_id(&self, id: TaskID) -> Result<Option<Task>, TaskReadRepositoryError>;
    }

    #[cfg_attr(test, automock)]
    #[async_trait::async_trait]
    pub trait TaskWriter: Send + Sync {
        async fn add_task(&self, task: Task) -> Result<Task, TaskAddRepositoryError>;
        async fn update_task(&self, task: Task) -> Result<Task, TaskUpdateRepositoryError>;
    }
}

mod models {
    use serde::Deserialize;

    use crate::service::model::Status;

    #[derive(Debug, Clone, Deserialize, Default, PartialEq, Eq)]
    pub struct TaskFilter {
        pub title: Option<String>,
        pub status: Option<Vec<Status>>,

        pub sort_by: TaskOrderField,
        pub order: Order,
    }

    #[derive(Debug, Clone, Copy, Deserialize, Default, PartialEq, Eq)]
    pub enum TaskOrderField {
        #[default]
        Id,
        Title,
        Status
    }

    #[derive(Debug, Clone, Copy, Deserialize, Default, PartialEq, Eq)]
    pub enum Order {
        #[default]
        Ascending,
        Descending
    }

    #[derive(Debug, thiserror::Error)]
    pub enum TaskReadRepositoryError {
        #[error("database failure")]
        DatabaseFailure
    }

    #[derive(Debug, thiserror::Error)]
    pub enum TaskAddRepositoryError {
        #[error("task already exists")]
        NotUnique,

        #[error("database failure")]
        DatabaseFailure
    }

    #[derive(Debug, thiserror::Error)]
    pub enum TaskUpdateRepositoryError {
        #[error("task not found")]
        NotFound,

        #[error("task already exists")]
        NotUnique,

        #[error("database failure")]
        DatabaseFailure
    }
}
