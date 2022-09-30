use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum Action {
    Damage(String, i32),
    AddEntity(String, i32, Option<i32>),
    Heal(String, i32),
    NudgeEntity(String),
    ChangeTurn,
    StartEncounter,
}
