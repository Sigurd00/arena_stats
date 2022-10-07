use crate::class::Class;

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub realm: Option<Realm>,
    pub class: Class,
    pub spec: String, //Todo, change this
}

//TODO: Obviously get a list of realms
#[derive(Debug)]
pub enum Realm {
    Draenor,
}
