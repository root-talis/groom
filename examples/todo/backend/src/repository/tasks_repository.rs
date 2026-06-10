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
mod tests {
    use std::collections::BTreeSet;

    use crate::{
        repository::tasks_repository::InMemoryTaskRepository,
        service::{
            model::{Status, Task},
            port::repository::{
                AddError, GetTasksQuery, ReadError, SortOrder, TaskReader, TaskSortField,
                TaskWriter, UpdateError,
            },
        },
    };
    use assert_matches::assert_matches;

    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn should_add_tasks() {
        let r = InMemoryTaskRepository::new();

        let tasks = vec![
            Task::new("do something", Status::Pending),
            Task::new("do something else", Status::Pending),
            Task::new("don't do this", Status::Cancelled),
            Task::new("already done that", Status::Done),
        ];

        let mut ids = BTreeSet::new();

        for t in &tasks {
            let added_task = r.add_task(t.clone()).await.expect("call to `add_task` should have succeeded");
            assert_eq!(added_task.title(), t.title());
            assert_eq!(added_task.status(), t.status());

            ids.insert(added_task.id().expect("added task should have an id"));
        }

        assert_eq!(ids.len(), tasks.len(), "should return unique id for each task")
    }

    #[tokio::test]
    async fn should_add_duplicates_if_not_pending() {
        let r = InMemoryTaskRepository::new();

        let tasks = vec![
            Task::new("do something", Status::Cancelled),
            Task::new("do something", Status::Cancelled),
            Task::new("do something", Status::Done),
            Task::new("do something", Status::Done),
            Task::new("do something", Status::Pending),
        ];

        let mut ids = BTreeSet::new();

        for t in &tasks {
            let added_task = r.add_task(t.clone()).await.expect("call to `add_task` should have succeeded");
            assert_eq!(added_task.title(), t.title());
            assert_eq!(added_task.status(), t.status());

            ids.insert(added_task.id().expect("added task should have an id"));
        }

        assert_eq!(ids.len(), tasks.len(), "should return unique id for each task")
    }

    #[tokio::test]
    async fn should_not_add_pending_duplicates() {
        let r = InMemoryTaskRepository::new();

        r.add_task(
            Task::new("do something", Status::Pending)
        ).await.expect("first call to `add_task` should have succeeded");

        let result = r.add_task(
            Task::new("do something", Status::Pending)
        ).await;

        assert_matches!(result, Err(AddError::NotUnique));
    }

    #[tokio::test]
    async fn should_update_tasks() {
        let repo = InMemoryTaskRepository::new();

        let tasks = vec![
            Task::new("do something", Status::Pending),
            Task::new("do something else", Status::Pending),
            Task::new("don't do this", Status::Cancelled),
            Task::new("already done that", Status::Done),
        ];

        let mut added_tasks: Vec<Task> = Vec::new();
        for t in &tasks {
            added_tasks.push(
                repo.add_task(t.clone()).await.expect("call to `add_task` should have succeeded")
            );
        }

        let new_statuses = vec![Status::Done, Status::Cancelled, Status::Pending, Status::Done];
        let new_titles: Vec<String> = (0..tasks.len()).map(|v| format!("Task #{v}: do something.")).collect();
        for i in 0..tasks.len() {
            let mut t = added_tasks[i].clone();
            let id = t.id();

            t.set_title(new_titles[i].clone()).set_status(new_statuses[i]);

            let updated_task = repo.update_task(t).await.expect("call to `update_task` should have succeeded");
            assert_eq!(updated_task.id(), id);
            assert_eq!(updated_task.title(), new_titles[i]);
            assert_eq!(updated_task.status(), new_statuses[i]);

            let refetched_task = repo.get_task_by_id(id.unwrap())
                .await
                .expect("call to `get_task_by_id` should have succeeded")
                .expect("result of `get_task_by_id` should be not None")
            ;
            assert_eq!(refetched_task.id(), id);
            assert_eq!(refetched_task.title(), new_titles[i]);
            assert_eq!(refetched_task.status(), new_statuses[i]);
        }
    }

