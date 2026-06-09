mod add_task {
    use mockall::predicate::eq;
    use assert_matches::assert_matches;
    use pretty_assertions::assert_eq;

    use crate::service::{model::{Status, Task, TaskID}, repository::{MockTaskReader, MockTaskWriter}};

    use super::super::*;

    #[tokio::test]
    async fn test_add_task() {
        let expected_add_task = Task::new("something to do", Status::Pending);

        let r = MockTaskReader::new();
        let mut w = MockTaskWriter::new();
        w.expect_add_task()
            .with(eq(expected_add_task.clone()))
            .once()
            .returning(|mut v| {
                v.set_id(TaskID::from(123));
                Ok(v.clone())
            });

        let result = TaskService::new(Arc::new(r), Arc::new(w))
            .add_task(AddTaskRequest { title: String::from("something to do") }).await;

        let mut expected_result = expected_add_task.clone();
        expected_result.set_id(TaskID::from(123));

        assert_eq!(result.unwrap(), expected_result)
    }

    #[tokio::test]
    async fn test_add_task_duplicate() {
        let r = MockTaskReader::new();
        let mut w = MockTaskWriter::new();
        w.expect_add_task()
            .once()
            .returning(|_| Err(TaskAddRepositoryError::NotUnique));

        let result = TaskService::new(Arc::new(r), Arc::new(w))
            .add_task(AddTaskRequest { title: String::from("something to do") }).await;

        assert_matches!(result.unwrap_err(), AddTaskError::Duplicate)
    }

    #[tokio::test]
    async fn test_add_task_short_name() {
        let r = MockTaskReader::new();
        let mut w = MockTaskWriter::new();

        w.expect_add_task()
            .with(eq(Task::new("somt", Status::Pending)))
            .once()
            .returning(|mut v| {
                v.set_id(TaskID::from(123));
                Ok(v.clone())
            })
        ;

        let svc = TaskService::new(Arc::new(r), Arc::new(w));

        let result = svc.add_task(AddTaskRequest { title: String::from("som") }).await;
        assert_matches!(result.unwrap_err(), AddTaskError::InvalidRequest(_));

        let result = svc.add_task(AddTaskRequest { title: String::from("somt") }).await;
        assert_eq!(result.is_ok(), true);
    }
}

mod get_task_by_id {
    use mockall::predicate::eq;
    use assert_matches::assert_matches;
    use pretty_assertions::assert_eq;

    use crate::service::{model::{Status, Task, TaskID}, repository::{MockTaskReader, MockTaskWriter}};

    use super::super::*;

    #[tokio::test]
    async fn test_get_task_by_id() {
        let id = TaskID::from(1234);
        let expected_task = Task::new("some task", Status::Done)
            .set_id(TaskID::from(1234))
            .to_owned();

        let mut r = MockTaskReader::new();
        let w = MockTaskWriter::new();

        let t = expected_task.clone();
        r.expect_get_task_by_id()
            .with(eq(id))
            .once()
            .returning(move |_| Ok(Some(t.clone())))
        ;

        let svc = TaskService::new(Arc::new(r), Arc::new(w));

        let result = svc.get_task_by_id(id).await;
        assert_eq!(result.unwrap(), Some(expected_task));
    }

    #[tokio::test]
    async fn test_get_task_by_id_not_found() {
        let id = TaskID::from(1234);

        let mut r = MockTaskReader::new();
        let w = MockTaskWriter::new();

        r.expect_get_task_by_id()
            .with(eq(id))
            .once()
            .returning(move |_| Ok(None))
        ;

        let svc = TaskService::new(Arc::new(r), Arc::new(w));

        let result = svc.get_task_by_id(id).await;
        assert_eq!(result.unwrap(), None);
    }

    #[tokio::test]
    async fn test_get_task_by_id_error() {
        let id = TaskID::from(1234);

        let mut r = MockTaskReader::new();
        let w = MockTaskWriter::new();

        r.expect_get_task_by_id()
            .with(eq(id))
            .once()
            .returning(move |_| Err(TaskReadRepositoryError::DatabaseFailure))
        ;

        let svc = TaskService::new(Arc::new(r), Arc::new(w));

        let result = svc.get_task_by_id(id).await;
        assert_matches!(result.unwrap_err(), GetTaskError::StorareError(TaskReadRepositoryError::DatabaseFailure));
    }
}

