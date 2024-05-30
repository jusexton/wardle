use std::collections::{HashMap, HashSet};

pub struct WordlePredicate {
    wrong_positions: Option<HashMap<char, u8>>,
    correct_positions: Option<HashMap<usize, char>>,
    invalid_letters: Option<HashSet<char>>,
}

impl WordlePredicate {
    pub fn new(
        wrong_positions: Option<String>,
        correct_positions: Option<String>,
        invalid_letters: Option<String>,
    ) -> Self {
        let wrong_positions = WordlePredicate::wrong_positions_map(wrong_positions);
        let correct_positions = WordlePredicate::correct_position_map(correct_positions);
        let invalid_letters = WordlePredicate::invalid_letter_set(invalid_letters);
        WordlePredicate {
            wrong_positions,
            correct_positions,
            invalid_letters,
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

    fn invalid_letter_set(invalid_letters: Option<String>) -> Option<HashSet<char>> {
        invalid_letters.map(|value| value.chars().collect())
    }

    /// Returns whether a given word matches against the provided wordle details. If the word
    /// matches, that means its an eligible candidate.
    pub fn matches(&self, word: &str) -> bool {
        // TODO: Potential performance improvement of doing these validations in one pass
        //  of all the word characters.

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

        // Validates the given word does not contain invalid letters
        let doesnt_contain_invalid_letters =
            self.invalid_letters.as_ref().map_or(true, |letters| {
                !word.chars().any(|letter| letters.contains(&letter))
            });

        contains_wrong_positions && contains_correct_positions && doesnt_contain_invalid_letters
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
        let invalid_letters = None;
        let predicate = WordlePredicate::new(wrong_positions, correct_positions, invalid_letters);

        // Valid words
        assert!(predicate.matches("video"));
        assert!(predicate.matches("ivey"));
        assert!(predicate.matches("iv"));
        assert!(predicate.matches("valid"));

        // Invalid words
        assert!(!predicate.matches("test"));
        assert!(!predicate.matches("nope"));
        assert!(!predicate.matches("false"));
    }

    #[test]
    fn wordle_predicate_should_match_words_that_contain_duplicate_wrong_position_characters() {
        let wrong_positions = Some(String::from("ll"));
        let correct_positions = None;
        let invalid_letters = None;
        let predicate = WordlePredicate::new(wrong_positions, correct_positions, invalid_letters);

        // Valid words
        assert!(predicate.matches("allow"));
        assert!(predicate.matches("all over"));

        // Invalid words
        assert!(!predicate.matches("almost"));
        assert!(!predicate.matches("alto"));
        assert!(!predicate.matches("false"));
    }

    #[test]
    fn wordle_predicate_should_match_words_that_contain_correct_position_characters() {
        let wrong_positions = None;
        let correct_positions = Some(String::from("vi___"));
        let invalid_letters = None;
        let predicate = WordlePredicate::new(wrong_positions, correct_positions, invalid_letters);

        // Valid words
        assert!(predicate.matches("video"));
        assert!(predicate.matches("virus"));
        assert!(predicate.matches("vital"));

        // Invalid words
        assert!(!predicate.matches("vote"));
        assert!(!predicate.matches("nope"));
        assert!(!predicate.matches("false"));
    }

    #[test]
    fn wordle_predicate_should_match_words_that_contain_correct_and_wrong_position_characters() {
        let wrong_positions = Some(String::from("ll"));
        let correct_positions = Some(String::from("a____"));
        let invalid_letters = None;
        let predicate = WordlePredicate::new(wrong_positions, correct_positions, invalid_letters);

        // Valid words
        assert!(predicate.matches("allow"));
        assert!(predicate.matches("all"));

        // Invalid words
        assert!(!predicate.matches("almost"));
        assert!(!predicate.matches("false"));
    }
}
