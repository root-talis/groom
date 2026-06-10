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
    // given:
    let tasks = vec![
        Task::new("do something", Status::Pending),
        Task::new("do something else", Status::Pending),
        Task::new("don't do this", Status::Cancelled),
        Task::new("already done that", Status::Done),
    ];

    // setup:
    let r = InMemoryTaskRepository::new();
    let mut ids = BTreeSet::new();

    for t in &tasks {
        // when:
        let added_task = r
            .add_task(Task::new(t.title(), t.status()))
            .await
            .expect("call to `add_task` should have succeeded");

        // then:
        assert_eq!(added_task.title(), t.title());
        assert_eq!(added_task.status(), t.status());
        ids.insert(added_task.id().expect("added task should have an id"));
    }

    assert_eq!(ids.len(), tasks.len(), "should return unique id for each task");
}

#[tokio::test]
async fn should_add_duplicates_if_not_pending() {
    // given:
    let tasks = vec![
        Task::new("do something", Status::Cancelled),
        Task::new("do something", Status::Cancelled),
        Task::new("do something", Status::Done),
        Task::new("do something", Status::Done),
        Task::new("do something", Status::Pending),
    ];

    // setup:
    let r = InMemoryTaskRepository::new();
    let mut ids = BTreeSet::new();

    for t in &tasks {
        // when:
        let added_task = r
            .add_task(Task::new(t.title(), t.status()))
            .await
            .expect("call to `add_task` should have succeeded");

        // then:
        assert_eq!(added_task.title(), t.title());
        assert_eq!(added_task.status(), t.status());
        ids.insert(added_task.id().expect("added task should have an id"));
    }

    assert_eq!(ids.len(), tasks.len(), "should return unique id for each task");
}

#[tokio::test]
async fn should_not_add_pending_duplicates() {
    // setup:
    let r = InMemoryTaskRepository::new();
    r.add_task(Task::new("do something", Status::Pending))
        .await
        .expect("first call to `add_task` should have succeeded");

    // when:
    let result = r.add_task(Task::new("do something", Status::Pending)).await;

    // then:
    assert_matches!(result, Err(AddError::NotUnique));
}

#[tokio::test]
async fn should_update_tasks() {
    // given:
    let tasks = vec![
        Task::new("do something", Status::Pending),
        Task::new("do something else", Status::Pending),
        Task::new("don't do this", Status::Cancelled),
        Task::new("already done that", Status::Done),
    ];
    let new_statuses = vec![Status::Done, Status::Cancelled, Status::Pending, Status::Done];
    let new_titles: Vec<String> =
        (0..tasks.len()).map(|v| format!("Task #{v}: do something.")).collect();

    // setup:
    let repo = InMemoryTaskRepository::new();
    let mut task_ids = Vec::new();
    for t in &tasks {
        let added_task = repo
            .add_task(Task::new(t.title(), t.status()))
            .await
            .expect("call to `add_task` should have succeeded");
        task_ids.push(added_task.id().expect("added task should have an id"));
    }

    for i in 0..tasks.len() {
        let id = task_ids[i];
        let task_for_update = Task::new(&new_titles[i], new_statuses[i])
            .set_id(id)
            .to_owned();
        let expected_task = Task::new(&new_titles[i], new_statuses[i])
            .set_id(id)
            .to_owned();

        // when:
        let updated_task = repo
            .update_task(task_for_update)
            .await
            .expect("call to `update_task` should have succeeded");
        let refetched_task = repo
            .get_task_by_id(id)
            .await
            .expect("call to `get_task_by_id` should have succeeded")
            .expect("result of `get_task_by_id` should be not None");

        // then:
        assert_eq!(updated_task, expected_task);
        assert_eq!(refetched_task, expected_task);
    }
}