mod list_tasks {
    use mockall::predicate::eq;
    use assert_matches::assert_matches;
    use pretty_assertions::assert_eq;

    use crate::service::{model::{Status, Task}, repository::{MockTaskReader, MockTaskWriter}};

    use super::super::*;

    #[tokio::test]
    async fn test_list_tasks() {
        let mut filter = TaskFilter::default();
        filter.title = Some("remind Joe".into());

        let mut r = MockTaskReader::new();
        let w = MockTaskWriter::new();

        r.expect_get_tasks()
            .with(eq(filter.clone()))
            .once()
            .returning(|_| Ok(vec![
                Task::new("hello 1", Status::Done),
                Task::new("hello 2", Status::Cancelled),
                Task::new("hello 2", Status::Pending),
            ]))
        ;

        let svc = TaskService::new(Arc::new(r), Arc::new(w));

        let result = svc.list_tasks(filter).await;
        assert_eq!(result.unwrap(), vec![
            Task::new("hello 1", Status::Done),
            Task::new("hello 2", Status::Cancelled),
            Task::new("hello 2", Status::Pending),
        ]);
    }

    #[tokio::test]
    async fn test_list_tasks_empty() {
        let mut filter = TaskFilter::default();
        filter.title = Some("remind Joe".into());

        let mut r = MockTaskReader::new();
        let w = MockTaskWriter::new();

        r.expect_get_tasks()
            .with(eq(filter.clone()))
            .once()
            .returning(|_| Ok(vec![]))
        ;

        let svc = TaskService::new(Arc::new(r), Arc::new(w));

        let result = svc.list_tasks(filter).await;
        assert_eq!(result.unwrap(), vec![]);
    }

    #[tokio::test]
    async fn test_list_tasks_err() {
        let mut filter = TaskFilter::default();
        filter.title = Some("remind Joe".into());

        let mut r = MockTaskReader::new();
        let w = MockTaskWriter::new();

        r.expect_get_tasks()
            .with(eq(filter.clone()))
            .once()
            .returning(|_| Err(TaskReadRepositoryError::DatabaseFailure))
        ;

        let svc = TaskService::new(Arc::new(r), Arc::new(w));

        let result = svc.list_tasks(filter).await;
        assert_matches!(result.unwrap_err(), ListTasksError::StorareError(TaskReadRepositoryError::DatabaseFailure));
    }
}

mod rename_task {
    use mockall::predicate::eq;
    use pretty_assertions::assert_eq;
    use assert_matches::assert_matches;

    use crate::service::{model::{Status, Task}, repository::{MockTaskReader, MockTaskWriter, TaskUpdateRepositoryError}};

    use super::super::*;

    #[tokio::test]
    pub async fn rename_task() {
        let id = TaskID::from(1234);
        let existing_task = Task::new("some task", Status::Done)
            .set_id(TaskID::from(1234))
            .to_owned();

        let expected_task = existing_task.clone().set_title("new title".into()).to_owned();

        let mut r = MockTaskReader::new();
        let mut w = MockTaskWriter::new();

        let t = existing_task.clone();
        r.expect_get_task_by_id()
            .with(eq(id))
            .once()
            .returning(move |_| Ok(Some(t.clone())))
        ;

        let t = expected_task.clone();
        w.expect_update_task()
            .with(eq(expected_task.clone()))
            .once()
            .returning(move |_| Ok(t.clone()))
        ;

        let svc = TaskService::new(Arc::new(r), Arc::new(w));

        let result = svc.rename_task(id, String::from("new title")).await;
        assert_eq!(result.unwrap(), expected_task);
    }

    #[tokio::test]
    pub async fn return_not_found() {
        let id = TaskID::from(1234);

        let mut r = MockTaskReader::new();
        let w = MockTaskWriter::new();

        r.expect_get_task_by_id()
            .with(eq(id))
            .once()
            .returning(move |_| Ok(None))
        ;

        let svc = TaskService::new(Arc::new(r), Arc::new(w));

        let result = svc.rename_task(id, String::from("new title")).await;
        assert_matches!(result.unwrap_err(), RenameTaskError::NotFound);
    }


