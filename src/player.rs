use crate::class::Class;

pub struct Player {
    name: String,
    class: Class,
}

impl Player {
    pub fn new(player_string: &str) -> Self {
        let mut split =player_string.split(',');
        let split_vec:Vec<&str> = split.collect();
        Player {
            name: String::from("Skli"),
            class: Class::Priest,
        }
    }
}