#[tokio::test]
async fn should_forbid_duplicates_on_update_if_pending() {
    // setup:
    let r = InMemoryTaskRepository::new();
    let first_task_id = r
        .add_task(Task::new("first task", Status::Pending))
        .await
        .expect("call to `add_task` should have succeeded")
        .id()
        .expect("added task should have an id");
    let cancelled_task_1_id = r
        .add_task(Task::new("cancelled task 1", Status::Cancelled))
        .await
        .expect("call to `add_task` should have succeeded")
        .id()
        .expect("added task should have an id");
    let cancelled_task_2_id = r
        .add_task(Task::new("cancelled task 2", Status::Cancelled))
        .await
        .expect("call to `add_task` should have succeeded")
        .id()
        .expect("added task should have an id");
    let done_task_1_id = r
        .add_task(Task::new("done task 1", Status::Done))
        .await
        .expect("call to `add_task` should have succeeded")
        .id()
        .expect("added task should have an id");
    let done_task_2_id = r
        .add_task(Task::new("done task 2", Status::Done))
        .await
        .expect("call to `add_task` should have succeeded")
        .id()
        .expect("added task should have an id");
    let pending_task_1_id = r
        .add_task(Task::new("pending task 1", Status::Pending))
        .await
        .expect("call to `add_task` should have succeeded")
        .id()
        .expect("added task should have an id");

    // when / then:
    assert_eq!(
        r.update_task(
            Task::new("first task", Status::Pending)
                .set_id(first_task_id)
                .to_owned()
        )
        .await
        .is_ok(),
        true,
        "should save unmodified pending task"
    );

    assert_matches!(
        r.update_task(
            Task::new("pending task 1", Status::Pending)
                .set_id(first_task_id)
                .to_owned()
        )
        .await,
        Err(UpdateError::NotUnique),
        "should not save pending task with the title of another pending task"
    );

    assert_eq!(
        r.update_task(
            Task::new("cancelled task 1", Status::Pending)
                .set_id(first_task_id)
                .to_owned()
        )
        .await
        .is_ok(),
        true,
        "should allow duplicated cancelled tasks"
    );

    assert_eq!(
        r.update_task(
            Task::new("done task 1", Status::Pending)
                .set_id(first_task_id)
                .to_owned()
        )
        .await
        .is_ok(),
        true,
        "should allow duplicated done tasks"
    );

    assert_eq!(
        r.update_task(
            Task::new("pending task 1", Status::Cancelled)
                .set_id(first_task_id)
                .to_owned()
        )
        .await
        .is_ok(),
        true,
        "should save cancelled task with the title of another pending task"
    );

    assert_eq!(
        r.update_task(
            Task::new("pending task 1", Status::Done)
                .set_id(first_task_id)
                .to_owned()
        )
        .await
        .is_ok(),
        true,
        "should save done task with the title of another pending task"
    );

    let expected_tasks = vec![
        Task::new("pending task 1", Status::Done)
            .set_id(first_task_id)
            .to_owned(),
        Task::new("cancelled task 1", Status::Cancelled)
            .set_id(cancelled_task_1_id)
            .to_owned(),
        Task::new("cancelled task 2", Status::Cancelled)
            .set_id(cancelled_task_2_id)
            .to_owned(),
        Task::new("done task 1", Status::Done)
            .set_id(done_task_1_id)
            .to_owned(),
        Task::new("done task 2", Status::Done)
            .set_id(done_task_2_id)
            .to_owned(),
        Task::new("pending task 1", Status::Pending)
            .set_id(pending_task_1_id)
            .to_owned(),
    ];

    let read_tasks = r.get_tasks(GetTasksQuery::default()).await;
    assert_eq!(read_tasks.is_ok(), true, "should read all tasks");
    assert_eq!(read_tasks.unwrap(), expected_tasks, "should read tasks as they were saved");
}

#[tokio::test]
async fn should_sort() {
    // setup:
    let r = InMemoryTaskRepository::new();
    r.add_task(Task::new("first task", Status::Pending))
        .await
        .unwrap();
    r.add_task(Task::new("cancelled task 1", Status::Cancelled))
        .await
        .unwrap();
    r.add_task(Task::new("cancelled task 2", Status::Cancelled))
        .await
        .unwrap();
    r.add_task(Task::new("done task 1", Status::Done))
        .await
        .unwrap();
    r.add_task(Task::new("done task 2", Status::Done))
        .await
        .unwrap();
    r.add_task(Task::new("pending task 1", Status::Pending))
        .await
        .unwrap();

    let ids = |r: Result<Vec<Task>, ReadError>| -> Vec<u64> {
        r.unwrap().iter().map(|v| v.id().unwrap().value()).collect()
    };

    let req = |f: fn(req: &mut GetTasksQuery) -> ()| {
        let mut req = GetTasksQuery::default();
        f(&mut req);
        req
    };

    // when / then: by id
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

    // when / then: by title
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

    // when / then: by status
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
    // setup:
    let r = InMemoryTaskRepository::new();
    r.add_task(Task::new("first task", Status::Pending))
        .await
        .unwrap();
    r.add_task(Task::new("cancelled task 1", Status::Cancelled))
        .await
        .unwrap();
    r.add_task(Task::new("cancelled task 2", Status::Cancelled))
        .await
        .unwrap();
    r.add_task(Task::new("a task that has been done 1", Status::Done))
        .await
        .unwrap();
    r.add_task(Task::new("done task 2", Status::Done))
        .await
        .unwrap();
    r.add_task(Task::new("pending task 1", Status::Pending))
        .await
        .unwrap();

    let ids = |r: Result<Vec<Task>, ReadError>| -> Vec<u64> {
        r.unwrap().iter().map(|v| v.id().unwrap().value()).collect()
    };

    let req = |f: fn(req: &mut GetTasksQuery) -> ()| {
        let mut req = GetTasksQuery::default();
        f(&mut req);
        req
    };

    // when / then: title
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
        ids(
            r.get_tasks(req(|r| r.title = Some(String::from("non-existent"))))
                .await
        ),
        Vec::<u64>::new(),
        "title = \"non-existent\""
    );

    // when / then: status
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
        ids(
            r.get_tasks(req(|r| r.status = Some(vec![Status::Cancelled, Status::Done])))
                .await
        ),
        vec![2, 3, 4, 5],
        "status = [cancelled, done]"
    );
    assert_eq!(
        ids(
            r.get_tasks(req(|r| {
                r.status = Some(vec![Status::Pending, Status::Cancelled, Status::Done])
            }))
            .await
        ),
        vec![1, 2, 3, 4, 5, 6],
        "status = [pending, cancelled, done]"
    );
    assert_eq!(
        ids(r.get_tasks(req(|r| r.status = Some(vec![]))).await),
        Vec::<u64>::new(),
        "status = []"
    );

    // when / then: title + status
    assert_eq!(
        ids(
            r.get_tasks(req(|r| {
                r.status = Some(vec![Status::Pending, Status::Done]);
                r.title = Some(String::from("1"));
            }))
            .await
        ),
        vec![4, 6],
        "status = [pending, done] && title = \"1\""
    );
}
