mod normalize_title {
    use super::super::TaskService;

    #[test]
    fn rejects_titles_with_at_most_three_characters() {
        assert!(TaskService::normalize_title("abc").is_err());
        assert!(TaskService::normalize_title("  ab  ").is_err());
        assert!(TaskService::normalize_title("  日本  ").is_err());
    }

    #[test]
    fn accepts_titles_with_more_than_three_characters() {
        assert_eq!(TaskService::normalize_title("abcd").unwrap(), "abcd");
        assert_eq!(TaskService::normalize_title("  abcd  ").unwrap(), "abcd");
        assert_eq!(TaskService::normalize_title("  日本語a  ").unwrap(), "日本語a");
    }

    #[test]
    fn counts_unicode_characters_not_bytes() {
        assert!(TaskService::normalize_title("🙂🙂🙂").is_err());
        assert_eq!(TaskService::normalize_title("🙂🙂🙂🙂").unwrap(), "🙂🙂🙂🙂");
        assert!(TaskService::normalize_title("日本語").is_err());
        assert_eq!(TaskService::normalize_title("日本語a").unwrap(), "日本語a");
    }

    #[test]
    fn rejects_titles_longer_than_512_unicode_characters() {
        let title = "a".repeat(512);
        assert_eq!(TaskService::normalize_title(&title).unwrap(), title);

        let title = "a".repeat(513);
        assert!(TaskService::normalize_title(&title).is_err());

        let title = "🙂".repeat(512);
        assert_eq!(TaskService::normalize_title(&title).unwrap(), title);

        let title = "🙂".repeat(513);
        assert!(TaskService::normalize_title(&title).is_err());
    }
}

mod add_task {
    use mockall::predicate::eq;
    use assert_matches::assert_matches;
    use pretty_assertions::assert_eq;

    use crate::service::{
        model::{Status, Task, TaskID},
        port::repository::{self, MockTaskReader, MockTaskWriter},
    };

    use super::super::*;

    #[tokio::test]
    async fn add_task() {
        // given:
        let task_for_repository = Task::new("something to do", Status::Pending);
        let expected_task = Task::new("something to do", Status::Pending)
            .set_id(TaskID::from(123))
            .to_owned();

        // setup:
        let r = MockTaskReader::new();
        let mut writer = MockTaskWriter::new();
        writer.expect_add_task()
            .with(eq(task_for_repository))
            .once()
            .returning(|mut v| {
                v.set_id(TaskID::from(123));
                Ok(v.clone())
            });
        let svc = TaskService::new(Arc::new(r), Arc::new(writer));

        // when:
        let result = svc
            .add_task(AddTaskRequest {
                title: String::from("something to do"),
            })
            .await;

        // then:
        assert_eq!(result.unwrap(), expected_task);
    }

    #[tokio::test]
    async fn fail_when_task_is_a_duplicate() {
        // setup:
        let r = MockTaskReader::new();
        let mut writer = MockTaskWriter::new();
        writer.expect_add_task()
            .once()
            .returning(|_| Err(repository::AddError::NotUnique));
        let svc = TaskService::new(Arc::new(r), Arc::new(writer));

        // when:
        let result = svc
            .add_task(AddTaskRequest {
                title: String::from("something to do"),
            })
            .await;

        // then:
        assert_matches!(result.unwrap_err(), AddTaskError::Duplicate);
    }

    #[tokio::test]
    async fn fail_when_title_is_short() {
        // given:
        let task_for_repository = Task::new("somt", Status::Pending);

        // setup:
        let r = MockTaskReader::new();
        let mut writer = MockTaskWriter::new();
        writer.expect_add_task()
            .with(eq(task_for_repository))
            .once()
            .returning(|mut v| {
                v.set_id(TaskID::from(123));
                Ok(v.clone())
            });
        let svc = TaskService::new(Arc::new(r), Arc::new(writer));

        // when / then:
        let result = svc
            .add_task(AddTaskRequest {
                title: String::from("som"),
            })
            .await;
        assert_matches!(result.unwrap_err(), AddTaskError::InvalidRequest(_));

        let result = svc
            .add_task(AddTaskRequest {
                title: String::from("somt"),
            })
            .await;
        assert_eq!(result.is_ok(), true);
    }

