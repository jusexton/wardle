use std::collections::HashMap;

use crate::Args;

pub struct WordlePredicate {
    wrong_positions: Option<HashMap<char, u8>>,
    correct_positions: Option<HashMap<usize, char>>,
}

impl WordlePredicate {
    fn new(wrong_positions: Option<String>, correct_positions: Option<String>) -> Self {
        let wrong_positions = WordlePredicate::wrong_positions_map(wrong_positions);
        let correct_positions = WordlePredicate::correct_position_map(correct_positions);
        WordlePredicate {
            wrong_positions,
            correct_positions,
        }
    }

    /// Builds a character frequency map of the wrong characters and their expected counts.
    fn wrong_positions_map(wrong_positions: Option<String>) -> Option<HashMap<char, u8>> {
        wrong_positions.map(|value| char_frequency_map(&value))
    }

    /// Builds a map of the correct characters and their positions.
    fn correct_position_map(correct_positions: Option<String>) -> Option<HashMap<usize, char>> {
        correct_positions.map(|value| {
            value
                .chars()
                .enumerate()
                .filter(|(_, character)| *character != '_')
                .fold(HashMap::new(), |mut acc, (index, character)| {
                    acc.insert(index, character);
                    acc
                })
        })
    }

    /// Returns whether a given word matches against the provided wordle details. If the word
    /// matches, that means its an eligible candidate.
    pub fn matches(&self, word: &str) -> bool {
        // Validates the given word contains all specified wrong position characters
        let contains_wrong_positions = self.wrong_positions.as_ref().map_or(true, |positions| {
            let freq_map = char_frequency_map(word);
            positions.iter().all(|(c, count)| {
                let actual_count = freq_map.get(c).unwrap_or(&0);
                actual_count >= count
            })
        });

        // Validates the given word contains the correct letters in the correct position
        let contains_correct_positions =
            self.correct_positions.as_ref().map_or(true, |positions| {
                word.chars().enumerate().all(|(index, character)| {
                    let correct = positions.get(&index);
                    correct.map_or(true, |c| *c == character)
                })
            });

        contains_wrong_positions && contains_correct_positions
    }
}

impl From<Args> for WordlePredicate {
    fn from(args: Args) -> Self {
        WordlePredicate::new(args.wrong_positions, args.correct)
    }
}

fn char_frequency_map(s: &str) -> HashMap<char, u8> {
    s.chars().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    })
}

#[cfg(test)]
mod tests {
    use crate::predicate::WordlePredicate;

    #[test]
    fn wordle_predicate_should_match_words_that_contain_wrong_position_characters() {
        let wrong_positions = Some(String::from("iv"));
        let correct_positions = None;
        let predicate = WordlePredicate::new(wrong_positions, correct_positions);

        // Valid words
        assert_eq!(predicate.matches("video"), true);
        assert_eq!(predicate.matches("ivey"), true);
        assert_eq!(predicate.matches("iv"), true);
        assert_eq!(predicate.matches("valid"), true);

        // Invalid words
        assert_eq!(predicate.matches("test"), false);
        assert_eq!(predicate.matches("nope"), false);
        assert_eq!(predicate.matches("false"), false);
    }

    #[test]
    fn wordle_predicate_should_match_words_that_contain_duplicate_wrong_position_characters() {
        let wrong_positions = Some(String::from("ll"));
        let correct_positions = None;
        let predicate = WordlePredicate::new(wrong_positions, correct_positions);

        // Valid words
        assert_eq!(predicate.matches("allow"), true);
        assert_eq!(predicate.matches("all over"), true);

        // Invalid words
        assert_eq!(predicate.matches("almost"), false);
        assert_eq!(predicate.matches("alto"), false);
        assert_eq!(predicate.matches("false"), false);
    }

    #[test]
    fn wordle_predicate_should_match_words_that_contain_correct_position_characters() {
        let wrong_positions = None;
        let correct_positions = Some(String::from("vi___"));
        let predicate = WordlePredicate::new(wrong_positions, correct_positions);

        // Valid words
        assert_eq!(predicate.matches("video"), true);
        assert_eq!(predicate.matches("virus"), true);
        assert_eq!(predicate.matches("vital"), true);

        // Invalid words
        assert_eq!(predicate.matches("vote"), false);
        assert_eq!(predicate.matches("nope"), false);
        assert_eq!(predicate.matches("false"), false);
    }

    #[test]
    fn wordle_predicate_should_match_words_that_contain_correct_and_wrong_position_characters() {
        let wrong_positions = Some(String::from("ll"));
        let correct_positions = Some(String::from("a____"));
        let predicate = WordlePredicate::new(wrong_positions, correct_positions);

        // Valid words
        assert_eq!(predicate.matches("allow"), true);
        assert_eq!(predicate.matches("all"), true);

        // Invalid words
        assert_eq!(predicate.matches("almost"), false);
        assert_eq!(predicate.matches("false"), false);
    }
}
