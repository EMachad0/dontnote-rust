use entity::workspace;
use uuid::Uuid;

#[derive(Debug, SimpleObject)]
pub struct WorkspaceType {
    #[graphql(skip)]
    pub id: i32,
    #[graphql(name = "id")]
    pub uuid: Uuid,
    pub title: String,
}

impl From<workspace::Model> for WorkspaceType {
    fn from(value: workspace::Model) -> Self {
        Self {
            id: value.id,
            uuid: Uuid::parse_str(&value.uuid).expect("invalid model uuid"),
            title: value.title,
        }
    }
}

pub struct WorkspaceList(Vec<WorkspaceType>);

impl From<Vec<workspace::Model>> for WorkspaceList {
    fn from(value: Vec<workspace::Model>) -> Self {
        let list = value.into_iter().map(WorkspaceType::from).collect();
        Self(list)
    }
}

impl From<WorkspaceList> for Vec<WorkspaceType> {
    fn from(value: WorkspaceList) -> Self {
        value.0
    }
}
