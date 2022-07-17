use std::fmt::{Display, Formatter};
use crate::location::Location;
use crate::player::{PlayerTurn, PlayerType};
use crate::ship::{Ship, ShipType};

const GRID_WIDTH: u32 = 10;
const GRID_HEIGHT: u32 = 10;

enum EnemyBoardCell {
    Empty,
    Hit,
    Miss,
}

enum OwnBoardCell {
    Empty,
    Ship,
    ShipHit,
}

#[derive(Eq,PartialEq,Clone)]
enum GameResult {
    InSetup,
    InProgress,
    Player1Win,
    Player2Win
}
impl GameResult {
    fn player_win(player: &PlayerType) -> Self {
        match player {
            PlayerType::Player1 => GameResult::Player1Win,
            PlayerType::Player2 => GameResult::Player2Win
        }
    }
}

#[derive(Copy,Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}
impl Display for Direction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::result::Result<(), ::std::fmt::Error> {
        match *self {
            Direction::Up => f.write_str("Up"),
            Direction::Down => f.write_str("Down"),
            Direction::Left => f.write_str("Left"),
            Direction::Right => f.write_str("Right"),
        }
    }
}

#[derive(Eq, PartialEq, Clone)]
pub struct Shot {
    pub location: Location,
    pub hit: bool
}

#[derive(Clone)]
pub struct GameState {
    p1_ships: Vec<Ship>,
    p1_shots: Vec<Shot>,
    p2_ships: Vec<Ship>,
    p2_shots: Vec<Shot>,
    result: GameResult,
    turn: PlayerTurn,
    messages: Vec<String>,
}

impl GameState {
    pub fn new() -> Self {
        return GameState {
            p1_ships: vec!(
                Ship::battleship(),
                Ship::carrier(),
                Ship::carrier(),
                Ship::submarine(),
                Ship::submarine(),
                Ship::destroyer(),
                Ship::destroyer()),
            p1_shots: Vec::new(),
            p2_ships: vec!(
                Ship::battleship(),
                Ship::carrier(),
                Ship::carrier(),
                Ship::submarine(),
                Ship::submarine(),
                Ship::destroyer(),
                Ship::destroyer()),
            p2_shots: Vec::new(),
            result: GameResult::InSetup,
            turn: PlayerTurn::Either,
            messages: vec![String::from("Players, please place your ships to begin the game!")],
        }
    }

    pub fn shots(&self, player: &PlayerType) -> &Vec<Shot> {
        match player {
            PlayerType::Player1 => &self.p1_shots,
            PlayerType::Player2 => &self.p2_shots
        }
    }
    pub fn shots_mut(&mut self, player: &PlayerType) -> &mut Vec<Shot> {
        match player {
            PlayerType::Player1 => &mut self.p1_shots,
            PlayerType::Player2 => &mut self.p2_shots
        }
    }
    pub fn ships(&self, player: &PlayerType) -> &Vec<Ship> {
        match player {
            PlayerType::Player1 => &self.p1_ships,
            PlayerType::Player2 => &self.p2_ships
        }
    }
    pub fn ships_mut(&mut self, player: &PlayerType) -> &mut Vec<Ship> {
        match player {
            PlayerType::Player1 => &mut self.p1_ships,
            PlayerType::Player2 => &mut self.p2_ships
        }
    }
    pub fn last_message(&self) -> Option<&String> {
        return self.messages.last();
    }
}

// check if players have placed all ships
fn ready(game: &GameState, player: PlayerType) -> bool {
    if game.result != GameResult::InSetup {
        return false
    }
    return match player {
        PlayerType::Player1 => {
            for ship in game.p1_ships.iter() {
                if ship.locations.len() == 0 {
                    return false;
                }
            }
            true
        }
        PlayerType::Player2 => {
            for ship in game.p2_ships.iter() {
                if ship.locations.len() == 0 {
                    return false;
                }
            }
            true
        }
    }
}

