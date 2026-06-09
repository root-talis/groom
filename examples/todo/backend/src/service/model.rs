use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct Task {
    id:     Option<TaskID>,
    title:  String,
    status: Status,
}

impl Task {
    pub fn new<T: Into<String>>(title: T, status: Status) -> Self {
        Self { id: None, title: title.into(), status }
    }

    pub fn id(&self) -> Option<TaskID> {
        self.id
    }

    pub fn set_id(&mut self, id: TaskID) -> &mut Self {
        self.id = Some(id);
        self
    }

    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub fn set_title(&mut self, title: String) -> &mut Self {
        self.title = title;
        self
    }

    pub fn status(&self) -> Status {
        self.status
    }

    pub fn set_status(&mut self, status: Status) -> &mut Self {
        self.status = status;
        self
    }
}

#[derive(Serialize, Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct TaskID(u64);

impl TaskID {
    pub fn value(&self) -> u64 {
        self.0
    }
}

impl From<u64> for TaskID {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<TaskID> for u64 {
    fn from(val: TaskID) -> Self {
        val.0
    }
}

#[derive(
    Debug, Clone, Copy,
    PartialEq, Eq, PartialOrd, Ord,
    Serialize, Deserialize, ToSchema
)]
pub enum Status {
    Pending,
    Done,
    Cancelled
}
