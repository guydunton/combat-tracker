use serde::{Deserialize, Serialize};

use crate::entity::Entity;

#[derive(Serialize, Deserialize, Default)]
pub struct State {
    entities: Vec<Entity>,
}

impl State {
    /// Print the combat encounter to screen
    pub fn show(&self) {
        let mut entities = self.entities.clone();
        entities.sort_by(|a, b| b.get_rank().partial_cmp(&a.get_rank()).unwrap());
        println!("NAME INITIATIVE HP");
        for entity in entities.iter() {
            println!(
                "{} {} {}",
                entity.get_name(),
                entity.get_initiative(),
                entity.display_hp()
            );
        }
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
    }

    pub fn damage_entity(&mut self, name: &str, damage: i32) -> Option<&Entity> {
        if let Some(entity) = self.entities.iter_mut().find(|e| e.get_name() == name) {
            entity.reduce_health(damage);
            return Some(entity);
        }
        None
    }

    pub fn nudge(&mut self, name: &str) {
        if let Some(entity) = self.entities.iter_mut().find(|e| e.get_name() == name) {
            entity.bump_rank();
        }
    }
}
