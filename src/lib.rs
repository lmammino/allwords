use std::collections::{HashMap, VecDeque};
use std::str;

pub struct Alphabet {
    next_char_map: HashMap<char, Option<char>>,
    first_char: char,
}

pub struct AlphabetIterator<'a> {
    alphabet: &'a Alphabet,
    max_len: Option<usize>,
    next_item: String,
}

impl Alphabet {
    pub fn from_chars_string<T: AsRef<str>>(alphabet: T) -> Result<Self, String> {
        let mut next_char_map = HashMap::new();
        let mut first_char: Option<char> = None;

        let mut previous_char: Option<char> = None;
        for c in alphabet.as_ref().chars() {
            if first_char.is_none() {
                first_char = Some(c);
                previous_char = Some(c);
            } else if previous_char.is_some()
                && previous_char.unwrap() != c
                && !next_char_map.contains_key(&c)
            {
                next_char_map.insert(previous_char.unwrap(), Some(c));
                previous_char = Some(c);
            }
        }
        // adds last char if hasn't been added yet
        if previous_char.is_some() && !next_char_map.contains_key(&previous_char.unwrap()) {
            next_char_map.insert(previous_char.unwrap(), None);
        }

        if next_char_map.keys().len() < 2 {
            return Err(String::from(
                "Invalid alphabet string. Found less than 2 unique chars",
            ));
        }

        Ok(Alphabet {
            next_char_map,
            first_char: first_char.unwrap(),
        })
    }

    pub fn all_words<'a>(&'a self, max_len: Option<usize>) -> AlphabetIterator<'a> {
        AlphabetIterator {
            alphabet: self,
            max_len: max_len,
            next_item: String::from(self.first_char),
        }
    }

    pub fn all_words_unbound<'a>(&'a self) -> AlphabetIterator<'a> {
        AlphabetIterator {
            alphabet: self,
            max_len: None,
            next_item: String::from(self.first_char),
        }
    }

    pub fn all_words_from<'a>(
        &'a self,
        start_word: String,
        max_len: Option<usize>,
    ) -> AlphabetIterator<'a> {
        AlphabetIterator {
            alphabet: self,
            max_len: max_len,
            next_item: start_word,
        }
    }

    pub fn all_words_from_len<'a>(
        &'a self,
        start_len: usize,
        max_len: Option<usize>,
    ) -> AlphabetIterator<'a> {
        AlphabetIterator {
            alphabet: self,
            max_len: max_len,
            next_item: (0..start_len).map(|_| self.first_char).collect::<String>(),
        }
    }
}

impl<'a> Iterator for AlphabetIterator<'a> {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        if self.max_len.is_some() && self.max_len.unwrap() < self.next_item.len() {
            return None;
        }

        let current_item = self.next_item.clone();
        let mut next_item: VecDeque<char> = VecDeque::with_capacity(current_item.len() + 1);
        let mut carry = true;
        for c in current_item.chars().rev() {
            if carry {
                let next_char = self.alphabet.next_char_map.get(&c).unwrap_or(&None);
                let next_char = match next_char {
                    Some(c) => {
                        carry = false;
                        *c
                    }
                    None => {
                        carry = true;
                        self.alphabet.first_char
                    }
                };
                next_item.push_front(next_char);
            } else {
                next_item.push_front(c);
            }
        }
        if carry {
            next_item.push_front(self.alphabet.first_char);
        }
        let next_item: String = next_item.iter().collect();
        self.next_item = next_item;

        Some(current_item)
    }
}

