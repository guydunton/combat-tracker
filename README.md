# Combat tracker

This app makes it simpler to manage combat as a TTRPG GM.

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

# Undo the last action
combat-tracker undo
```

## How it works

The program will create a `.combat-tracker` directory in `$HOME` to save combat data in between invocations.
