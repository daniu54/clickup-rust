use std::rc::Rc;

use super::SpaceFilter;

#[derive(Debug, Default)]
pub struct TeamFilter {
    pub team_ids: Option<Rc<Vec<String>>>
}

impl From<&SpaceFilter> for TeamFilter {
    fn from(filter: &SpaceFilter) -> Self {
        TeamFilter {
            team_ids: filter.team_ids.clone(),
            ..Default::default()
        }
    }
}

impl From<SpaceFilter> for TeamFilter {
    fn from(filter: SpaceFilter) -> Self {
        TeamFilter {
            team_ids: filter.team_ids.clone(),
            ..Default::default()
        }
    }
}