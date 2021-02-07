use super::*;
use std::collections::HashSet;

#[test]
fn it_creates_an_alphabet_from_a_string_with_few_unique_chars() {
    let a = Alphabet::from_chars_in_str("ab").unwrap();

    let expected_map: HashMap<char, Option<char>> =
        [('a', Some('b')), ('b', None)].iter().cloned().collect();

    assert_eq!(a.first_char, 'a');
    assert_eq!(a.next_char_map, expected_map);
}

#[test]
fn it_creates_an_alphabet_from_a_string_with_many_unique_chars() {
    let a = Alphabet::from_chars_in_str("abcde").unwrap();

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
    let a = Alphabet::from_chars_in_str("aaabbbcccddddebbbeeea").unwrap();

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
    let a = Alphabet::from_chars_in_str("😀😃😄😁😅").unwrap();

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
    let a = Alphabet::from_chars_in_str("");

    match a {
        Ok(_) => panic!("An alphabet was created when we expected an error"),
        Err(e) => assert_eq!(
            e,
            String::from("Invalid alphabet string. Found less than 2 unique chars")
        ),
    };

    let a = Alphabet::from_chars_in_str("aaaaaaaaaaa");

    match a {
        Ok(_) => panic!("An alphabet was created when we expected an error"),
        Err(e) => assert_eq!(
            e,
            String::from("Invalid alphabet string. Found less than 2 unique chars")
        ),
    };
}

#[test]
fn it_can_generate_all_words_up_to_a_certain_length() {
    let a = Alphabet::from_chars_in_str("01").unwrap();

    let words: Vec<String> = a.all_words(Some(3)).collect();

    let expected_words: Vec<String> = [
        "0", "1", "00", "01", "10", "11", "000", "001", "010", "011", "100", "101", "110", "111",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    assert_eq!(words, expected_words);
}

#[test]
fn it_can_generate_all_words_endlessly() {
    let a = Alphabet::from_chars_in_str("ab").unwrap();

    // testing infinite is... tough :) so we just limit this to 1000 items and check that they are all different
    let words: Vec<String> = a.all_words_unbound().take(1000).collect();
    let unique_words: HashSet<String> = words.iter().cloned().collect();

    assert_eq!(unique_words.len(), 1000);
}

#[test]
fn it_can_generate_all_words_up_to_a_certain_length_from_a_starting_string() {
    let a = Alphabet::from_chars_in_str("01").unwrap();

    let words: Vec<String> = a
        .all_words_starting_from(String::from("011"), Some(3))
        .collect();

    let expected_words: Vec<String> = ["011", "100", "101", "110", "111"]
        .iter()
        .map(|s| s.to_string())
        .collect();

    assert_eq!(words, expected_words);
}

#[test]
fn it_can_generate_all_words_from_a_given_length_up_to_another_length() {
    let a = Alphabet::from_chars_in_str("01").unwrap();

    let words: Vec<String> = a.all_words_with_len(3, Some(3)).collect();

    let expected_words: Vec<String> = ["000", "001", "010", "011", "100", "101", "110", "111"]
        .iter()
        .map(|s| s.to_string())
        .collect();

    assert_eq!(words, expected_words);
}
