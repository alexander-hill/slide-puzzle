extern crate slide_puzzle;

use std::io::stdin;

use slide_puzzle::game::*;
use slide_puzzle::search::a_star;

macro_rules! main_puzzle_board {
    ( $x:expr ) => {
        match $x {
            None => {
                println!("Oh no! I can’t make a puzzle board out of that! 😦");
                return;
            },
            Some(board) => board
        }
    };
}

fn main() {
    println!("Welcome to the world’s least user-friendly sliding puzzle solver!😼");

    println!("Please enter your puzzle, one line per cell in row-major order.");
    println!("(End with a blank line.)");
    let start_board = main_puzzle_board!(Board::from_vec(read_numbers()));

    let goal_board = Board::from_vec(
        (1 .. start_board.side() * start_board.side() + 1)
            .map(|i| match i {
                _ if i < start_board.side() => i as u8,
                _ if i == start_board.side() => 0,
                _ => (i - 1) as u8
            })
            .collect()
        ).unwrap();

    println!("Let me think about that.");
    match a_star(start_board, &goal_board, &ALL_MOVES) {
        None => println!("That puzzle doesn’t appear to have a solution. 😬"),
        Some(moves) => {
            println!("Follow this sequence of moves:");
            for mov in moves {
                println!("{:?}", mov);
            }
        }
    }
}

fn read_numbers() -> Vec<u8> {
    let mut numbers = Vec::new();

    loop {
        let mut user_input = String::new();
        stdin().read_line(&mut user_input).ok().expect("Failed to read line");

        let trimmed = user_input.trim();
        if trimmed.len() == 0 {
            break
        }

        numbers.push(trimmed.parse()
                     .ok()
                     .expect("That doesn’t look like a number."));
    }

    numbers
}
