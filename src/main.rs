mod action;
mod entity;
mod state;
mod table;

use action::Action;
use clap::{Arg, Command};
use state::State;
use std::fs::{create_dir_all, read_to_string, File};
use std::io::Write;
use std::path::{Path, PathBuf};

fn get_app_file(app_dir: PathBuf) -> PathBuf {
    let config_file_name = "config.json";
    app_dir.join(config_file_name)
}

fn get_app_dir() -> Option<PathBuf> {
    if cfg!(debug_assertions) {
        return Some(Path::new(".").to_path_buf());
    } else {
        return dirs::data_local_dir().map(|local_dir| local_dir.join(Path::new("CombatTracker")));
    }
}

fn read_config_state(config_path: &PathBuf) -> State {
    read_to_string(config_path)
        .ok()
        .and_then(|data| serde_json::from_str(&data).ok())
        .unwrap_or_default()
}

fn print_action(state: &State, action: &Action) {
    match action {
        Action::Damage(name, _) => {
            if let Some(entity) = state.get_entity(&name) {
                if entity.is_dead() {
                    println!("{} Killed", entity.get_name());
                } else {
                    println!("{}: {}", entity.get_name(), entity.display_hp());
                }
            }
        }
        Action::Heal(name, _) => {
            if let Some(entity) = state.get_entity(&name) {
                println!("{} {}", entity.get_name(), entity.display_hp());
            }
        }
        Action::StartEncounter => {
            println!("{}'s turn", state.current_turn_entity().get_name());
        }
        Action::ChangeTurn => {
            println!("{}'s turn", state.current_turn_entity().get_name());
        }
        _ => {}
    }
}

fn main() {
    let config_dir = get_app_dir().expect("To find a path to local data dir");

    // If the config directory doesn't exist then create it
    if !config_dir.exists() {
        create_dir_all(&config_dir).expect("To be able to create a config directory");
    }

    let config_path = get_app_file(config_dir);

    let mut state: State = read_config_state(&config_path);

    // Create the CLI
    let matches = Command::new("Combat Tracker")
        .version("0.1")
        .about("Keep track of combat in D&D")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("add")
                .about("Add a player or monster to the encounter")
                .visible_alias("a")
                .arg(
                    Arg::new("NAME")
                        .index(1)
                        .required(true)
                        .help("The name of the player or monster"),
                )
                .arg(
                    Arg::new("INITIATIVE")
                        .index(2)
                        .required(true)
                        .value_parser(clap::value_parser!(i32).range(0..100))
                        .help("The initiative value"),
                )
                .arg(
                    Arg::new("HP")
                        .index(3)
                        .required(false)
                        .value_parser(clap::value_parser!(i32).range(0..1000))
                        .help("The HP to use for the player or monster"),
                ),
        )
        .subcommand(
            Command::new("reset")
                .arg(
                    Arg::new("confirm")
                        .long("confirm")
                        .help("Confirm that you really want to reset the encounter")
                        .required(true),
                )
                .about("Reset the current encounter and remove all monsters and players")
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("show")
                .visible_alias("ls")
                .about("Show the encounter"),
        )
        .subcommand(
            Command::new("damage")
                .about("Remove health from a monster")
                .visible_alias("dg")
                .arg(
                    Arg::new("NAME")
                        .index(1)
                        .help("The name of the monster")
                        .required(true),
                )                .arg(
                    Arg::new("HP")
                        .index(2)
                        .help("The amount of damage")
                        .required(true)
                        .value_parser(clap::value_parser!(i32).range(0..1000)),
                ),
        )
        .subcommand(
            Command::new("nudge")
                .about("Move a character up in the initiative. For cases when 2 characters have the same initiative")
                .arg(
                    Arg::new("NAME")
                        .help("The name of the character")
                        .index(1)
                        .required(true)
                )
            )
        .subcommand(
            Command::new("heal")
                .about("Heal a monster")
                .arg(
                    Arg::new("NAME")
                        .help("The name of the creature")
                        .index(1)
                        .required(true)
                )
                .arg(
                    Arg::new("HP")
                        .help("The amount of health to heal by")
                        .index(2)
                        .required(true)
                        .value_parser(clap::value_parser!(i32).range(0..1000)),
                )
        )
        .subcommand(
            Command::new("start")
                .about("Start the combat")
        )
        .subcommand(
            Command::new("next")
                .about("Move combat onto the next monster/character")
        )
        .subcommand(
            Command::new("undo")
                .about("Undo the last action")
        )
        .get_matches();

    // Modify the app data
    match matches.subcommand() {
        Some(("add", add_subcommand)) => {
            let name = add_subcommand
                .get_one::<String>("NAME")
                .expect("To have a NAME parameter for add");

            let initiative = *add_subcommand
                .get_one::<i32>("INITIATIVE")
                .expect("To have an INITIATIVE parameter for add");

            let max_hp = add_subcommand.get_one::<i32>("HP").map(|v| *v);

            let action = Action::AddEntity(name.clone(), initiative, max_hp);
            state.process_action(&action)
        }
        Some(("damage", damage_subcommand)) => {
            let name = damage_subcommand
                .get_one::<String>("NAME")
                .expect("To have a NAME parameter for add");

            let damage = *damage_subcommand
                .get_one::<i32>("HP")
                .expect("To have an HP parameter for add");

            let action = Action::Damage(name.clone(), damage);

            state.process_action(&action);
            print_action(&state, &action);
        }
        Some(("heal", heal_subcommand)) => {
            let name = heal_subcommand
                .get_one::<String>("NAME")
                .expect("To have a NAME parameter for add");

            let hp = *heal_subcommand
                .get_one::<i32>("HP")
                .expect("To have an HP parameter for add");

            let action = Action::Heal(name.clone(), hp);
            state.process_action(&action);
            print_action(&state, &action);
        }
        Some(("nudge", nudge_subcommand)) => {
            let name = nudge_subcommand
                .get_one::<String>("NAME")
                .expect("To have a NAME parameter for add");

            let action = Action::NudgeEntity(name.clone());
            state.process_action(&action);
        }
        Some(("start", _)) => {
            let action = Action::StartEncounter;
            state.process_action(&action);
            print_action(&state, &action);
        }
        Some(("next", _)) => {
            let action = Action::ChangeTurn;
            state.process_action(&action);
            print_action(&state, &action);
        }
        Some(("reset", _)) => {
            state = State::default();
        }
        Some(("undo", _)) => {
            state.undo();
        }
        Some(("show", _)) => state.show(),
        _ => {
            println!("Unknown command. Please refer to combat-tracker --help");
        }
    }

    // Re-save the app data
    let state_json = serde_json::to_string_pretty(&state).unwrap();
    File::create(config_path)
        .unwrap()
        .write_all(state_json.as_bytes())
        .unwrap();
}
