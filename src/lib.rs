//! `allwords` is a crate that allows you to generate words over a given alphabet.
//!
//! Word generation can be useful in several scenarios:
//!
//!  - Pseudo-random data generation (e.g. testing / mocking)
//!  - Brute forcing of keys / passwords
//!  - Id or Serial number generation
//!
//! The basic idea for using this library is that you create an [`Alphabet`] with a set of
//! characters and then you can use it to generate a [`WordsIterator`]. You can use the iterator
//! to generate all the possible words over the alphabet.
//!
//! For instance if you want to generate all the possible words containing `"a"`, `"b"`, `"c"` with
//! a maximum length of 3 chars:
//!
//! ```rust
//! use allwords::{Alphabet};
//!
//! let a = Alphabet::from_chars_in_str("abc").unwrap();
//!
//! let words: Vec<String> = a.all_words(Some(3)).collect();
//!
//! let expected_words: Vec<String> = [
//!     "a", "b", "c",
//!     "aa", "ab", "ac", "ba", "bb", "bc", "ca", "cb", "cc",
//!     "aaa", "aab", "aac", "aba", "abb", "abc", "aca", "acb", "acc",
//!     "baa", "bab", "bac", "bba", "bbb", "bbc", "bca", "bcb", "bcc",
//!     "caa", "cab", "cac", "cba", "cbb", "cbc", "cca", "ccb", "ccc"]
//!     .iter()
//!     .map(|s| s.to_string())
//!     .collect();
//!
//! assert_eq!(words, expected_words);
//! ```

use std::collections::{HashMap, VecDeque};
use std::str;

/// A representation of an alphabet
pub struct Alphabet {
    /// An hashmap used to track what's the next character for every given character.
    /// The last caracter will point to None.
    pub next_char_map: HashMap<char, Option<char>>,
    /// The first character in the alphabet
    pub first_char: char,
}

/// A iterator that can generate words for a given alphabet
pub struct WordsIterator<'a> {
    /// The reference alphabet instance
    pub alphabet: &'a Alphabet,
    max_len: Option<usize>,
    next_item: String,
}

