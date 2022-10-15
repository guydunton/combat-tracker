use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum Action {
    Damage {
        name: String,
        hp: i32,
    },
    AddEntity {
        name: String,
        initiative: i32,
        hp: Option<i32>,
    },
    Heal {
        name: String,
        hp: i32,
    },
    NudgeEntity {
        name: String,
    },
    ChangeTurn,
    StartEncounter,
}
