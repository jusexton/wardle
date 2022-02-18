use clap::{Parser, Subcommand};

use crate::app::App;
use crate::Command::{Eligible, Random};

mod app;
mod predicate;

static FIVE_LETTER_WORDS: &'static str = include_str!("words/five-letters.txt");

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Generate a random word or words  the game can be started with.
    Random {
        /// The number of random words that should be displayed
        #[clap(short, long)]
        count: Option<u8>,
    },
    /// Calculates all eligible wordle guesses based on given known information.
    Eligible {
        /// The letters that are correct and in the correct position.
        /// To preserve positions, this value should contain underscores for all
        /// other unknown characters. ex. The word video might look like the following when
        /// the i and d have not been discovered: v__eo
        #[clap(short, long)]
        correct_positions: Option<String>,

        /// The letters that are in the word but not in the correct position
        #[clap(short, long)]
        wrong_positions: Option<String>,

        /// Letters that have been used in previous guesses and that are not contained within the word.
        #[clap(short, long)]
        invalid_letters: Option<String>,
    },
}

fn main() {
    let args = Args::parse();
    let words = get_words();
    let app = App::new(words);
    match &args.command {
        Random { count } => app.handle_random(count),
        Eligible {
            correct_positions,
            wrong_positions,
            invalid_letters
        } => app.handle_eligible(correct_positions, wrong_positions, invalid_letters),
    }
}

fn get_words() -> Vec<String> {
    FIVE_LETTER_WORDS.lines().map(String::from).collect()
}
