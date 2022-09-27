# Combat tracker

This app makes it simpler to manage combat as a TTRPG GM.

## How to install

- Check out the code
- Install the code:

```bash
cargo install --path .
```

## Commands

```bash
# Reset any previously combat
combat-tracker reset

# Add player Dain with initiative 12
combat-tracker add player Dain 12

# Add a beholder with initiative 8 & max HP 120
combat-tracker add monster beholder 8 120

# start the combat
combat-tracker start

# List all the combatants & show initiative
combat-tracker show

# Reduce the health of the beholder by 14
combat-tracker damage beholder 14

# Heal the beholder by 10
combat-tracker heal beholder 10

# Undo the last action
combat-tracker undo
```

## How it works

The program will create a `CombatTracker` directory one of the following directories to save combat data in between invocations:

| Platform | Value                                | Example                                  |
| -------- | ------------------------------------ | ---------------------------------------- |
| Linux    | $XDG_DATA_HOME or $HOME/.local/share | /home/alice/.local/share                 |
| macOS    | $HOME/Library/Application Support    | /Users/Alice/Library/Application Support |
| Windows  | {FOLDERID_LocalAppData}              | C:\Users\Alice\AppData\Local             |

## TODOS

- [ ] Undo - Use `combat-tracker undo` to undo the last action
- [ ] History - Be able to view the history of commands which have modified state
- [ ] Query history - Be able to query history e.g. _how much damage was done to monster X in the last turn_
