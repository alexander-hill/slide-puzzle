mod search;
mod game;

use std::io::stdin;

use game::*;
use search::a_star;

macro_rules! main_puzzle_board {
    ( $x:expr ) => {
        match $x {
            None => {
                println!("Oh no! I can’t make a puzzle board out of that string! 😦");
                return;
            },
            Some(board) => board
        }
    };
}

fn main() {
    println!("Welcome to the world’s least user-friendly sliding puzzle solver!😼");
    println!("Please enter your puzzle, as a string of the nine numbers 0–8.");

    let mut user_input = String::new();
    stdin().read_line(&mut user_input).ok().expect("Failed to read line");

    let start_board = main_puzzle_board!(board_of_string(&user_input));

    println!("Now, please enter the target configuration.");
    user_input.clear();
    stdin().read_line(&mut user_input).ok().expect("Failed to read line");
    let goal_board = main_puzzle_board!(board_of_string(&user_input));

    match a_star(start_board, &goal_board, &all_moves) {
        None => println!("That puzzle doesn’t appear to have a solution. 😬"),
        Some(moves) => {
            println!("Follow this sequence of moves:");
            for mov in moves {
                println!("{:?}", mov);
            }
        }
    }
}

fn board_of_string(s: &str) -> Option<Board> {
    let trimmed = s.trim();
    if trimmed.len() != 9 {
        return None
    }

    let mut storage = [0; 9];
    for (i, b) in trimmed.bytes().enumerate() {
        if b < 0x30 || b > 0x39 {
            return None;
        }

        storage[i] = b - 0x30;
    }

    Board::from_array(storage)
}
