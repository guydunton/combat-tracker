use serde::{Deserialize, Serialize};

use crate::entity::Entity;

#[derive(Serialize, Deserialize, Default)]
pub struct State {
    turn: usize,
    entities: Vec<Entity>,
}

impl State {
    /// Print the combat encounter to screen
    pub fn show(&self) {
        println!("NAME INITIATIVE HP");
        for (index, entity) in self.entities.iter().enumerate() {
            println!(
                "{} {} {} {}",
                if index == self.turn { "*" } else { " " },
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

    pub fn start(&mut self) -> Option<&Entity> {
        self.turn = 0;
        self.entities.get(self.turn)
    }

    pub fn next_turn(&mut self) -> Option<&Entity> {
        // Find the next index which doesn't have a dead character
        let mut index = self.turn;
        loop {
            // Increment the index
            index = if index + 1 >= self.entities.len() {
                0
            } else {
                index + 1
            };

            // if the character isn't dead break
            if !self.entities[index].is_dead() {
                self.turn = index;
                return self.entities.get(index);
            }

            // Prevent an infinite loop
            if index == self.turn {
                self.turn = index;
                return None;
            }
        }
    }
}
