mod lib;
use lib::*;

use std::io::{self, Stdin, Write};

use multiagent::{expectimax, ExpectimaxAgentType, GameState};

fn get_player_move(game: &TicTacToeBoard) -> Position {
    get_input_position(&mut io::stdin(), "Which position would you like to play? [0-9]", &game)
}
fn get_ai_move(game: &TicTacToeBoard) -> Position {
    let action = expectimax(
        game,
        9,
        |state| match state.winner() {
            Some(Player::O) => 1.,
            Some(Player::X) => -1.,
            None => 0.,
        },
        // If we're being formally correct, you would need to actually figure out which one the AI
        // is playing as. I'm too lazy to implement it correctly, so I just change this to
        // whichever one the AI is playing as this time.
        // That would be bad if I was trying to actually write something good, but the point of
        // this is just to be an example of how to use the library, so it's good enough for me.
        |player| if player == &Player::O { ExpectimaxAgentType::Maximizer } else { ExpectimaxAgentType::Random },
    );
    println!("AI played: {:?}", action);
    action.0.unwrap()
}

fn main() {
    let (x_move, o_move): (&dyn Fn(&TicTacToeBoard) -> Position, &dyn Fn(&TicTacToeBoard) -> Position) = if affirm(&mut io::stdin(), "Would you like to be X?") {
        (&get_player_move, &get_ai_move)
    } else {
        (&get_ai_move, &get_player_move)
    };
    let mut game = TicTacToeBoard::new();
    loop {
        println!("Current board:\n{}", game);
        let action = match game.next_agent() {
            Some(Player::X) => {
                x_move(&game)
            },
            Some(Player::O) => {
                o_move(&game)
            },
            None => break,
        };
        game = game.successor(&action).unwrap();
    }
}

fn affirm(stdin: &mut Stdin, question: &str) -> bool {
    loop {
        print!("{} [Y/n]", question);
        io::stdout().flush().unwrap();
        let mut response = String::new();
        stdin.read_line(&mut response).unwrap();
        response = String::from(response.trim());
        if response == "Y" || response == "y" {
            break true;
        } else if response == "N" || response == "n" {
            break false;
        } else {
            println!("I didn't catch that.");
        }
    }
}

fn get_input_position(stdin: &mut Stdin, prompt: &str, board: &TicTacToeBoard) -> Position {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        let mut response = String::new();
        stdin.read_line(&mut response).unwrap();
        response = String::from(response.trim());
        let position = match response.as_str() {
            "0" => Position::TopLeft,
            "1" => Position::TopCenter,
            "2" => Position::TopRight,
            "3" => Position::CenterLeft,
            "4" => Position::Center,
            "5" => Position::CenterRight,
            "6" => Position::BottomLeft,
            "7" => Position::BottomCenter,
            "8" => Position::BottomRight,
            _ => {
                println!("I didn't catch that.");
                continue;
            }
        };
        if board.is_legal(&position) {
            break position;
        } else {
            println!("That square has already been played.");
        }
    }
}