    #[tokio::test]
    async fn should_forbid_duplicates_on_update_if_pending() {
        let r = InMemoryTaskRepository::new();

        let mut tasks = vec![
            r.add_task(Task::new("first task", Status::Pending)).await.expect("call to `add_task` should have succeeded"),
            r.add_task(Task::new("cancelled task 1", Status::Cancelled)).await.expect("call to `add_task` should have succeeded"),
            r.add_task(Task::new("cancelled task 2", Status::Cancelled)).await.expect("call to `add_task` should have succeeded"),
            r.add_task(Task::new("done task 1", Status::Done)).await.expect("call to `add_task` should have succeeded"),
            r.add_task(Task::new("done task 2", Status::Done)).await.expect("call to `add_task` should have succeeded"),
            r.add_task(Task::new("pending task 1", Status::Pending)).await.expect("call to `add_task` should have succeeded"),
        ];

        let first_task = &mut tasks[0];

        assert_eq!(true, r.update_task(first_task.clone()).await.is_ok(),
            "should save unmodified pending task");

        first_task.set_title(String::from("pending task 1"));
        assert_matches!(r.update_task(first_task.clone()).await, Err(UpdateError::NotUnique),
            "should not save pending task with the title of another pending task");

        first_task.set_title(String::from("cancelled task 1"));
        assert_eq!(true, r.update_task(first_task.clone()).await.is_ok(),
            "should allow duplicated canelled tasks");

        first_task.set_title(String::from("done task 1"));
        assert_eq!(true, r.update_task(first_task.clone()).await.is_ok(),
            "should allow duplicated done tasks");

        first_task.set_title(String::from("pending task 1")).set_status(Status::Cancelled);
        assert_eq!(true, r.update_task(first_task.clone()).await.is_ok(),
            "should save cancelled task with the title of another pending task");

        first_task.set_title(String::from("pending task 1")).set_status(Status::Done);
        assert_eq!(true, r.update_task(first_task.clone()).await.is_ok(),
            "should save done task with the title of another pending task");

        let read_tasks = r.get_tasks(GetTasksQuery::default()).await;
        assert_eq!(true, read_tasks.is_ok(),
            "should read all tasks");
        assert_eq!(read_tasks.unwrap(), tasks, "should read asks as they were saved");
    }

    #[tokio::test]
    async fn should_sort() {
        let r = InMemoryTaskRepository::new();

        r.add_task(Task::new("first task", Status::Pending)).await.unwrap();
        r.add_task(Task::new("cancelled task 1", Status::Cancelled)).await.unwrap();
        r.add_task(Task::new("cancelled task 2", Status::Cancelled)).await.unwrap();
        r.add_task(Task::new("done task 1", Status::Done)).await.unwrap();
        r.add_task(Task::new("done task 2", Status::Done)).await.unwrap();
        r.add_task(Task::new("pending task 1", Status::Pending)).await.unwrap();
        
        let ids = |r: Result<Vec<Task>, ReadError>| -> Vec<u64> {
            r.unwrap().iter().map(|v| v.id().unwrap().value()).collect()
        };

        let req = |f: fn(req: &mut GetTasksQuery) -> ()| {
            let mut req = GetTasksQuery::default();
            f(&mut req);
            req
        };

        // by id
        assert_eq!(
            ids(r.get_tasks(GetTasksQuery::default()).await),
            vec![1, 2, 3, 4, 5, 6],
            "by id asc"
        );
        assert_eq!(
            ids(r.get_tasks(req(|r| r.order = SortOrder::Descending)).await),
            vec![6, 5, 4, 3, 2, 1],
            "by id desc"
        );

        // by title
        assert_eq!(
            ids(r.get_tasks(req(|r| {
                r.sort_by = TaskSortField::Title
            })).await),
            vec![2, 3, 4, 5, 1, 6],
            "by title asc"
        );
        assert_eq!(
            ids(r.get_tasks(req(|r| {
                r.sort_by = TaskSortField::Title;
                r.order = SortOrder::Descending
            })).await),
            vec![6, 1, 5, 4, 3, 2],
            "by title desc"
        );

        // by status
        assert_eq!(
            ids(r.get_tasks(req(|r| {
                r.sort_by = TaskSortField::Status
            })).await),
            vec![1, 6, 4, 5, 2, 3],
            "by status asc"
        );
        assert_eq!(
            ids(r.get_tasks(req(|r| {
                r.sort_by = TaskSortField::Status;
                r.order = SortOrder::Descending
            })).await),
            vec![3, 2, 5, 4, 6, 1],
            "by status desc"
        );
    }

