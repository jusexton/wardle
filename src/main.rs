use std::collections::HashSet;

use clap::Parser;

use crate::predicate::WordlePredicate;

mod predicate;

static FIVE_LETTER_WORDS: &'static str = include_str!("words/five-letters.txt");

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// The letters that are correct and in the correct position.
    /// To preserve positions, this value should contain underscores for all
    /// other unknown characters. ex. The word video might look like the following when
    /// the i and d have not been discovered: v__eo
    #[clap(short, long)]
    correct_positions: Option<String>,

    /// The letters that are in the word but not in the correct position
    #[clap(short, long)]
    wrong_positions: Option<String>,
}

fn main() {
    let args = Args::parse();

    let predicate = WordlePredicate::from(args);
    let words = get_words();
    let eligible_guesses = words
        .into_iter()
        .filter(|word| predicate.matches(word))
        .collect::<Vec<&str>>();

    println!("Eligible Wordle Contenders: {:?}", &eligible_guesses);
}

fn get_words() -> HashSet<&'static str> {
    FIVE_LETTER_WORDS.lines().collect()
}
