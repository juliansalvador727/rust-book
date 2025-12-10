// use std::cmp::Ordering;
use std::io;

use rand::Rng;

#[derive(PartialEq, Debug, Copy, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn from_u32(n: u32) -> Option<Self> {
        match n {
            1 => Some(Move::Rock),
            2 => Some(Move::Paper),
            3 => Some(Move::Scissors),
            _ => None,
        }
    }

    fn beats(self, other: Move) -> bool {
        matches!(
            (self, other),
            (Move::Rock, Move::Scissors)
                | (Move::Paper, Move::Rock)
                | (Move::Scissors, Move::Paper)
        )
    }
}

fn main() {
    println! {"Let's play rock paper scissors."};

    let mut rng = rand::thread_rng();
    loop {
        println!("\nChoose: Rock (1), Paper (2), or Scissors(3), or 0 to quit:");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let num: u32 = match input.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Please type a number between 1 and 3.");
                continue;
            }
        };

        if num == 0 {
            println!("Goodbye~!");
            break;
        }

        let player = match Move::from_u32(num) {
            Some(m) => m,
            None => {
                println!("Please enter 1, 2, or 3.");
                continue;
            }
        };

        let foe = Move::from_u32(rng.gen_range(1..=3)).unwrap();

        println!("You chose {:?}, opponent chose {:?}.", player, foe);

        if player == foe {
            println!("Tie!");
        } else if player.beats(foe) {
            println!("You win!");
        } else {
            println!("You lose!");
        }
    }
}