    #[tokio::test]
    async fn trims_surrounding_whitespace_before_persisting() {
        // given:
        let task_for_repository = Task::new("something to do", Status::Pending);
        let expected_task = Task::new("something to do", Status::Pending)
            .set_id(TaskID::from(123))
            .to_owned();

        // setup:
        let r = MockTaskReader::new();
        let mut writer = MockTaskWriter::new();
        writer.expect_add_task()
            .with(eq(task_for_repository))
            .once()
            .returning(|mut v| {
                v.set_id(TaskID::from(123));
                Ok(v.clone())
            });
        let svc = TaskService::new(Arc::new(r), Arc::new(writer));

        // when:
        let result = svc
            .add_task(AddTaskRequest {
                title: String::from("  something to do  "),
            })
            .await;

        // then:
        assert_eq!(result.unwrap(), expected_task);
    }

    #[tokio::test]
    async fn fail_when_title_is_short_unicode() {
        // setup:
        let r = MockTaskReader::new();
        let w = MockTaskWriter::new();
        let svc = TaskService::new(Arc::new(r), Arc::new(w));

        // when:
        let result = svc
            .add_task(AddTaskRequest {
                title: String::from("🙂🙂🙂"),
            })
            .await;

        // then:
        assert_matches!(result.unwrap_err(), AddTaskError::InvalidRequest(_));
    }

    #[tokio::test]
    async fn fail_when_title_is_too_long() {
        // setup:
        let r = MockTaskReader::new();
        let w = MockTaskWriter::new();
        let svc = TaskService::new(Arc::new(r), Arc::new(w));

        // when:
        let result = svc
            .add_task(AddTaskRequest {
                title: "a".repeat(513),
            })
            .await;

        // then:
        assert_matches!(result.unwrap_err(), AddTaskError::InvalidRequest(_));
    }
}

mod get_task_by_id {
    use mockall::predicate::eq;
    use assert_matches::assert_matches;
    use pretty_assertions::assert_eq;

    use crate::service::{
        model::{Status, Task, TaskID},
        port::repository::{self, MockTaskReader, MockTaskWriter},
    };

    use super::super::*;

    #[tokio::test]
    async fn return_task() {
        // given:
        let id = TaskID::from(1234);
        let stored_task = Task::new("some task", Status::Done)
            .set_id(id)
            .to_owned();
        let expected_task = Task::new("some task", Status::Done)
            .set_id(id)
            .to_owned();

        // setup:
        let mut reader = MockTaskReader::new();
        let w = MockTaskWriter::new();
        reader.expect_get_task_by_id()
            .with(eq(id))
            .once()
            .returning(move |_| Ok(Some(stored_task.clone())));
        let svc = TaskService::new(Arc::new(reader), Arc::new(w));

        // when:
        let result = svc.get_task_by_id(id).await;

        // then:
        assert_eq!(result.unwrap(), Some(expected_task));
    }

    #[tokio::test]
    async fn return_none_when_task_not_found() {
        // given:
        let id = TaskID::from(1234);

        // setup:
        let mut reader = MockTaskReader::new();
        let w = MockTaskWriter::new();
        reader.expect_get_task_by_id()
            .with(eq(id))
            .once()
            .returning(move |_| Ok(None));
        let svc = TaskService::new(Arc::new(reader), Arc::new(w));

        // when:
        let result = svc.get_task_by_id(id).await;

        // then:
        assert_eq!(result.unwrap(), None);
    }

    #[tokio::test]
    async fn fail_on_reader_error() {
        // given:
        let id = TaskID::from(1234);

        // setup:
        let mut reader = MockTaskReader::new();
        let w = MockTaskWriter::new();
        reader.expect_get_task_by_id()
            .with(eq(id))
            .once()
            .returning(move |_| Err(repository::ReadError::DatabaseFailure));
        let svc = TaskService::new(Arc::new(reader), Arc::new(w));

        // when:
        let result = svc.get_task_by_id(id).await;

        // then:
        let err = result.unwrap_err();
        assert_matches!(err, GetTaskError::StorageError(_));

        let storage_err = std::error::Error::source(&err).unwrap();
        let repo_err = std::error::Error::source(storage_err).unwrap();
        assert_matches!(
            repo_err.downcast_ref::<repository::ReadError>(),
            Some(repository::ReadError::DatabaseFailure)
        );
    }
}