fn expand(location: Location, class: ShipType, direction: Direction) -> Result<Vec<Location>, String> {
    let size = class.size();
    match direction {
        Direction::Up => {
            if location.row - size > 0 {
                let mut v: Vec<Location> = Vec::new();
                for row in (location.row - size)..location.row {
                    v.push(Location { row, col: location.col });
                }
                return Ok(v);
            }
        }
        Direction::Down => {
            if location.row + size < GRID_HEIGHT {
                let mut v: Vec<Location> = Vec::new();
                for row in location.row..(location.row+size) {
                    v.push(Location { row, col: location.col });
                }
                return Ok(v);
            }
        }
        Direction::Left => {
            if location.col - size > 0 {
                let mut v: Vec<Location> = Vec::new();
                for col in (location.col - size)..location.col {
                    v.push(Location { row: location.row, col });
                }
                return Ok(v);
            }
        }
        Direction::Right => {
            if location.col + size < GRID_WIDTH {
                let mut v: Vec<Location> = Vec::new();
                for col in location.col..(location.col+size) {
                    v.push(Location { row: location.row, col });
                }
                return Ok(v);
            }
        }
    }
    Err(format!("Not enough room to place a {} at {} {}", class, location, direction))
}

fn ship_at(game: &GameState, location: &Location) -> bool {
    for ship in game.p1_ships.iter() {
        for ship_location in ship.locations.iter() {
            if ship_location.eq(&location) {
                return true;
            }
        }
    }
    for ship in game.p2_ships.iter() {
        for ship_location in ship.locations.iter() {
            if ship_location.eq(&location) {
                return true;
            }
        }
    }
    return false;
}

fn place(game_state: GameState, player: PlayerType, class: ShipType, location: Location, direction: Direction) -> Result<GameState, String> {
    if !game_state.result.eq(&GameResult::InSetup) {
        return Err(format!("Cannot place ships after the game has started."));
    }

    let mut game = game_state.clone();

    // validate placement
    let mut expanded_locations = expand(location, class, direction)?;
    for loc in expanded_locations.iter() {
        if ship_at(&game_state, &loc) {
            return Err(format!("Cannot place a ship at {} {}, as it would overlap another ship.", location, direction));
        }
    }


    // attempt to place
    match player {
        PlayerType::Player1 => {
            for mut ship in game.p1_ships.iter_mut() {
                if ship.class == class && ship.locations.len() == 0 {
                    ship.locations.append(&mut expanded_locations);
                    return Ok(game);
                }
            }
        }
        PlayerType::Player2 => {
            for mut ship in game.p2_ships.iter_mut() {
                if ship.class == class && ship.locations.len() == 0 {
                    if ship.class == class && ship.locations.len() == 0 {
                        ship.locations.append(&mut expanded_locations);
                        return Ok(game);
                    }
                }
            }
        }
    }
    Err(format!("There are no ships of class {} left to place", class))
}

fn remove(game_state: &GameState, player: PlayerType, class: ShipType, location: Location) -> Result<GameState, String> {
    let mut game = game_state.clone();
    match player {
        PlayerType::Player1 => {
            for ship in game.p1_ships.iter_mut() {
                if ship.class == class && ship.locations.contains( &location) {
                    ship.locations.clear();
                    return Ok(game);
                }
            }
        }
        PlayerType::Player2 => {
            for ship in game.p2_ships.iter_mut() {
                if ship.class == class && ship.locations.contains( &location) {
                    ship.locations.clear();
                    return Ok(game);
                }
            }
        }
    }
    return Err(format!("Could not find a {} at {}", class, location));
}

fn fire(game_state: GameState, player: PlayerType, location: &Location) -> Result<GameState, String> {
    if !game_state.result.eq(&GameResult::InProgress) {
        return Err(format!("Cannot fire when game is not in progress"));
    }
    if !game_state.turn.eq(&PlayerTurn::from(&player)) {
        return Err(format!("{} cannot fire, it is not their turn.", player))
    }
    // location fields use usize, so don't have to check for < 0
    if &location.row >= &GRID_HEIGHT || &location.col >= &GRID_WIDTH {
        return Err(format!("Invalid fire coordinates {}, must be between {} and {}.", &location, Location{row:0, col:0}, Location{row: GRID_HEIGHT-1, col: GRID_WIDTH-1}));
    }

    //for shots in game_state.p1_shots
    for shot in game_state.shots(&player) {
        if shot.location.eq(location) {
            return Err(format!("Cannot fire on {}, you have already fired there!", &location));
        }
    }
    let mut next_state = game_state.clone();
    let mut hit = false;
    let mut sunk = false;
    let mut class = ShipType::Battleship;

    'outer: for mut ship in next_state.ships_mut(&PlayerType::other(&player)) {
        for ship_location in &ship.locations {
            if ship_location.eq(location) {
                hit = true;
                ship.hits += 1;
                sunk = ship.sunk();
                class = ship.class;
                break 'outer;
            }
        }
    }

    let mut shots = next_state.shots_mut(&player);
    shots.push(Shot{location: location.clone(), hit });


    if hit & sunk {
        next_state.messages.push(format!("{} sunk {}'s {}!", player, PlayerType::other(&player), class));
    } else if hit {
        next_state.messages.push(format!("{} fires at {} and hits {}'s ship!", player, location, &PlayerType::other(&player)));
    } else {
        next_state.messages.push(format!("{} fires at {} and misses!", player, location));
    }

    // check for win condition
    let mut unsunk_ships_remain = false;
    for ship in next_state.ships(&PlayerType::other(&player)) {
        if ship.sunk() == false {
            unsunk_ships_remain = true;
            break;
        }
    }

    if unsunk_ships_remain == false {
        next_state.result = GameResult::player_win(&player);
        next_state.messages.push(format!("Game over. {} wins!", player));
        next_state.turn = PlayerTurn::Neither;
    } else {
        next_state.turn = PlayerTurn::from(&PlayerType::other(&player));
    }

    return Ok(next_state);
}


