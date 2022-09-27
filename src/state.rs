use serde::{Deserialize, Serialize};

use crate::entity::Entity;

#[derive(Serialize, Deserialize, Default)]
pub struct State {
    entities: Vec<Entity>,
}

impl State {
    /// Print the combat encounter to screen
    pub fn show(&self) {
        println!("NAME INITIATIVE HP");
        for entity in self.entities.iter() {
            println!(
                "{} {} {}",
                entity.get_name(),
                entity.get_initiative(),
                entity.display_hp()
            );
        }
    }

    pub fn add_entity(&mut self, entity: Entity) {
        let index = self
            .entities
            .iter()
            .enumerate()
            .find(|(_, e)| e.get_initiative() < entity.get_initiative())
            .map(|(index, _)| index)
            .unwrap_or(self.entities.len());
        self.entities.insert(index, entity);
    }

    pub fn damage_entity(&mut self, name: &str, damage: i32) -> Option<&Entity> {
        if let Some(entity) = self.entities.iter_mut().find(|e| e.get_name() == name) {
            entity.reduce_health(damage);
            return Some(entity);
        }
        None
    }

    pub fn nudge(&mut self, name: &str) {
        for (index, window) in self.entities.windows(2).enumerate() {
            if window[1].get_name() == name
                && window[0].get_initiative() == window[1].get_initiative()
            {
                // Swap them
                self.entities.swap(index, index + 1);
                return;
            }
        }
    }
}
