use crate::class::Class;

#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub realm: Option<Realm>,
    pub class: Class,
    pub spec: String, //Todo, change this
}

//TODO: Obviously get a list of realms
#[derive(Debug, Clone)]
pub enum Realm {
    Draenor,
}
