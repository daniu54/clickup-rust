use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
pub struct TeamsResponse {
    pub teams: Vec<Team>,
}

#[derive(Deserialize, Debug, Default)]
pub struct Team {
    pub id: String,
    pub name: String,
    pub color: String,
    pub avatar: Option<String>,
    pub members: Option<Vec<UserResponse>>,
}

#[derive(Deserialize, Debug, Default)]
pub struct SpacesResponse {
    pub spaces: Vec<Space>,
}

#[derive(Deserialize, Debug, Default)]
pub struct Space {
    pub id: String,
    pub name: String,
    pub private: bool,
    pub color: Option<String>,
    pub avatar: Option<String>,
    pub admin_can_manage: Option<bool>,
    pub archived: Option<bool>,
    pub members: Option<Vec<UserResponse>>, // TODO rename user response?
    pub statuses: Vec<Status>,
    pub multiple_assignees: bool, // `features` omitted, see https://clickup.com/api/clickupreference/operation/GetSpaces/#!c=200&path=spaces/features&t=response
}

#[derive(Deserialize, Debug, Default)]
pub struct SpaceReference {
    pub id: String,
    pub name: Option<String>,
    pub access: Option<bool>,
}

#[derive(Deserialize, Debug, Default)]
pub struct FoldersResponse {
    pub folders: Vec<Folder>,
}

#[derive(Deserialize, Debug, Default)]
pub struct Folder {
    pub id: String,
    pub name: String,
    pub orderindex: i32,
    pub override_statuses: bool,
    pub hidden: bool,
    pub space: SpaceReference,
    pub task_count: String,
    pub lists: Vec<List>,
}

#[derive(Deserialize, Debug, Default)]
pub struct FolderReference {
    pub id: String,
    pub name: String,
    pub hidden: Option<bool>,
    pub access: Option<bool>,
}

#[derive(Deserialize, Debug, Default)]
pub struct ListsResponse {
    pub lists: Vec<List>,
}

#[derive(Deserialize, Debug, Default)]
pub struct List {
    pub id: String,
    pub name: String,
    pub orderindex: i32,
    pub status: Option<ListStatus>,
    pub priority: Option<ListPriority>,
    pub assignee: Option<String>,
    pub task_count: i32,
    pub due_date: Option<String>,
    pub start_date: Option<String>,
    // folder: FolderReference,
    pub space: SpaceReference,
    pub archived: Option<bool>,
    pub override_statuses: Option<bool>,
    pub statuses: Option<Vec<Status>>,
    pub permission_level: String,
}

#[derive(Deserialize, Debug, Default)]
pub struct ListStatus {
    pub status: Option<String>,
    pub color: Option<String>,
    pub hide_label: Option<bool>,
}

#[derive(Deserialize, Debug, Default)]
pub struct ListPriority {
    pub priority: Option<String>,
    pub color: Option<String>,
}

#[derive(Deserialize, Debug, Default)]
pub struct Status {
    pub status: String,
    pub color: String,
    pub orderindex: i32,
    #[serde(rename = "type")]
    pub status_type: String,
}

#[derive(Deserialize, Debug, Default)]
pub struct UserResponse {
    pub user: User,
    pub invited_by: Option<User>,
}

#[derive(Deserialize, Debug, Default)]
pub struct UserReference {
    pub id: u32,
    pub username: String,
    pub color: String,
    #[serde(rename = "profilePicture")]
    pub profile_picture: Option<String>,
}

#[derive(Deserialize, Debug, Default)]
pub struct User {
    pub id: u32,
    pub username: Option<String>,
    pub email: Option<String>,
    pub color: Option<String>,
    pub profile_picture: Option<String>,
}

#[derive(Deserialize, Debug, Default)]
pub struct PaginatedTasksResponse {
    pub tasks: Vec<Task>,
    pub last_page: bool,
}

#[derive(Deserialize, Debug, Default)]
pub struct Task {
    pub id: String,
    pub custom_item_id: Option<u32>,
    pub name: String,
    pub status: Status,
    #[serde(rename = "orderindex")]
    pub order_index: String,
    pub date_created: String,
    pub date_updated: String,
    pub date_closed: Option<String>,
    pub date_done: Option<String>,
    pub creator: Option<UserReference>,
    pub assignees: Vec<User>,
    pub checklists: Vec<CheckList>,
    pub tags: Vec<Tag>,
    pub parent: Option<String>,
    pub priority: Option<Priority>,
    pub due_date: Option<String>,
    pub start_date: Option<String>,
    pub time_estimate: Option<i32>,
    pub time_spent: Option<i32>,
    pub list: ListReference,
    pub project: ProjectReference,
    pub folder: FolderReference,
    pub space: SpaceReference,
    pub url: String,
}

#[derive(Deserialize, Debug, Default)]
pub struct Priority {
    pub color: String,
    pub id: String,
    #[serde(rename = "orderindex")]
    pub order_index: String,
    pub priority: String,
}

#[derive(Deserialize, Debug, Default)]
pub struct Tag {
    pub name: String,
    pub tag_fg: String,
    pub tag_bg: String,
    pub creator: u32,
}

#[derive(Deserialize, Debug, Default)]
pub struct CheckList {
    pub id: String,
    pub task_id: String,
    pub name: String,
    pub date_created: String,
    #[serde(rename = "orderindex")]
    pub order_index: f32,
    pub creator: i32,
    pub resolved: u32,
    pub unresolved: u32,
    pub items: Vec<CheckListItem>,
}

#[derive(Deserialize, Debug, Default)]
pub struct CheckListItem {
    pub id: String,
    pub name: String,
    #[serde(rename = "orderindex")]
    pub order_index: f32,
    pub creator: Option<i32>,
    pub assignee: Option<User>,
    pub group_assignee: Option<User>,
    pub resolved: bool,
    pub parent: Option<String>,
    pub date_created: String,
    pub children: Vec<CheckListItem>,
}

#[derive(Deserialize, Debug, Default)]
pub struct ListReference {
    pub id: String,
    pub name: String,
    pub access: bool,
}

#[derive(Deserialize, Debug, Default)]
pub struct ProjectReference {
    pub id: String,
    pub name: String,
    pub access: bool,
    pub hidden: bool,
}