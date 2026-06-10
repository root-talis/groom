use std::sync::Arc;

use crate::service::{
    error::StorageError,
    model::{self, Status, Task, TaskID},
    port::repository::{self, TaskReader, TaskWriter},
};

use repository::{AddError, ReadError::DatabaseFailure, UpdateError};


#[cfg(test)]
mod tests;


#[derive(Clone)]
pub struct TaskService {
    reader: Arc<dyn TaskReader>,
    writer: Arc<dyn TaskWriter>,
}

impl TaskService {
    pub fn new(reader: Arc<dyn TaskReader>, writer: Arc<dyn TaskWriter>) -> Self {
        Self { reader, writer }
    }
    
    fn validate_title(t: &str) -> Result<(), &'static str> {
        if t.trim().chars().count() <= 3 {
            Err("title is too short")
        } else {
            Ok(())
        }
    }
}

// region: add_task
//

pub struct AddTaskRequest {
    pub title: String,
}

#[derive(Debug, thiserror::Error)]
pub enum AddTaskError {
    #[error("task already exists")]
    Duplicate,

    #[error("invalid request: {0}")]
    InvalidRequest(&'static str),

    #[error("storage error")]
    StorageError(#[from] StorageError),
}

impl TaskService {
    pub async fn add_task(&self, req: AddTaskRequest) -> Result<Task, AddTaskError> {
        Self::validate_title(&req.title).map_err(AddTaskError::InvalidRequest)?;

        self.writer
            .add_task(model::Task::new(req.title, Status::Pending))
            .await
            .map_err(|e| match e {
                AddError::NotUnique =>
                    AddTaskError::Duplicate,

                AddError::DatabaseFailure =>
                    AddTaskError::StorageError(StorageError::new(e)),
            })
    }
}

//
// endregion: add_task

// region: get_task_by_id
//

#[derive(Debug, thiserror::Error)]
pub enum GetTaskError {
    #[error("storage error")]
    StorageError(#[from] StorageError),
}

impl TaskService {
    pub async fn get_task_by_id(&self, id: TaskID) -> Result<Option<Task>, GetTaskError> {
        self.reader.get_task_by_id(id).await.map_err(|e| match e {
            DatabaseFailure => GetTaskError::StorageError(StorageError::new(e)),
        })
    }
}

//
// endregion: get_task_by_id

// region: list_tasks
//

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ListTasksRequest {
    pub title:  Option<String>,
    pub status: Option<Vec<Status>>,

    pub sort_by: TaskSortField,
    pub order:   SortOrder,
}

impl Into<repository::GetTasksQuery> for ListTasksRequest {
    fn into(self) -> repository::GetTasksQuery {
        repository::GetTasksQuery {
            title:   self.title,
            status:  self.status,
            sort_by: match self.sort_by {
                TaskSortField::Id     => repository::TaskSortField::Id,
                TaskSortField::Title  => repository::TaskSortField::Title,
                TaskSortField::Status => repository::TaskSortField::Status,
            },
            order: match self.order {
                SortOrder::Ascending  => repository::SortOrder::Ascending,
                SortOrder::Descending => repository::SortOrder::Descending,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TaskSortField {
    #[default]
    Id,
    Title,
    Status,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SortOrder {
    #[default]
    Ascending,
    Descending,
}

#[derive(Debug, thiserror::Error)]
pub enum ListTasksError {
    #[error("storage error")]
    StorageError(#[from] StorageError),
}

impl TaskService {
    pub async fn list_tasks(&self, req: ListTasksRequest) -> Result<Vec<Task>, ListTasksError> {
        self.reader.get_tasks(req.into())
            .await
            .map_err(|e| match e {
                DatabaseFailure => ListTasksError::StorageError(StorageError::new(e)),
            })
    }
}

//
// endregion: list_tasks

// region: rename task
//

impl TaskService {
    pub async fn rename_task(&self, task_id: TaskID, title: String) -> Result<Task, RenameTaskError> {
        Self::validate_title(&title).map_err(RenameTaskError::InvalidRequest)?;

        let mut task = self.reader.get_task_by_id(task_id)
            .await
            .map_err(|e| match e {
                DatabaseFailure => RenameTaskError::StorageReadError(StorageError::new(e)),
            })?
            .ok_or(RenameTaskError::NotFound)?
        ;
    
        task.set_title(title);

        self.writer.update_task(task).await.map_err(|e| match e {
            UpdateError::NotFound => RenameTaskError::NotFound,
            UpdateError::NotUnique => RenameTaskError::Duplicate,
            UpdateError::DatabaseFailure =>
                RenameTaskError::StorageWriteError(StorageError::new(e)),
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RenameTaskError {
    #[error("task not found")]
    NotFound,
    
    #[error("invalid request: {0}")]
    InvalidRequest(&'static str),

    #[error("task becomes a duplicate after rename")]
    Duplicate,

    #[error("storage error while fetching task")]
    StorageReadError(#[source] StorageError),

    #[error("storage error while updating task")]
    StorageWriteError(#[source] StorageError),
}

//
// endregion: rename task

// region: change status
//

impl TaskService {
    pub async fn change_status(&self, task_id: TaskID, status: Status) -> Result<Task, ChangeStatusError> {
        let mut task = self.reader.get_task_by_id(task_id)
            .await
            .map_err(|e| match e {
                DatabaseFailure => ChangeStatusError::StorageReadError(StorageError::new(e)),
            })?
            .ok_or(ChangeStatusError::NotFound)?
        ;
    
        task.set_status(status);

        self.writer.update_task(task).await.map_err(|e| match e {
            UpdateError::NotFound => ChangeStatusError::NotFound,
            UpdateError::NotUnique => ChangeStatusError::Duplicate,
            UpdateError::DatabaseFailure =>
                ChangeStatusError::StorageWriteError(StorageError::new(e)),
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ChangeStatusError {
    #[error("task not found")]
    NotFound,
    
    #[error("task becomes a duplicate after status change")]
    Duplicate,

    #[error("storage error while fetching task")]
    StorageReadError(#[source] StorageError),

    #[error("storage error while updating task")]
    StorageWriteError(#[source] StorageError),
}

//
// endregion: change status