impl str::FromStr for Alphabet {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Alphabet::from_chars_string(&String::from(s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn it_creates_an_alphabet_from_a_string_with_few_unique_chars() {
        let a = Alphabet::from_chars_string("ab").unwrap();

        let expected_map: HashMap<char, Option<char>> =
            [('a', Some('b')), ('b', None)].iter().cloned().collect();

        assert_eq!(a.first_char, 'a');
        assert_eq!(a.next_char_map, expected_map);
    }

    #[test]
    fn it_creates_an_alphabet_from_a_string_with_many_unique_chars() {
        let a = Alphabet::from_chars_string("abcde").unwrap();

        let expected_map: HashMap<char, Option<char>> = [
            ('a', Some('b')),
            ('b', Some('c')),
            ('c', Some('d')),
            ('d', Some('e')),
            ('e', None),
        ]
        .iter()
        .cloned()
        .collect();

        assert_eq!(a.first_char, 'a');
        assert_eq!(a.next_char_map, expected_map);
    }

    #[test]
    fn it_creates_an_alphabet_from_a_string_with_duplicate_chars() {
        let a = Alphabet::from_chars_string("aaabbbcccddddebbbeeea").unwrap();

        let expected_map: HashMap<char, Option<char>> = [
            ('a', Some('b')),
            ('b', Some('c')),
            ('c', Some('d')),
            ('d', Some('e')),
            ('e', None),
        ]
        .iter()
        .cloned()
        .collect();

        assert_eq!(a.first_char, 'a');
        assert_eq!(a.next_char_map, expected_map);
    }

    #[test]
    fn it_creates_an_alphabet_from_a_string_with_unicode_chars() {
        let a = Alphabet::from_chars_string("😀😃😄😁😅").unwrap();

        let expected_map: HashMap<char, Option<char>> = [
            ('😀', Some('😃')),
            ('😃', Some('😄')),
            ('😄', Some('😁')),
            ('😁', Some('😅')),
            ('😅', None),
        ]
        .iter()
        .cloned()
        .collect();

        assert_eq!(a.first_char, '😀');
        assert_eq!(a.next_char_map, expected_map);
    }

    #[test]
    fn it_fails_if_alphabet_doesnt_have_at_least_2_unique_chars() {
        let a = Alphabet::from_chars_string("");

        match a {
            Ok(_) => assert!(false),
            Err(e) => assert_eq!(
                e,
                String::from("Invalid alphabet string. Found less than 2 unique chars")
            ),
        };

        let a = Alphabet::from_chars_string("aaaaaaaaaaa");

        match a {
            Ok(_) => assert!(false),
            Err(e) => assert_eq!(
                e,
                String::from("Invalid alphabet string. Found less than 2 unique chars")
            ),
        };
    }

    #[test]
    fn it_can_generate_all_words_up_to_a_certain_length() {
        let a = Alphabet::from_chars_string("01").unwrap();

        let words: Vec<String> = a.all_words(Some(3)).collect();

        let expected_words: Vec<String> = [
            "0", "1", "00", "01", "10", "11", "000", "001", "010", "011", "100", "101", "110",
            "111",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        assert_eq!(words, expected_words);
    }

    #[test]
    fn it_can_generate_all_words_endlessly() {
        let a = Alphabet::from_chars_string("ab").unwrap();

        // testing infinite is... tough :) so we just limit this to 1000 items and check that they are all different
        let words: Vec<String> = a.all_words_unbound().take(1000).collect();
        let unique_words: HashSet<String> = words.iter().cloned().collect();

        assert_eq!(unique_words.len(), 1000);
    }

    #[test]
    fn it_can_generate_all_words_up_to_a_certain_length_from_a_starting_string() {
        let a = Alphabet::from_chars_string("01").unwrap();

        let words: Vec<String> = a.all_words_from(String::from("011"), Some(3)).collect();

        let expected_words: Vec<String> = ["011", "100", "101", "110", "111"]
            .iter()
            .map(|s| s.to_string())
            .collect();

        assert_eq!(words, expected_words);
    }

    #[test]
    fn it_can_generate_all_words_from_a_given_length_up_to_another_length() {
        let a = Alphabet::from_chars_string("01").unwrap();

        let words: Vec<String> = a.all_words_from_len(3, Some(3)).collect();

        let expected_words: Vec<String> = ["000", "001", "010", "011", "100", "101", "110", "111"]
            .iter()
            .map(|s| s.to_string())
            .collect();

        assert_eq!(words, expected_words);
    }
}
