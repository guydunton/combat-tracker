use serde::{Deserialize, Serialize};

use crate::{action::Action, entity::Entity, table::Table};

#[derive(Serialize, Deserialize)]
pub struct Event {
    round: i32,
    action: Action,
}

#[derive(Serialize, Deserialize, Default)]
pub struct State {
    round: i32,
    turn: usize,
    entities: Vec<Entity>,
    history: Vec<Event>,
}

impl State {
    /// Print the combat encounter to screen
    pub fn show(&self) {
        let mut table = Table::new(vec![
            "".to_owned(),
            "NAME".to_owned(),
            "INITIATIVE".to_owned(),
            "HP".to_owned(),
        ]);

        for (index, entity) in self.entities.iter().enumerate() {
            table.add_row(vec![
                if index == self.turn {
                    "*".to_owned()
                } else {
                    "".to_owned()
                },
                entity.get_name().to_owned(),
                format!("{}", entity.get_initiative()),
                entity.display_hp(),
            ]);
        }

        table.print();
    }

    fn add_entity(&mut self, entity: Entity) {
        let index = self
            .entities
            .iter()
            .enumerate()
            .find(|(_, e)| e.get_initiative() < entity.get_initiative())
            .map(|(index, _)| index)
            .unwrap_or(self.entities.len());
        self.entities.insert(index, entity);
    }

    fn damage_entity(&mut self, name: &str, damage: i32) -> Option<&Entity> {
        if let Some(entity) = self.entities.iter_mut().find(|e| e.get_name() == name) {
            entity.reduce_health(damage);
            return Some(entity);
        }
        None
    }

    fn nudge(&mut self, name: &str) {
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

    fn start(&mut self) -> Option<&Entity> {
        self.turn = 0;
        self.round = 1;
        self.entities.get(self.turn)
    }

    fn next_turn(&mut self) -> Option<&Entity> {
        // Find the next index which doesn't have a dead character
        let mut index = self.turn;
        loop {
            // Increment the index
            if index + 1 >= self.entities.len() {
                self.round += 1;
                index = 0;
            } else {
                index += 1;
            }

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

    fn heal_entity(&mut self, name: &str, hp: i32) -> Option<&Entity> {
        if let Some(entity) = self.entities.iter_mut().find(|e| e.get_name() == name) {
            entity.increase_health(hp);
            return Some(entity);
        }
        None
    }

    pub fn current_turn_entity(&self) -> &Entity {
        &self.entities[self.turn]
    }

    pub fn get_entity(&self, name: &str) -> Option<&Entity> {
        self.entities.iter().find(|e| e.get_name() == name)
    }

    fn get_entity_mut(&mut self, name: &str) -> Option<&mut Entity> {
        self.entities.iter_mut().find(|e| e.get_name() == name)
    }

    pub fn process_action(&mut self, action: &Action) {
        // Store action
        self.history.push(Event {
            action: action.clone(),
            round: self.round,
        });

        match action.clone() {
            Action::Damage(name, hp) => {
                self.damage_entity(&name, hp);
            }
            Action::AddEntity(name, initiative, hp) => match hp {
                Some(hp) => {
                    self.add_entity(Entity::monster(&name, initiative, hp));
                }
                None => {
                    self.add_entity(Entity::player(&name, initiative));
                }
            },
            Action::Heal(name, hp) => {
                self.heal_entity(&name, hp);
            }
            Action::NudgeEntity(name) => {
                self.nudge(&name);
            }
            Action::ChangeTurn => {
                self.next_turn();
            }
            Action::StartEncounter => {
                self.start();
            }
        }
    }

    pub fn undo(&mut self) {
        // Pull the last action from the queue
        if let Some(action) = self.history.pop() {
            match action.action {
                Action::Damage(name, hp) => {
                    if let Some(entity) = self.get_entity_mut(&name) {
                        entity.increase_health(hp);
                    }
                }
                Action::Heal(name, hp) => {
                    if let Some(entity) = self.get_entity_mut(&name) {
                        entity.reduce_health(hp);
                    }
                }
                Action::AddEntity(name, _, _) => {
                    if let Some(index) = self
                        .entities
                        .iter()
                        .enumerate()
                        .find(|(_, e)| e.get_name() == name)
                        .map(|(index, _)| index)
                    {
                        self.entities.remove(index);
                    }
                }
                Action::ChangeTurn => {
                    let (subbed_value, underflow) = self.turn.overflowing_sub(1);
                    if underflow {
                        self.turn = self.entities.len() - 1;
                        self.round -= 1;
                    } else {
                        self.turn = subbed_value;
                    }
                }
                _ => {
                    println!("Action not supported by undo. Cannot undo");
                    self.history.push(action);
                }
            }
        }
    }

    pub fn history(&self) -> &Vec<Event> {
        &self.history
    }
}
