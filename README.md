# allwords

[![build badge](https://github.com/lmammino/allwords/workflows/Rust/badge.svg)](https://github.com/lmammino/allwords/actions?query=workflow%3ARust)
[![codecov](https://codecov.io/gh/lmammino/allwords/branch/master/graph/badge.svg)](https://codecov.io/gh/lmammino/allwords)
[![crates.io badge](https://img.shields.io/crates/v/allwords.svg)](https://crates.io/crates/allwords)
[![Documentation](https://docs.rs/allwords/badge.svg)](https://docs.rs/allwords)
[![License: MIT](https://img.shields.io/crates/l/allwords.svg)](#license)

TODO: expand with a better description and more examples


## Usage

```rust
use allwords::Alphabet;

let alphabet = Alphabet::from_chars_string("01").unwrap();
let words: Vec<String> = a.all_words_from_len(3, Some(3)).collect();

println!("{:?}", words); // ["000", "001", "010", "011", "100", "101", "110", "111"]
```
