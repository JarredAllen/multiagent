use enum_iterator::IntoEnumIterator;
use multiagent::GameState;

use std::fmt::{self, Display};

#[derive(Copy, Clone, PartialEq, Eq, Debug, IntoEnumIterator)]
pub enum Player {
    X,
    O,
}
impl Player {
    fn other(self) -> Self {
        use Player::*;
        match self {
            X => O,
            O => X,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, IntoEnumIterator)]
pub enum Position {
    TopLeft,
    TopCenter,
    TopRight,
    CenterLeft,
    Center,
    CenterRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}
impl From<Position> for (u8, u8) {
    fn from(pos: Position) -> (u8, u8) {
        use Position::*;
        match pos {
            TopLeft => (0, 0),
            TopCenter => (0, 1),
            TopRight => (0, 2),
            CenterLeft => (1, 0),
            Center => (1, 1),
            CenterRight => (1, 2),
            BottomLeft => (2, 0),
            BottomCenter => (2, 1),
            BottomRight => (2, 2),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct TicTacToeBoard {
    active_player: Option<Player>,
    state_data: [Option<Player>; 9],
}
impl TicTacToeBoard {
    pub fn new() -> Self {
        TicTacToeBoard {
            active_player: Some(Player::X),
            state_data: [None; 9],
        }
    }

    pub fn get_position(&self, position: Position) -> &Option<Player> {
        let (r, c) = position.into();
        &self.state_data[(r * 3 + c) as usize]
    }
    pub fn get_position_mut(&mut self, position: Position) -> &mut Option<Player> {
        let (r, c) = position.into();
        &mut self.state_data[(r * 3 + c) as usize]
    }

    pub fn winner(&self) -> Option<Player> {
        for row in 0..3 {
            if let Some(player) = self.state_data[row * 3] {
                if Some(player) == self.state_data[row * 3 + 1]
                    && Some(player) == self.state_data[row * 3 + 2]
                {
                    return Some(player);
                }
            }
        }
        for col in 0..3 {
            if let Some(player) = self.state_data[col] {
                if Some(player) == self.state_data[col + 3]
                    && Some(player) == self.state_data[col + 6]
                {
                    return Some(player);
                }
            }
        }
        if let Some(player) = self.state_data[2] {
            if Some(player) == self.state_data[4] && Some(player) == self.state_data[6] {
                return Some(player);
            }
        }
        if let Some(player) = self.state_data[0] {
            if Some(player) == self.state_data[4] && Some(player) == self.state_data[8] {
                return Some(player);
            }
        }
        None
    }
}
impl GameState for TicTacToeBoard {
    type Action = Position;
    type Agent = Player;

    fn next_agent(&self) -> Option<Self::Agent> {
        self.active_player
    }

    fn successor(&self, action: &Self::Action) -> Option<Self> {
        if self.active_player.is_none() || self.get_position(*action).is_some() {
            None
        } else {
            let mut successor = self.clone();
            *successor.get_position_mut(*action) = self.active_player;
            if successor.winner().is_some() || successor.state_data.iter().all(Option::is_some) {
                successor.active_player = None;
            } else {
                successor.active_player = Some(successor.active_player.unwrap().other());
            }
            Some(successor)
        }
    }
}
impl Display for TicTacToeBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let square_to_string = |p: Option<Player>| match p {
            None => " ",
            Some(Player::X) => "X",
            Some(Player::O) => "O",
        };
        write!(
            f,
            "{} | {} | {}\n--+---+--\n{} | {} | {}\n--+---+--\n{} | {} | {}",
            square_to_string(self.state_data[0]),
            square_to_string(self.state_data[1]),
            square_to_string(self.state_data[2]),
            square_to_string(self.state_data[3]),
            square_to_string(self.state_data[4]),
            square_to_string(self.state_data[5]),
            square_to_string(self.state_data[6]),
            square_to_string(self.state_data[7]),
            square_to_string(self.state_data[8]),
        )
    }
}
