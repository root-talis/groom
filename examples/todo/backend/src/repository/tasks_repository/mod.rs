use std::collections::HashMap;

use tokio::sync::Mutex;

use crate::service::{
    model::{Status, Task, TaskID},
    port::repository::{
        AddError, GetTasksQuery, ReadError, SortOrder, TaskReader, TaskSortField, TaskWriter,
        UpdateError,
    },
};

/// Thread safe in-memory implementation of TaskReader and TaskWriter
pub struct InMemoryTaskRepository {
    store: Mutex<Store>,
}

impl Default for InMemoryTaskRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl InMemoryTaskRepository {
    pub fn new() -> Self {
        Self { 
            store: Mutex::from(Store::new()) 
        }
    }
}

#[async_trait::async_trait]
impl TaskReader for InMemoryTaskRepository {
    async fn get_tasks(&self, query: GetTasksQuery) -> Result<Vec<Task>, ReadError> {
        self.store.lock().await.get_tasks(query)
    }

    async fn get_task_by_id(&self, id: TaskID) -> Result<Option<Task>, ReadError> {
        self.store.lock().await.get_task_by_id(id)
    }
}

#[async_trait::async_trait]
impl TaskWriter for InMemoryTaskRepository {
    async fn add_task(&self, task: Task) -> Result<Task, AddError> {
        self.store.lock().await.add_task(task)
    }

    async fn update_task(&self, task: Task) -> Result<Task, UpdateError> {
        self.store.lock().await.update_task(task)
    }
}

/// Inner non thread safe storage
struct Store {
    tasks: HashMap<TaskID, Task>,
    next_id: TaskID,
}

impl Store {
    pub fn new() -> Self {
        Self { tasks: HashMap::new(), next_id: TaskID::from(1) }
    }

    fn is_duplicate(&self, task: &Task) -> bool {
        if task.status() != Status::Pending {
            return false;
        }

        self.tasks.values()
            .find(|v| 
                   v.status() == Status::Pending 
                && v.id() != task.id() 
                && v.title() == task.title()
            )
            .is_some()
    }

    pub fn add_task(&mut self, mut task: Task) -> Result<Task, AddError> {
        if self.is_duplicate(&task) {
            return Err(AddError::NotUnique);
        }

        let id = self.next_id;
        self.next_id = TaskID::from(self.next_id.value() + 1);

        if self.tasks.contains_key(&id) {
            panic!("BUGBUG: hashmap already contains new id");
        }

        task.set_id(id);
        self.tasks.insert(id, task.clone());

        Ok(task)
    }

    pub fn update_task(&mut self, task: Task) -> Result<Task, UpdateError> {
        if self.is_duplicate(&task) {
            return Err(UpdateError::NotUnique);
        }

        let id = match task.id() {
            Some(id) => id,
            None => return Err(UpdateError::NotFound),
        };

        if !self.tasks.contains_key(&id) {
            return Err(UpdateError::NotFound);
        }

        self.tasks.insert(id, task.clone());
        Ok(task)
    }

    pub fn get_task_by_id(&self, id: TaskID) -> Result<Option<Task>, ReadError> {
        Ok(self.tasks.get(&id).cloned())
    }

    pub fn get_tasks(&self, query: GetTasksQuery) -> Result<Vec<Task>, ReadError> {
        let mut t: Vec<Task> = self.tasks
            .values()
            .filter(|t| {
                if let Some(status) = &query.status {
                    status.contains(&t.status())
                } else {
                    true
                }
            })
            .filter(|t| {
                if let Some(title) = &query.title {
                    t.title().contains(title)
                } else {
                    true
                }
            })
            .cloned()
            .collect()
        ;

        t.sort_by(|a, b| {
            let by_id = a.id().cmp(&b.id());
            let ordering = match query.sort_by {
                TaskSortField::Id     => by_id,
                TaskSortField::Title  => a.title().cmp(&b.title()).then(by_id),
                TaskSortField::Status => a.status().cmp(&b.status()).then(by_id),
            };

            match query.order {
                SortOrder::Ascending  => ordering,
                SortOrder::Descending => ordering.reverse(),
            }
        });

        Ok(t)
    }
}

#[cfg(test)]
mod tests;

