# allwords 🦀

[![build badge](https://github.com/lmammino/allwords/workflows/Rust/badge.svg)](https://github.com/lmammino/allwords/actions?query=workflow%3ARust)
[![codecov](https://codecov.io/gh/lmammino/allwords/branch/main/graph/badge.svg?token=4CNbvgaDc1)](https://codecov.io/gh/lmammino/allwords)
[![crates.io badge](https://img.shields.io/crates/v/allwords.svg)](https://crates.io/crates/allwords)
[![Documentation](https://docs.rs/allwords/badge.svg)](https://docs.rs/allwords)

`allwords` is a Rust crate that allows you to generate words over a given alphabet.

Word generation can be useful in several scenarios:

  - Pseudo-random data generation (e.g. testing / mocking)
  - Brute forcing of keys / passwords
  - Id or Serial number generation


## Install

To install the library add the following lines to your `Cargo.toml`

```toml
[dependencies]
allwords = "0"
```

Or, if you have [`cargo add`](https://github.com/killercup/cargo-edit), you can run the following command:

```bash
cargo add allwords@0
```


## Sample Usage

The basic idea for using this library is that you create an [`Alphabet`](https://docs.rs/allwords/latest/allwords/struct.Alphabet.html) with a set of
characters and then you can use it to generate a [`WordsIterator`](https://docs.rs/allwords/latest/allwords/struct.WordsIterator.html). You can use the iterator
to generate all the possible words over the alphabet.

For instance if you want to generate all the possible words containing `"a"`, `"b"`, `"c"` with
a maximum length of 3 chars:

```rust
use allwords::{Alphabet};

let a = Alphabet::from_chars_in_str("abc").unwrap();

let words: Vec<String> = a.all_words(Some(3)).collect();

let expected_words: Vec<String> = [
    "a", "b", "c",
    "aa", "ab", "ac", "ba", "bb", "bc", "ca", "cb", "cc",
    "aaa", "aab", "aac", "aba", "abb", "abc", "aca", "acb", "acc",
    "baa", "bab", "bac", "bba", "bbb", "bbc", "bca", "bcb", "bcc",
    "caa", "cab", "cac", "cba", "cbb", "cbc", "cca", "ccb", "ccc"]
    .iter()
    .map(|s| s.to_string())
    .collect();

assert_eq!(words, expected_words);
```

### WordsIterator

Once you create an alphabet `a`, there are 4 different ways to get an iterator:

  - [`a.all_words(max_len)`](https://docs.rs/allwords/latest/allwords/struct.Alphabet.html#method.all_words) - Creates an iterator that will generate all the words for a given alphabet. You can optionally specifify a maximum length, after which, the iterator will terminate.
  - [`a.all_words_unbound()`](https://docs.rs/allwords/latest/allwords/struct.Alphabet.html#method.all_words_unbound) - A shortcut for creating an unbound (endless) iterator for the given alphabet.
  - [`a.all_words_starting_from(start_word, max_len)`](https://docs.rs/allwords/latest/allwords/struct.Alphabet.html#method.all_words_starting_from) - Creates an iterator that will generate all the words for a given alphabet starting from a given word.
  - [`a.all_words_with_len(start_len, max_len)`](https://docs.rs/allwords/latest/allwords/struct.Alphabet.html#method.all_words_with_len) - Creates an iterator that will generate all the words for a given alphabet starting from the first word with a given minimum length.

Consult the [crate documentation](https://docs.rs/allwords/latest/) for more details and examples.


## Contributing

Everyone is very welcome to contribute to this project.
You can contribute just by submitting bugs or suggesting improvements by
[opening an issue on GitHub](https://github.com/lmammino/allwords/issues).


## License

Licensed under [MIT License](LICENSE). © Luciano Mammino.