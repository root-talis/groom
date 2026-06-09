use std::sync::Arc;

use crate::service::{model::{Status, Task, TaskID}, repository::{TaskAddRepositoryError, TaskFilter, TaskReadRepositoryError, TaskReader, TaskWriter}};

use super::repository::{TaskReadRepositoryError::DatabaseFailure, TaskUpdateRepositoryError};


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
        if t.trim().len() <= 3 {
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
    StorareError(#[source] TaskAddRepositoryError),
}

impl TaskService {
    pub async fn add_task(&self, req: AddTaskRequest) -> Result<Task, AddTaskError> {
        Self::validate_title(&req.title).map_err(AddTaskError::InvalidRequest)?;

        self.writer
            .add_task(Task::new(req.title, Status::Pending))
            .await
            .map_err(|e| match e {
                TaskAddRepositoryError::NotUnique => 
                    AddTaskError::Duplicate,

                TaskAddRepositoryError::DatabaseFailure => 
                    AddTaskError::StorareError(e),
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
    StorareError(#[source] TaskReadRepositoryError),
}

impl TaskService {
    pub async fn get_task_by_id(&self, id: TaskID) -> Result<Option<Task>, GetTaskError> {
        self.reader.get_task_by_id(id).await.map_err(|e| match e {
            DatabaseFailure => GetTaskError::StorareError(e),
        })
    }
}

//
// endregion: get_task_by_id

// region: list_tasks
//

#[derive(Debug, thiserror::Error)]
pub enum ListTasksError {
    #[error("storage error")]
    StorareError(#[source] TaskReadRepositoryError),
}

impl TaskService {
    pub async fn list_tasks(&self, filter: TaskFilter) -> Result<Vec<Task>, ListTasksError> {
        self.reader.get_tasks(filter).await.map_err(|e| match e {
            DatabaseFailure => ListTasksError::StorareError(e),
        })
    }
}

//
// endregion: get_task_by_id

// region: rename task
//

impl TaskService {
    pub async fn rename_task(&self, task_id: TaskID, title: String) -> Result<Task, RenameTaskError> {
        Self::validate_title(&title).map_err(RenameTaskError::InvalidRequest)?;

        let mut task = self.reader.get_task_by_id(task_id)
            .await
            .map_err(|e| match e {
                DatabaseFailure => RenameTaskError::StorageReadError(e),
            })?
            .ok_or(RenameTaskError::NotFound)?
        ;
    
        task.set_title(title);

        self.writer.update_task(task).await.map_err(|e| match e {
            TaskUpdateRepositoryError::NotFound => RenameTaskError::NotFound,
            TaskUpdateRepositoryError::NotUnique => RenameTaskError::Duplicate,
            TaskUpdateRepositoryError::DatabaseFailure => RenameTaskError::StorageWriteError(e),
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
    StorageReadError(#[source] TaskReadRepositoryError),

    #[error("storage error while updating task")]
    StorageWriteError(#[source] TaskUpdateRepositoryError),
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
                DatabaseFailure => ChangeStatusError::StorageReadError(e),
            })?
            .ok_or(ChangeStatusError::NotFound)?
        ;
    
        task.set_status(status);

        self.writer.update_task(task).await.map_err(|e| match e {
            TaskUpdateRepositoryError::NotFound => ChangeStatusError::NotFound,
            TaskUpdateRepositoryError::NotUnique => ChangeStatusError::Duplicate,
            TaskUpdateRepositoryError::DatabaseFailure => ChangeStatusError::StorageWriteError(e),
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
    StorageReadError(#[source] TaskReadRepositoryError),

    #[error("storage error while updating task")]
    StorageWriteError(#[source] TaskUpdateRepositoryError),
}

//
// endregion: change status
