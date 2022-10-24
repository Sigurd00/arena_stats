# Todo List
- [ ] Make lists of wins and lists of losses for optimization **<-- Doing this later**
- [x] Create a GameBucket struct
- [ ] Put every game into its gamebucket effeciently. Maybe when the games are being parsed?
- Things to analyze:
    - [x] Calculate every comp played against
    - [x] Rating difference
    - [x] Winrate
    - [ ] Best winrate vs comp
    - [ ] Worst winrate vs comp
    - [ ] Most often played against
    - [ ] Chart rating diff? requires plotting *sigh*
    - [ ] Maps with best winrate?
    - [ ] Game frequency?
    - [ ] How much rating lost / gained vs comps
    - [x] Average game time
    - [x] Game time vs different comps
    - [ ] Highest rating / mmr
- [ ] User experience
    - [ ] Show list of commands on startup?
    - [ ] How should the commands look like?
    - [ ] Handle user input
    - [ ] Make file paramater optional
    - [ ] Let user paste csv into cli
    - [ ] Let user import csv from file while program running
- [ ] Tests and benches

# How to run
- Export data from your REflex Addon and save the file as `<filename>.csv` in the project.
- `cargo run <filename>.csv`