// fn asOwnBoard(player: PlayerType) -> [[OwnBoardCell; 10]; 10] {}
// fn asEnemyBoard(player: PlayerType) -> [[OwnBoardCell; 10]; 10] {}
#[cfg(test)]
mod tests {
    use crate::game::{Direction, fire, GameResult, GameState, GRID_HEIGHT, GRID_WIDTH, place};
    use crate::location::Location;
    use crate::player::{PlayerTurn, PlayerType};
    use crate::ship::{Ship, ShipType};

    #[test]
    fn can_place_ships() {
        let state = GameState::new();
        let player = PlayerType::Player1;
        let class = ShipType::Submarine;
        let state_2 = place(state, player, class, Location{row:0, col:0}, Direction::Down);
        assert!(state_2.is_ok());
        let state_3 = place(state_2.unwrap(), player, class, Location{row:0, col:1}, Direction::Down);
        assert!(state_3.is_ok());
    }

    #[test]
    fn cant_place_extra_ships() {
        let state = GameState::new();
        let player = PlayerType::Player1;
        let class = ShipType::Submarine;
        let state_2 = place(state, player, class, Location{row:0, col:0}, Direction::Down);
        assert!(state_2.is_ok());
        let state_3 = place(state_2.unwrap(), player, class, Location{row:0, col:1}, Direction::Down);
        assert!(state_3.is_ok());
        let state_4 = place(state_3.unwrap(), player, class, Location{row:0, col:2}, Direction::Down);
        assert!(state_4.is_err());
        assert!(state_4.err().unwrap().contains("There are no ships of class"));
    }

    #[test]
    fn cant_place_ships_on_top_of_each_other() {
        let state = GameState::new();
        let player = PlayerType::Player1;
        let class = ShipType::Submarine;
        let state_2 = place(state, player, class, Location{row:0, col:0}, Direction::Down);
        assert!(state_2.is_ok());
        let state_3 = place(state_2.unwrap(), player, class, Location{row:0, col:0}, Direction::Down);
        assert!(state_3.is_err());
        assert!(state_3.err().unwrap().contains("as it would overlap another ship"));
    }

    #[test]
    fn cant_place_ships_if_game_is_not_in_setup() {
        let mut state = GameState::new();
        state.result = GameResult::InProgress;
        let player = PlayerType::Player1;
        let class = ShipType::Submarine;

        let state_2 = place(state, player, class, Location{row:0, col:0}, Direction::Down);
        assert!(state_2.is_err());
        assert!(state_2.err().unwrap().contains("after the game has started"));
    }

    #[test]
    fn cant_fire_if_game_is_not_in_progress() {
        let state = GameState::new();
        let player = PlayerType::Player1;

        let state_2 = fire(state, player, &Location{row:0, col:0});
        assert!(state_2.is_err());
        assert!(state_2.err().unwrap().contains("Cannot fire when game is not in progress"));
    }

    #[test]
    fn cant_fire_if_not_players_turn() {
        let mut state = GameState::new();
        state.result = GameResult::InProgress;
        state.turn = PlayerTurn::Player1;
        let player = PlayerType::Player2;

        let state_2 = fire(state, player, &Location{row:0, col:0});
        assert!(state_2.is_err());
        assert!(state_2.err().unwrap().contains("cannot fire, it is not their turn."));
    }

