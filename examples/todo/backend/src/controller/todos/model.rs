use groom_macros::DTO;
use serde::Deserialize;
use utoipa::ToSchema;

use crate::service::{model::{Status, Task}, repository::{Order, TaskOrderField}};

//
// TaskViewModel
//

#[DTO(response)]
pub struct TaskViewModel {
    pub id:     u64,
    pub title:  String,
    pub status: Status,
}

impl TryFrom<Task> for TaskViewModel {
    type Error = ();
    
    fn try_from(t: Task) -> Result<Self, Self::Error> {
        Ok(TaskViewModel {
            id: if let Some(id) = t.id() {
                    id.value()
                } else {
                    tracing::error!("task_id is expected to be set");
                    return Err(())
                },
            title: t.title(),
            status: t.status(),
        })
    }
}


//
// TasksSortBy
//

#[derive(Default, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum TasksSortBy {
    #[default]
    Id,
    Title,
    Status
}

impl Into<TaskOrderField> for TasksSortBy {
    fn into(self) -> TaskOrderField {
        match self {
            TasksSortBy::Id     => TaskOrderField::Id,
            TasksSortBy::Title  => TaskOrderField::Title,
            TasksSortBy::Status => TaskOrderField::Status,
        }
    }
}

//
// SortDirection
//

#[derive(Default, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum SortDirection {
    #[default]
    Asc,
    Desc
}

impl Into<Order> for SortDirection {
    fn into(self) -> Order {
        match self {
            SortDirection::Asc  => Order::Ascending,
            SortDirection::Desc => Order::Descending,
        }
    }
}