mod list_tasks {
    use mockall::predicate::eq;
    use assert_matches::assert_matches;
    use pretty_assertions::assert_eq;

    use crate::service::{
        model::{Status, Task},
        port::repository::{
            self, GetTasksQuery as RepositoryListTasksRequest, MockTaskReader, MockTaskWriter,
        },
    };

    use super::super::*;

    #[tokio::test]
    async fn return_list_of_tasks() {
        // given:
        let req = ListTasksRequest {
            title: Some("remind Joe".into()),
            ..ListTasksRequest::default()
        };
        let repository_query = RepositoryListTasksRequest {
            title: Some("remind Joe".into()),
            ..RepositoryListTasksRequest::default()
        };
        let stored_tasks = vec![
            Task::new("hello 1", Status::Done),
            Task::new("hello 2", Status::Cancelled),
            Task::new("hello 2", Status::Pending),
        ];
        let expected_tasks = vec![
            Task::new("hello 1", Status::Done),
            Task::new("hello 2", Status::Cancelled),
            Task::new("hello 2", Status::Pending),
        ];

        // setup:
        let mut reader = MockTaskReader::new();
        let w = MockTaskWriter::new();
        reader.expect_get_tasks()
            .with(eq(repository_query))
            .once()
            .returning(move |_| Ok(stored_tasks.clone()));
        let svc = TaskService::new(Arc::new(reader), Arc::new(w));

        // when:
        let result = svc.list_tasks(req).await;

        // then:
        assert_eq!(result.unwrap(), expected_tasks);
    }

    #[tokio::test]
    async fn return_empty_list() {
        // given:
        let req = ListTasksRequest {
            title: Some("remind Joe".into()),
            ..ListTasksRequest::default()
        };
        let repository_query = RepositoryListTasksRequest {
            title: Some("remind Joe".into()),
            ..RepositoryListTasksRequest::default()
        };

        // setup:
        let mut reader = MockTaskReader::new();
        let w = MockTaskWriter::new();
        reader.expect_get_tasks()
            .with(eq(repository_query))
            .once()
            .returning(|_| Ok(vec![]));
        let svc = TaskService::new(Arc::new(reader), Arc::new(w));

        // when:
        let result = svc.list_tasks(req).await;

        // then:
        assert_eq!(result.unwrap(), vec![]);
    }

    #[tokio::test]
    async fn fail_on_reader_error() {
        // given:
        let req = ListTasksRequest {
            title: Some("remind Joe".into()),
            ..ListTasksRequest::default()
        };
        let repository_query = RepositoryListTasksRequest {
            title: Some("remind Joe".into()),
            ..RepositoryListTasksRequest::default()
        };

        // setup:
        let mut reader = MockTaskReader::new();
        let w = MockTaskWriter::new();
        reader.expect_get_tasks()
            .with(eq(repository_query))
            .once()
            .returning(|_| Err(repository::ReadError::DatabaseFailure));
        let svc = TaskService::new(Arc::new(reader), Arc::new(w));

        // when:
        let result = svc.list_tasks(req).await;

        // then:
        let err = result.unwrap_err();
        assert_matches!(err, ListTasksError::StorageError(_));

        let storage_err = std::error::Error::source(&err).unwrap();
        let repo_err = std::error::Error::source(storage_err).unwrap();
        assert_matches!(
            repo_err.downcast_ref::<repository::ReadError>(),
            Some(repository::ReadError::DatabaseFailure)
        );
    }
}

mod rename_task {
    use mockall::predicate::eq;
    use pretty_assertions::assert_eq;
    use assert_matches::assert_matches;

    use crate::service::{
        model::{Status, Task, TaskID},
        port::repository::{self, MockTaskReader, MockTaskWriter},
    };

    use super::super::*;