    #[tokio::test]
    pub async fn short_name() {
        let id = TaskID::from(1234);

        let r = MockTaskReader::new();
        let w = MockTaskWriter::new();

        let svc = TaskService::new(Arc::new(r), Arc::new(w));

        let result = svc.rename_task(id, String::from("new")).await;
        assert_matches!(result.unwrap_err(), RenameTaskError::InvalidRequest(_));
    }

    #[tokio::test]
    pub async fn duplicate() {
        let id = TaskID::from(1234);
        let existing_task = Task::new("some task", Status::Done)
            .set_id(TaskID::from(1234))
            .to_owned();

        let expected_task = existing_task.clone().set_title("new title".into()).to_owned();

        let mut r = MockTaskReader::new();
        let mut w = MockTaskWriter::new();

        let t = existing_task.clone();
        r.expect_get_task_by_id()
            .with(eq(id))
            .once()
            .returning(move |_| Ok(Some(t.clone())))
        ;

        w.expect_update_task()
            .with(eq(expected_task.clone()))
            .once()
            .returning(move |_| Err(TaskUpdateRepositoryError::NotUnique))
        ;

        let svc = TaskService::new(Arc::new(r), Arc::new(w));

        let result = svc.rename_task(id, String::from("new title")).await;
        assert_matches!(result.unwrap_err(), RenameTaskError::Duplicate);
    }
}

mod change_status {
    use mockall::predicate::eq;
    use pretty_assertions::assert_eq;
    use assert_matches::assert_matches;

    use crate::service::{model::{Status, Task}, repository::{MockTaskReader, MockTaskWriter, TaskUpdateRepositoryError}};

    use super::super::*;

    #[tokio::test]
    pub async fn change_status() {
        let id = TaskID::from(1234);
        let existing_task = Task::new("some task", Status::Pending)
            .set_id(TaskID::from(1234))
            .to_owned();

        let expected_task = existing_task.clone().set_status(Status::Done).to_owned();

        let mut r = MockTaskReader::new();
        let mut w = MockTaskWriter::new();

        let t = existing_task.clone();
        r.expect_get_task_by_id()
            .with(eq(id))
            .once()
            .returning(move |_| Ok(Some(t.clone())))
        ;

        let t = expected_task.clone();
        w.expect_update_task()
            .with(eq(expected_task.clone()))
            .once()
            .returning(move |_| Ok(t.clone()))
        ;

        let svc = TaskService::new(Arc::new(r), Arc::new(w));

        let result = svc.change_status(id, Status::Done).await;
        assert_eq!(result.unwrap(), expected_task);
    }

    #[tokio::test]
    pub async fn return_not_found() {
        let id = TaskID::from(1234);

        let mut r = MockTaskReader::new();
        let w = MockTaskWriter::new();

        r.expect_get_task_by_id()
            .with(eq(id))
            .once()
            .returning(move |_| Ok(None))
        ;

        let svc = TaskService::new(Arc::new(r), Arc::new(w));

        let result = svc.change_status(id, Status::Done).await;
        assert_matches!(result.unwrap_err(), ChangeStatusError::NotFound);
    }

    #[tokio::test]
    pub async fn duplicate() {
        let id = TaskID::from(1234);
        let existing_task = Task::new("some task", Status::Done)
            .set_id(TaskID::from(1234))
            .to_owned();

        let expected_task = existing_task.clone().set_status(Status::Pending).to_owned();

        let mut r = MockTaskReader::new();
        let mut w = MockTaskWriter::new();

        let t = existing_task.clone();
        r.expect_get_task_by_id()
            .with(eq(id))
            .once()
            .returning(move |_| Ok(Some(t.clone())))
        ;

        w.expect_update_task()
            .with(eq(expected_task.clone()))
            .once()
            .returning(move |_| Err(TaskUpdateRepositoryError::NotUnique))
        ;

        let svc = TaskService::new(Arc::new(r), Arc::new(w));

        let result = svc.change_status(id, Status::Pending).await;
        assert_matches!(result.unwrap_err(), ChangeStatusError::Duplicate);
    }
}
