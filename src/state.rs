use serde::{Deserialize, Serialize};

use crate::entity::Entity;

#[derive(Serialize, Deserialize, Default)]
pub struct State {
    pub entities: Vec<Entity>,
}
