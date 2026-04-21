use std::sync::Arc;

use crate::{repository::tasks_repository::InMemoryTaskRepository, service::task_service::TaskService};

pub struct Bootstrap {
    pub task_service: Arc<TaskService>
}

impl Bootstrap {
    pub fn new() -> Self {
        Self { task_service: Arc::from(Self::make_task_service()) }
    }

    fn make_task_service() -> TaskService {
        let repo = Arc::new(Self::make_in_memory_task_repo());
        TaskService::new(repo.clone(), repo.clone())
    }

    fn make_in_memory_task_repo() -> InMemoryTaskRepository {
        InMemoryTaskRepository::new()
    }
}