    #[tokio::test]
    pub async fn rename_task() {
        // given:
        let id = TaskID::from(1234);
        let stored_task = Task::new("some task", Status::Done)
            .set_id(id)
            .to_owned();
        let task_for_update = Task::new("new title", Status::Done)
            .set_id(id)
            .to_owned();
        let expected_task = Task::new("new title", Status::Done)
            .set_id(id)
            .to_owned();
        let repository_response = Task::new("new title", Status::Done)
            .set_id(id)
            .to_owned();

        // setup:
        let mut reader = MockTaskReader::new();
        let mut writer = MockTaskWriter::new();
        reader.expect_get_task_by_id()
            .with(eq(id))
            .once()
            .returning(move |_| Ok(Some(stored_task.clone())));
        writer.expect_update_task()
            .with(eq(task_for_update))
            .once()
            .returning(move |_| Ok(repository_response.clone()));
        let svc = TaskService::new(Arc::new(reader), Arc::new(writer));

        // when:
        let result = svc.rename_task(id, String::from("new title")).await;

        // then:
        assert_eq!(result.unwrap(), expected_task);
    }

    #[tokio::test]
    pub async fn fail_when_task_not_found() {
        // given:
        let id = TaskID::from(1234);

        // setup:
        let mut reader = MockTaskReader::new();
        let w = MockTaskWriter::new();
        reader.expect_get_task_by_id()
            .with(eq(id))
            .once()
            .returning(move |_| Ok(None));
        let svc = TaskService::new(Arc::new(reader), Arc::new(w));

        // when:
        let result = svc.rename_task(id, String::from("new title")).await;

        // then:
        assert_matches!(result.unwrap_err(), RenameTaskError::NotFound);
    }

    #[tokio::test]
    pub async fn trims_surrounding_whitespace_before_persisting() {
        // given:
        let id = TaskID::from(1234);
        let stored_task = Task::new("some task", Status::Done)
            .set_id(id)
            .to_owned();
        let task_for_update = Task::new("new title", Status::Done)
            .set_id(id)
            .to_owned();
        let expected_task = Task::new("new title", Status::Done)
            .set_id(id)
            .to_owned();
        let repository_response = Task::new("new title", Status::Done)
            .set_id(id)
            .to_owned();

        // setup:
        let mut reader = MockTaskReader::new();
        let mut writer = MockTaskWriter::new();
        reader.expect_get_task_by_id()
            .with(eq(id))
            .once()
            .returning(move |_| Ok(Some(stored_task.clone())));
        writer.expect_update_task()
            .with(eq(task_for_update))
            .once()
            .returning(move |_| Ok(repository_response.clone()));
        let svc = TaskService::new(Arc::new(reader), Arc::new(writer));

        // when:
        let result = svc.rename_task(id, String::from("  new title  ")).await;

        // then:
        assert_eq!(result.unwrap(), expected_task);
    }

    #[tokio::test]
    pub async fn fail_if_name_is_too_short() {
        // setup:
        let r = MockTaskReader::new();
        let w = MockTaskWriter::new();
        let svc = TaskService::new(Arc::new(r), Arc::new(w));

        // when:
        let id = TaskID::from(1234);
        let result = svc.rename_task(id, String::from("new")).await;

        // then:
        assert_matches!(result.unwrap_err(), RenameTaskError::InvalidRequest(_));
    }

    #[tokio::test]
    pub async fn fail_if_name_is_too_short_unicode() {
        // setup:
        let r = MockTaskReader::new();
        let w = MockTaskWriter::new();
        let svc = TaskService::new(Arc::new(r), Arc::new(w));

        // when:
        let id = TaskID::from(1234);
        let result = svc.rename_task(id, String::from("日本語")).await;

        // then:
        assert_matches!(result.unwrap_err(), RenameTaskError::InvalidRequest(_));
    }

    #[tokio::test]
    pub async fn fail_if_name_is_too_long() {
        // setup:
        let r = MockTaskReader::new();
        let w = MockTaskWriter::new();
        let svc = TaskService::new(Arc::new(r), Arc::new(w));

        // when:
        let id = TaskID::from(1234);
        let result = svc.rename_task(id, "a".repeat(513)).await;

        // then:
        assert_matches!(result.unwrap_err(), RenameTaskError::InvalidRequest(_));
    }

