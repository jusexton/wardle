use std::collections::HashSet;

use rand::Rng;

use crate::predicate::WordlePredicate;

pub struct App {
    words: Vec<String>,
}

impl App {
    pub fn new(words: Vec<String>) -> Self {
        App { words }
    }

    pub fn handle_random(&self, count: &Option<u8>) {
        if let Some(count) = count {
            let words: String = self
                .random_words(*count as usize)
                .into_iter()
                .map(String::from)
                .collect::<Vec<String>>()
                .join(", ");

            println!("Randomly Selected Words: {}", words)
        } else {
            let word = self.random_word();
            println!("Randomly Selected Word: {}", word)
        }
    }

    fn random_word(&self) -> &str {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..self.words.len());
        self.words.get(index).unwrap()
    }

    fn random_words(&self, count: usize) -> HashSet<&str> {
        let mut result = HashSet::with_capacity(count);
        while result.len() != count {
            result.insert(self.random_word());
        }
        result
    }

    pub fn handle_eligible(
        &self,
        correct_positions: &Option<String>,
        wrong_positions: &Option<String>,
        invalid_letters: &Option<String>,
    ) {
        let predicate = WordlePredicate::new(
            wrong_positions.clone(),
            correct_positions.clone(),
            invalid_letters.clone(),
        );
        let eligible_guesses = self
            .words
            .iter()
            .filter(|word| predicate.matches(word.as_str()))
            .collect::<Vec<&String>>();

        println!("Eligible Wordle Contenders: {:?}", &eligible_guesses);
    }
}
