use entity::note;
use uuid::Uuid;

#[derive(Debug, SimpleObject)]
pub struct NoteType {
    #[graphql(skip)]
    pub id: i32,
    #[graphql(name = "id")]
    pub uuid: Uuid,
    pub title: String,
    pub color: String,
    pub content: String,
}

impl From<note::Model> for NoteType {
    fn from(value: note::Model) -> Self {
        Self {
            id: value.id,
            uuid: Uuid::parse_str(&value.uuid).expect("invalid model uuid"),
            title: value.title,
            color: value.color,
            content: value.content,
        }
    }
}

pub struct NoteList(Vec<NoteType>);

impl From<Vec<note::Model>> for NoteList {
    fn from(value: Vec<note::Model>) -> Self {
        let list = value.into_iter().map(NoteType::from).collect();
        Self(list)
    }
}

impl From<NoteList> for Vec<NoteType> {
    fn from(value: NoteList) -> Self {
        value.0
    }
}
