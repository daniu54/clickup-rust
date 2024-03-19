use std::rc::Rc;

use super::{FolderFilter, TeamFilter};

#[derive(Debug, Default)]
pub struct SpaceFilter {
    pub space_ids: Option<Rc<Vec<String>>>,
    pub team_ids: Option<Rc<Vec<String>>>
}

impl From<&TeamFilter> for SpaceFilter {
    fn from(filter: &TeamFilter) -> Self {
        SpaceFilter {
            team_ids: filter.team_ids.clone(),
            ..Default::default()
        }
    }
}

impl From<TeamFilter> for SpaceFilter {
    fn from(filter: TeamFilter) -> Self {
        SpaceFilter {
            team_ids: filter.team_ids.clone(),
            ..Default::default()
        }
    }
}

impl From<FolderFilter> for SpaceFilter {
    fn from(filter: FolderFilter) -> Self {
        SpaceFilter {
            space_ids: filter.space_ids.clone(),
            ..Default::default()
        }
    }
}

impl From<&FolderFilter> for SpaceFilter {
    fn from(filter: &FolderFilter) -> Self {
        SpaceFilter {
            space_ids: filter.space_ids.clone(),
            ..Default::default()
        }
    }
}