    #[tokio::test]
    pub async fn fail_if_task_becomes_a_pending_duplicate() {
        // given:
        let id = TaskID::from(1234);
        let stored_task = Task::new("some task", Status::Done)
            .set_id(id)
            .to_owned();
        let task_for_update = Task::new("new title", Status::Done)
            .set_id(id)
            .to_owned();

        // setup:
        let mut reader = MockTaskReader::new();
        let mut writer = MockTaskWriter::new();
        reader.expect_get_task_by_id()
            .with(eq(id))
            .once()
            .returning(move |_| Ok(Some(stored_task.clone())));
        writer.expect_update_task()
            .with(eq(task_for_update))
            .once()
            .returning(|_| Err(repository::UpdateError::NotUnique));
        let svc = TaskService::new(Arc::new(reader), Arc::new(writer));

        // when:
        let result = svc.rename_task(id, String::from("new title")).await;

        // then:
        assert_matches!(result.unwrap_err(), RenameTaskError::Duplicate);
    }
}

mod change_status {
    use mockall::predicate::eq;
    use pretty_assertions::assert_eq;
    use assert_matches::assert_matches;

    use crate::service::{
        model::{Status, Task, TaskID},
        port::repository::{self, MockTaskReader, MockTaskWriter},
    };

    use super::super::*;

    #[tokio::test]
    pub async fn change_status() {
        // given:
        let id = TaskID::from(1234);
        let stored_task = Task::new("some task", Status::Pending)
            .set_id(id)
            .to_owned();
        let task_for_update = Task::new("some task", Status::Done)
            .set_id(id)
            .to_owned();
        let expected_task = Task::new("some task", Status::Done)
            .set_id(id)
            .to_owned();
        let repository_response = Task::new("some task", Status::Done)
            .set_id(id)
            .to_owned();

        // setup:
        let mut reader = MockTaskReader::new();
        let mut writer = MockTaskWriter::new();
        reader.expect_get_task_by_id()
            .with(eq(id))
            .once()
            .returning(move |_| Ok(Some(stored_task.clone())));
        writer.expect_update_task()
            .with(eq(task_for_update))
            .once()
            .returning(move |_| Ok(repository_response.clone()));
        let svc = TaskService::new(Arc::new(reader), Arc::new(writer));

        // when:
        let result = svc.change_status(id, Status::Done).await;

        // then:
        assert_eq!(result.unwrap(), expected_task);
    }

    #[tokio::test]
    pub async fn fail_when_task_not_found() {
        // given:
        let id = TaskID::from(1234);

        // setup:
        let mut reader = MockTaskReader::new();
        let w = MockTaskWriter::new();
        reader.expect_get_task_by_id()
            .with(eq(id))
            .once()
            .returning(move |_| Ok(None));
        let svc = TaskService::new(Arc::new(reader), Arc::new(w));

        // when:
        let result = svc.change_status(id, Status::Done).await;

        // then:
        assert_matches!(result.unwrap_err(), ChangeStatusError::NotFound);
    }

    #[tokio::test]
    pub async fn fail_if_task_becomes_a_pending_duplicate() {
        // given:
        let id = TaskID::from(1234);
        let stored_task = Task::new("some task", Status::Done)
            .set_id(id)
            .to_owned();
        let task_for_update = Task::new("some task", Status::Pending)
            .set_id(id)
            .to_owned();

        // setup:
        let mut reader = MockTaskReader::new();
        let mut writer = MockTaskWriter::new();
        reader.expect_get_task_by_id()
            .with(eq(id))
            .once()
            .returning(move |_| Ok(Some(stored_task.clone())));
        writer.expect_update_task()
            .with(eq(task_for_update))
            .once()
            .returning(|_| Err(repository::UpdateError::NotUnique));
        let svc = TaskService::new(Arc::new(reader), Arc::new(writer));

        // when:
        let result = svc.change_status(id, Status::Pending).await;

        // then:
        assert_matches!(result.unwrap_err(), ChangeStatusError::Duplicate);
    }
}
