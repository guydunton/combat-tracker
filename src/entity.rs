use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Entity {
    name: String,
    initiative: i32,
    max_hp: Option<i32>,
    current_hp: Option<i32>,
}

impl Entity {
    pub fn player(name: &str, initiative: i32) -> Self {
        Self {
            name: name.to_owned(),
            initiative,
            max_hp: None,
            current_hp: None,
        }
    }

    pub fn monster(name: &str, initiative: i32, max_hp: i32) -> Self {
        Self {
            name: name.to_owned(),
            initiative,
            max_hp: Some(max_hp),
            current_hp: Some(max_hp),
        }
    }
}