    #[tokio::test]
    async fn should_filter() {
        let r = InMemoryTaskRepository::new();

        r.add_task(Task::new("first task", Status::Pending)).await.unwrap();
        r.add_task(Task::new("cancelled task 1", Status::Cancelled)).await.unwrap();
        r.add_task(Task::new("cancelled task 2", Status::Cancelled)).await.unwrap();
        r.add_task(Task::new("a task that has been done 1", Status::Done)).await.unwrap();
        r.add_task(Task::new("done task 2", Status::Done)).await.unwrap();
        r.add_task(Task::new("pending task 1", Status::Pending)).await.unwrap();

        let ids = |r: Result<Vec<Task>, ReadError>| -> Vec<u64> {
            r.unwrap().iter().map(|v| v.id().unwrap().value()).collect()
        };

        let req = |f: fn(req: &mut GetTasksQuery) -> ()| {
            let mut req = GetTasksQuery::default();
            f(&mut req);
            req
        };

        // title:

        assert_eq!(
            ids(r.get_tasks(GetTasksQuery::default()).await),
            vec![1, 2, 3, 4, 5, 6],
            "default"
        );

        assert_eq!(
            ids(r.get_tasks(req(|r| r.title = Some(String::from("first")))).await),
            vec![1],
            "title = \"first\""
        );

        assert_eq!(
            ids(r.get_tasks(req(|r| r.title = Some(String::from("done")))).await),
            vec![4, 5],
            "title = \"done\""
        );

        assert_eq!(
            ids(r.get_tasks(req(|r| r.title = Some(String::from("non-existent")))).await),
            Vec::<u64>::new(),
            "title = \"non-existent\""
        );

        // status:

        assert_eq!(
            ids(r.get_tasks(req(|r| r.status = Some(vec![Status::Pending]))).await),
            vec![1, 6],
            "status = [pending]"
        );

        assert_eq!(
            ids(r.get_tasks(req(|r| r.status = Some(vec![Status::Cancelled]))).await),
            vec![2, 3],
            "status = [cancelled]"
        );

        assert_eq!(
            ids(r.get_tasks(req(|r| r.status = Some(vec![Status::Cancelled, Status::Done]))).await),
            vec![2, 3, 4, 5],
            "status = [cancelled, done]"
        );

        assert_eq!(
            ids(r.get_tasks(req(|r| r.status = Some(vec![Status::Pending, Status::Cancelled, Status::Done]))).await),
            vec![1, 2, 3, 4, 5, 6],
            "status = [pending, cancelled, done]"
        );

        assert_eq!(
            ids(r.get_tasks(req(|r| r.status = Some(vec![]))).await),
            Vec::<u64>::new(),
            "status = []"
        );

        // title + status:

        assert_eq!(
            ids(r.get_tasks(req(|r| {
                r.status = Some(vec![Status::Pending, Status::Done]);
                r.title = Some(String::from("1"));
            })).await),
            vec![4, 6],
            "status = [pending, done] && title = \"1\""
        );
    }
}

