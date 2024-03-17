use std::rc::Rc;

use super::{FolderFilter, TaskFilter};

#[derive(Debug, Default)]
pub struct ListFilter {
    pub folder_ids: Option<Rc<Vec<String>>>,
    pub list_ids: Option<Rc<Vec<String>>>,
    pub archived: bool
}

impl From<&FolderFilter> for ListFilter {
    fn from(filter: &FolderFilter) -> Self {
        ListFilter {
            folder_ids: filter.folder_ids.clone(),
            ..Default::default()
        }
    }
}

impl From<FolderFilter> for ListFilter {
    fn from(filter: FolderFilter) -> Self {
        ListFilter {
            folder_ids: filter.folder_ids.clone(),
            ..Default::default()
        }
    }
}

impl From<&TaskFilter> for ListFilter {
    fn from(filter: &TaskFilter) -> Self {
        ListFilter {
            list_ids: filter.list_ids.clone(),
            ..Default::default()
        }
    }
}

impl From<TaskFilter> for ListFilter {
    fn from(filter: TaskFilter) -> Self {
        ListFilter {
            list_ids: filter.list_ids.clone(),
            ..Default::default()
        }
    }
}