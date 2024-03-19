use std::rc::Rc;

use crate::{
    filters::{FolderFilter, ListFilter, SpaceFilter, TaskFilter, TeamFilter},
    response_objects::{
        Folder, FoldersResponse, List, ListsResponse, PaginatedTasksResponse, Space,
        SpacesResponse, Task, Team, TeamsResponse,
    },
};

use ureq::serde_json;

pub struct ClickupApi {
    api_token: String,
    api_url: String,
}

impl ClickupApi {
    pub fn new(api_token: &str, api_url: &str) -> Self {
        Self {
            api_token: api_token.to_string(),
            api_url: api_url.to_string(),
        }
    }

    pub fn get_teams(&self, filter: &TeamFilter) -> Result<Vec<Team>, ureq::Error> {
        let url = format!("{0}/team", self.api_url);

        let request = ureq::get(&url).set("Authorization", &self.api_token);

        let response: TeamsResponse = request.call()?.into_json()?;

        let mut teams = response.teams;

        if let Some(filter_team_ids) = &filter.team_ids {
            teams = teams
                .into_iter()
                .filter(|t| filter_team_ids.contains(&t.id))
                .collect::<Vec<_>>();
        }

        Ok(teams)
    }

    pub fn get_spaces(&self, filter: &SpaceFilter) -> Result<Vec<Space>, ureq::Error> {
        let team_ids: Rc<Vec<String>>;

        if let Some(filter_team_ids) = &filter.team_ids {
            team_ids = filter_team_ids.clone()
        } else {
            team_ids = Rc::new(
                self.get_teams(&filter.into())
                    .unwrap()
                    .into_iter()
                    .map(|t| t.id)
                    .collect(),
            );
        }

        let mut spaces: Vec<Space> = vec![];

        for team_id in team_ids.iter() {
            let url = format!("{0}/team/{team_id}/space", self.api_url);

            let request = ureq::get(&url).set("Authorization", &self.api_token);

            let response: SpacesResponse = request.call()?.into_json()?;

            let mut response_spaces: Vec<Space> = response.spaces;

            if let Some(filter_space_ids) = &filter.space_ids {
                response_spaces = response_spaces
                    .into_iter()
                    .filter(|s| filter_space_ids.contains(&s.id))
                    .collect::<Vec<_>>();
            }

            spaces.append(&mut response_spaces);
        }

        Ok(spaces)
    }

    pub fn get_folders(&self, filter: &FolderFilter) -> Result<Vec<Folder>, ureq::Error> {
        let space_ids: Rc<Vec<String>>;

        if let Some(filter_space_ids) = &filter.space_ids {
            space_ids = filter_space_ids.clone()
        } else {
            space_ids = Rc::new(
                self.get_spaces(&filter.into())
                    .unwrap()
                    .into_iter()
                    .map(|s| s.id)
                    .collect(),
            );
        }

        let mut folders: Vec<Folder> = vec![];

        for space_id in space_ids.iter() {
            let url = format!("{0}/space/{space_id}/folder", self.api_url);

            let response_string = ureq::get(&url)
                .set("Authorization", &self.api_token)
                .query("archived", &filter.archived.to_string())
                .call()?
                .into_string()?;

            let response: FoldersResponse = match serde_json::from_str(&response_string) {
                Ok(value) => value,
                Err(error) => {
                    let buffer = 30;

                    let start_index = error.column().checked_sub(buffer).or(Some(0)).unwrap();
                    let end_index = (error.column() + buffer).min(response_string.len());

                    let error_context: String = response_string[start_index..end_index].to_string();

                    println!("{}", response_string);
                    println!("{}", error_context);
                    println!("{:?}", &error);
                    FoldersResponse::default() // FIXME here we silently swallow deserialization error, should retrow
                }
            };

            let mut response_folders: Vec<Folder> = response.folders;

            if let Some(filter_folder_ids) = &filter.folder_ids {
                response_folders = response_folders
                    .into_iter()
                    .filter(|f| filter_folder_ids.contains(&f.id))
                    .collect::<Vec<_>>();
            }

            folders.append(&mut response_folders);
        }

        Ok(folders)
    }

