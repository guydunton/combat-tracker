use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
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

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_initiative(&self) -> i32 {
        self.initiative
    }

    pub fn display_hp(&self) -> String {
        match (self.max_hp, self.current_hp) {
            (Some(max_hp), Some(current_hp)) => format!("{}/{}", current_hp, max_hp),
            _ => String::new(),
        }
    }

    pub fn reduce_health(&mut self, damage: i32) {
        self.current_hp = self.current_hp.map(|hp| (hp - damage).max(0))
    }

    pub fn is_dead(&self) -> bool {
        if let Some(hp) = self.current_hp {
            return hp <= 0;
        }
        false
    }
}