impl Alphabet {
    /// Creates a new alphabet starting from the unique characters found in a given string.
    ///
    /// This function will extract all the unique characters found in order in the given string.
    /// It will return an `Err` if there are less than 2 unique characters in the given string.
    ///
    /// # Arguments
    ///
    /// * `alphabet_str` - A string-like instance that contains the sequence of characters that
    ///     we want to use to initialize our `Alphabet` instance
    ///
    /// # Returns
    ///
    /// It returns a Result containing the new `Alphabet` instance in case of success.
    ///
    /// # Examples
    ///
    /// Creates an alphabet using characters from `'a'` to `'f'`:
    ///
    /// ```rust
    /// use allwords::{Alphabet};
    ///
    /// let alphabet = Alphabet::from_chars_in_str("abcdef").unwrap();
    /// ```
    ///
    /// Passing an empty string or a string with less than 2 unique chars will return an error:
    ///
    /// ```rust
    /// use allwords::{Alphabet};
    ///
    /// let a = Alphabet::from_chars_in_str("zzzzzzzzzzzzzz");
    ///
    /// match a {
    ///     Ok(_) => panic!("An alphabet was created when we expected an error"),
    ///     Err(e) => assert_eq!(
    ///         e,
    ///         String::from("Invalid alphabet string. Found less than 2 unique chars")
    ///     ),
    /// };
    /// ```
    ///
    /// Since `Alphabet` implements `str::FromStr`, you can also do the following:
    ///
    /// ```rust
    /// use allwords::{Alphabet};
    ///
    /// let value = "abcdef";
    /// let alphabet = value.parse::<Alphabet>().unwrap(); // long life to the turbofish!
    /// ```
    pub fn from_chars_in_str<T: AsRef<str>>(alphabet_str: T) -> Result<Self, String> {
        // creates the map of next characters removing duplicates
        let mut next_char_map = HashMap::new();
        let mut first_char: Option<char> = None;

        let mut previous_char: Option<char> = None;
        for c in alphabet_str.as_ref().chars() {
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
        if let Some(pc) = previous_char {
            next_char_map.entry(pc).or_insert(None);
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

    /// Creates an iterator that will generate all the words for a given alphabet. You can optionally
    /// specifify a maximum length, after which, the iterator will terminate.
    ///
    /// # Arguments
    ///
    /// * `max_len` - an optional `usize` that, if present, will specify the maximum length of the
    ///     generated string. If `None` the iterator will be endless.
    ///
    /// # Returns
    ///
    /// An instance of a [`WordsIterator`].
    ///
    /// # Examples
    ///
    /// Creates an iterator with `max_len` = `2` for a given alphabet:
    ///
    /// ```rust
    /// use allwords::{Alphabet};
    ///
    /// let alphabet = Alphabet::from_chars_in_str("01").unwrap();
    /// let iterator = alphabet.all_words(Some(2));
    /// let words: Vec<String> = iterator.collect();
    /// assert_eq!(words, vec!["0", "1", "00", "01", "10", "11"]);
    /// ```
    pub fn all_words(&self, max_len: Option<usize>) -> WordsIterator {
        WordsIterator {
            alphabet: self,
            max_len,
            next_item: String::from(self.first_char),
        }
    }

    /// A shortcut for creating an unbound (endless) iterator for the given alphabet.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use allwords::{Alphabet};
    ///
    /// let alphabet = Alphabet::from_chars_in_str("01").unwrap();
    /// let iterator = alphabet.all_words_unbound(); // equivalent to `alphabet.all_words(None)`
    /// ```
    pub fn all_words_unbound(&self) -> WordsIterator {
        WordsIterator {
            alphabet: self,
            max_len: None,
            next_item: String::from(self.first_char),
        }
    }

    /// Creates an iterator that will generate all the words for a given alphabet starting from a given word.
    /// This method can be useful in case you want to restart a partially completed iteration from another execution or
    /// if you want to distribute computation across indepentend processes or threads.
    ///
    /// **Note:** this method does not check that the starting word complies with the alphabet. If there are characters
    /// in the string that are NOT present in the alphabet, the iterator will consider these characters as last character and
    /// restart the sequence from the first character in the alphabet.
    ///
    /// # Arguments
    ///
    /// * `start_word` - a `String` instance representing the starting string. This string will be returned by the
    ///     first `.next()` call to the iterator).
    /// * `max_len` - an optional `usize` that, if present, will specify the maximum length of the
    ///     generated string. If `None` the iterator will be endless.
    ///
    /// # Returns
    ///
    /// An instance of a [`WordsIterator`].
    ///
    /// # Examples
    ///
    /// Creates an iterator with `max_len` = `2` starting from "01" for a given alphabet:
    ///
    /// ```rust
    /// use allwords::{Alphabet};
    ///
    /// let alphabet = Alphabet::from_chars_in_str("01").unwrap();
    /// let iterator = alphabet.all_words_starting_from(String::from("01"), Some(2));
    /// let words: Vec<String> = iterator.collect();
    /// assert_eq!(words, vec!["01", "10", "11"]);
    /// ```
    pub fn all_words_starting_from(
        &self,
        start_word: String,
        max_len: Option<usize>,
    ) -> WordsIterator {
        WordsIterator {
            alphabet: self,
            max_len,
            next_item: start_word,
        }
    }

    /// Creates an iterator that will generate all the words for a given alphabet starting from the first word with
    /// a given minimum length.
    ///
    /// # Arguments
    ///
    /// * `start_len` - a `usize` defining the minimum length of the first word emitted by the iterator.
    /// * `max_len` - an optional `usize` that, if present, will specify the maximum length of the
    ///     generated string. If `None` the iterator will be endless.
    ///
    /// # Returns
    ///
    /// An instance of a [`WordsIterator`].
    ///
    /// # Examples
    ///
    /// Creates an iterator for all the words with length bethween 2 and 3 chars:
    ///
    /// ```rust
    /// use allwords::{Alphabet};
    ///
    /// let alphabet = Alphabet::from_chars_in_str("01").unwrap();
    /// let iterator = alphabet.all_words_with_len(2, Some(3));
    /// let words: Vec<String> = iterator.collect();
    /// assert_eq!(words, vec!["00", "01", "10", "11", "000", "001", "010", "011", "100", "101", "110", "111"]);
    /// ```
    pub fn all_words_with_len(&self, start_len: usize, max_len: Option<usize>) -> WordsIterator {
        WordsIterator {
            alphabet: self,
            max_len,
            next_item: (0..start_len).map(|_| self.first_char).collect::<String>(),
        }
    }
}

impl<'a> Iterator for WordsIterator<'a> {
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
        Alphabet::from_chars_in_str(&String::from(s))
    }
}

#[cfg(test)]
mod test;
