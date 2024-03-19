use std::rc::Rc;

use super::ListFilter;

#[derive(Debug, Default)]
pub struct TaskFilter {
    pub list_ids: Option<Rc<Vec<String>>>,
    pub task_ids: Option<Rc<Vec<String>>>,
}

impl From<&ListFilter> for TaskFilter {
    fn from(filter: &ListFilter) -> Self {
        TaskFilter {
            list_ids: filter.list_ids.clone(),
            ..Default::default()
        }
    }
}

impl From<ListFilter> for TaskFilter {
    fn from(filter: ListFilter) -> Self {
        TaskFilter {
            list_ids: filter.list_ids.clone(),
            ..Default::default()
        }
    }
}