    #[test]
    fn cant_fire_out_of_bounds() {
        let mut state = GameState::new();
        state.result = GameResult::InProgress;
        state.turn = PlayerTurn::Player1;
        let player = PlayerType::Player1;

        let state_2 = fire(state.clone(), player, &Location{row:GRID_HEIGHT, col:0});
        assert!(state_2.is_err());
        assert!(state_2.err().unwrap().contains("Invalid fire coordinates"));

        let state_3 = fire(state, player, &Location{row:0, col:GRID_WIDTH});
        assert!(state_3.is_err());
        assert!(state_3.err().unwrap().contains("Invalid fire coordinates"));
    }

    #[test]
    fn cant_fire_in_same_location() {
        let mut state = GameState::new();
        state.result = GameResult::InProgress;
        state.turn = PlayerTurn::Player1;
        let player = PlayerType::Player1;
        let mut state_2 = fire(state, player, &Location{row:0, col:0}).unwrap();
        // keep it same player's turn for test
        state_2.turn = PlayerTurn::Player1;
        let state_3 = fire(state_2, player, &Location{row:0, col:0});
        assert!(state_3.is_err());
        assert!(state_3.err().unwrap().contains("you have already fired there"));
    }

    #[test]
    fn fire_and_miss() {
        let state = GameState::new();
        let mut state_2 = place(state, PlayerType::Player2, ShipType::Submarine, Location{row:0, col:0}, Direction::Down).unwrap();
        state_2.result = GameResult::InProgress;
        state_2.turn = PlayerTurn::Player1;
        let player = PlayerType::Player1;
        let state_3 = fire(state_2, player, &Location{row:0, col:1}).unwrap();
        let message = state_3.last_message().unwrap();
        assert!(message.contains(&String::from("and misses")));
        assert!(state_3.turn.eq(&PlayerTurn::Player2));
    }

    #[test]
    fn fire_and_hit() {
        let state = GameState::new();
        let mut state_2 = place(state, PlayerType::Player2, ShipType::Submarine, Location{row:0, col:0}, Direction::Down).unwrap();
        state_2.result = GameResult::InProgress;
        state_2.turn = PlayerTurn::Player1;
        let player = PlayerType::Player1;
        let state_3 = fire(state_2, player, &Location{row:0, col:0}).unwrap();
        let message = state_3.last_message().unwrap();
        assert!(message.contains(&String::from("and hits")));
        assert!(state_3.turn.eq(&PlayerTurn::Player2));
    }

    #[test]
    fn fire_and_sink(){
        let state = GameState::new();
        let mut state_2 = place(state, PlayerType::Player2, ShipType::Submarine, Location{row:0, col:0}, Direction::Down).unwrap();
        state_2.result = GameResult::InProgress;
        state_2.turn = PlayerTurn::Player1;
        let player = PlayerType::Player1;
        let mut state_3 = fire(state_2, player, &Location{row:0, col:0}).unwrap();
        state_3.turn = PlayerTurn::Player1;
        let mut state_4 = fire(state_3, player, &Location{row:1, col:0}).unwrap();
        let message = state_4.last_message().unwrap();
        assert!(message.contains(&String::from("sunk")));
        assert!(state_4.turn.eq(&PlayerTurn::Player2));
    }

    #[test]
    fn fire_and_win() {
        let state = GameState {
            p1_ships: vec!(
                Ship{
                    class: ShipType::Destroyer,
                    locations: vec![Location{row:0, col:0}, Location{row:1,col:0}],
                    hits: 0
                }),
            p1_shots: Vec::new(),
            p2_ships: vec!(
                Ship{
                    class: ShipType::Destroyer,
                    locations: vec![Location{row:0, col:0}, Location{row:1,col:0}],
                    hits: 0
                }),
            p2_shots: Vec::new(),
            result: GameResult::InProgress,
            turn: PlayerTurn::Player1,
            messages: vec![String::from("It's Player 1's turn.")],
        };
        let player = PlayerType::Player1;
        let mut state_2 = fire(state, player, &Location{row:0,col:0}).unwrap();
        state_2.turn = PlayerTurn::Player1;
        let state_3 = fire(state_2, player, &Location{row:1,col:0}).unwrap();
        assert!(state_3.result.eq(&GameResult::Player1Win));
        assert!(state_3.turn.eq(&PlayerTurn::Neither));
        let message = state_3.last_message().unwrap();
        assert!(message.contains(&String::from("Game over. Player 1 wins!")));
    }

}