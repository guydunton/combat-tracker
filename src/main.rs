mod entity;
mod state;

use clap::{Arg, Command};
use entity::Entity;
use state::State;
use std::fs::{read_to_string, File};
use std::io::Write;
use std::path::{Path, PathBuf};

fn get_app_dir() -> Option<PathBuf> {
    let config_file_name = "config.json";
    if cfg!(debug_assertions) {
        return Some(Path::new(".").join(config_file_name).to_path_buf());
    } else {
        return dirs::data_local_dir().map(|local_dir| {
            local_dir
                .join(Path::new("CombatTracker"))
                .join(config_file_name)
        });
    }
}

fn read_config_state(config_path: &PathBuf) -> State {
    read_to_string(config_path)
        .ok()
        .and_then(|data| serde_json::from_str(&data).ok())
        .unwrap_or_default()
}

fn main() {
    let config_path = get_app_dir();

    let config_path = config_path.expect("To find a path to local data dir");

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
                .alias("a")
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
                        .short('c')
                        .help("Confirm that you really want to reset the encounter")
                        .required(true),
                )
                .about("Reset the current encounter and remove all monsters and players")
                .arg_required_else_help(true),
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

            match max_hp {
                Some(hp) => {
                    state.entities.push(Entity::monster(name, initiative, hp));
                }
                None => state.entities.push(Entity::player(name, initiative)),
            }
        }
        Some(("reset", _)) => {
            state = State::default();
        }
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