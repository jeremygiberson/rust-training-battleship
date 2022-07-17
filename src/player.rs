use std::fmt::{Display, Formatter};

#[derive(Clone, Copy)]
pub enum PlayerType {
    Player1,
    Player2
}

impl PlayerType {
    pub fn other(player: &PlayerType) -> Self {
        match player {
            PlayerType::Player1 => PlayerType::Player2,
            PlayerType::Player2 => PlayerType::Player1
        }
    }
}

impl Display for PlayerType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            PlayerType::Player1 => f.write_str("Player 1"),
            PlayerType::Player2 => f.write_str("Player 2"),
        }
    }
}

#[derive(Clone,Eq,PartialEq)]
pub enum PlayerTurn {
    Player1,
    Player2,
    Either,
    Neither,
}

impl PlayerTurn {
    pub fn from(player: &PlayerType) -> Self {
        match player {
            PlayerType::Player1 => PlayerTurn::Player1,
            PlayerType::Player2 => PlayerTurn::Player2,
        }
    }
}

impl Display for PlayerTurn {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> std::result::Result<(), ::std::fmt::Error> {
        match *self {
            PlayerTurn::Player1 => f.write_str("Player 1"),
            PlayerTurn::Player2 => f.write_str("Player 2"),
            PlayerTurn::Either => f.write_str("Either Player"),
            PlayerTurn::Neither => f.write_str("Neither Player")
        }
    }
}
