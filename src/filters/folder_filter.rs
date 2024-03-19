use std::rc::Rc;

use super::{ListFilter, SpaceFilter};

#[derive(Debug, Default)]
pub struct FolderFilter {
    pub space_ids: Option<Rc<Vec<String>>>,
    pub folder_ids: Option<Rc<Vec<String>>>,
    pub archived: bool
}

impl From<&SpaceFilter> for FolderFilter {
    fn from(filter: &SpaceFilter) -> Self {
        FolderFilter {
            space_ids: filter.space_ids.clone(),
            ..Default::default()
        }
    }
}

impl From<SpaceFilter> for FolderFilter {
    fn from(filter: SpaceFilter) -> Self {
        FolderFilter {
            space_ids: filter.space_ids.clone(),
            ..Default::default()
        }
    }
}

impl From<ListFilter> for FolderFilter {
    fn from(filter: ListFilter) -> Self {
        FolderFilter {
            folder_ids: filter.folder_ids.clone(),
            ..Default::default()
        }
    }
}

impl From<&ListFilter> for FolderFilter {
    fn from(filter: &ListFilter) -> Self {
        FolderFilter {
            folder_ids: filter.folder_ids.clone(),
            ..Default::default()
        }
    }
}
