mod puzzle;
use std::{io::{self}, process::exit};

use puzzle::Puzzle;

fn main() {
    let puzzle = match Puzzle::import("puzzle.yaml") {
        Ok(p) => p,
        Err(e) => {
            println!("{e}");
            exit(1)
        }
    };
    loop {
        if puzzle.check() {
            puzzle.display();
            println!("You Win!");
            break;
        }
        puzzle.display();

        let mut play = String::new();

        io::stdin()
            .read_line(&mut play)
            .expect("Failed to read line");

        let play: Vec<&str> = play.trim().split(" ").collect();
        if play.len() != 3 {
            println!("Wrong command");
            discard_input();
            continue;
        }

        let (play, x, y) = (play[0], play[1].parse(), play[2].parse());
        let (x, y) = match (x, y) {
            (Ok(a), Ok(b)) => (a, b),
            _ => {
                println!("Wrong command");
                discard_input();
                continue;
            }
        };
        let play = puzzle::Play::build(play);
        match play {
            Some(p) => puzzle.act_on_cell(p, x, y),
            None => {
                println!("Wrong command");
                discard_input();
                continue;
            }
        }
    }
}

fn discard_input() {
    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("Failed to read line");
}