    pub fn get_lists(&self, filter: &ListFilter) -> Result<Vec<List>, ureq::Error> {
        let folder_ids: Rc<Vec<String>>;

        if let Some(filter_folder_ids) = &filter.folder_ids {
            folder_ids = filter_folder_ids.clone()
        } else {
            folder_ids = Rc::new(
                self.get_folders(&filter.into())
                    .unwrap()
                    .into_iter()
                    .map(|f| f.id)
                    .collect(),
            );
        }

        let mut lists: Vec<List> = vec![];

        for folder_id in folder_ids.iter() {
            let url = format!("{0}/folder/{folder_id}/list", self.api_url);

            let response_string = ureq::get(&url)
                .set("Authorization", &self.api_token)
                .query("archived", &filter.archived.to_string())
                .call()?
                .into_string()?;

            let lists_response: ListsResponse = match serde_json::from_str(&response_string) {
                Ok(value) => value,
                Err(error) => {
                    let buffer = 30;

                    let start_index = error.column().checked_sub(buffer).or(Some(0)).unwrap();
                    let end_index = (error.column() + buffer).min(response_string.len());

                    let error_context: String = response_string[start_index..end_index].to_string();

                    println!("{}", response_string);
                    println!("{}", error_context);
                    println!("{:?}", &error);
                    ListsResponse::default() // FIXME here we silently swallow deserialization error, should retrow
                }
            };

            let mut response_lists: Vec<List> = lists_response.lists;

            if let Some(filter_list_ids) = &filter.list_ids {
                response_lists = response_lists
                    .into_iter()
                    .filter(|l| filter_list_ids.contains(&l.id))
                    .collect::<Vec<_>>();
            }

            lists.append(&mut response_lists);
        }

        Ok(lists)
    }

    pub fn get_tasks(&self, filter: &TaskFilter) -> Result<Vec<Task>, ureq::Error> {
        let list_ids: Rc<Vec<String>>;

        if let Some(filter_list_ids) = &filter.list_ids {
            list_ids = filter_list_ids.clone()
        } else {
            list_ids = Rc::new(
                self.get_lists(&filter.into())
                    .unwrap()
                    .into_iter()
                    .map(|l| l.id)
                    .collect(),
            );
        }

        let mut tasks: Vec<Task> = Vec::new();

        for list_id in list_ids.iter() {
            let mut page_number = 0;

            loop {
                let url = format!("{0}/list/{list_id}/task", self.api_url);

                let response_string = ureq::get(&url)
                    .set("Authorization", &self.api_token)
                    .query("page", &page_number.to_string())
                    .call()?
                    .into_string()?;

                let tasks_response: PaginatedTasksResponse =
                    match serde_json::from_str(&response_string) {
                        Ok(value) => value,
                        Err(error) => {
                            let buffer = 30;

                            let start_index =
                                error.column().checked_sub(buffer).or(Some(0)).unwrap();
                            let end_index = (error.column() + buffer).min(response_string.len());

                            let error_context: String =
                                response_string[start_index..end_index].to_string();

                            println!("{}", response_string);
                            println!("{}", error_context);
                            println!("{:?}", &error);
                            PaginatedTasksResponse::default()
                        }
                    };

                let mut response_tasks: Vec<Task> = tasks_response.tasks;

                if let Some(filter_task_ids) = &filter.task_ids {
                    response_tasks = response_tasks
                        .into_iter()
                        .filter(|t| filter_task_ids.contains(&t.id))
                        .collect::<Vec<_>>();
                }

                tasks.extend(response_tasks);

                if tasks_response.last_page {
                    break;
                }

                page_number += 1;
            }
        }

        Ok(tasks)
    }
}
