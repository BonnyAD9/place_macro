# Place-macro
[![crates.io][crates.io-badge]][crates.io]
[![Downloads][downloads-badge]][releases]

Macros you wish you had while you were writing your non-proc macro.

This library privides some macros that make **writing regural** non-proc
macros much **simpler**, readable and with less dirty tricks.

The main macro of this library is `place`. It is able to **expand** the
**macros** in this library **in reverse expansion order**.

If you have some macros that you would like to add feel free to open new issue
on [GitHub][github].

## How to get it

### With cargo
```sh
cargo add place_macro
```

### In Cargo.toml
```toml
[dependencies]
place_macro = "1.0.0"
```

## Macros
+ `place`: expands the following macros in reverse order, see [docs][docs.rs]
   for more info
- `ignore`: expands to nothing
- `identity`: expands to what is given, it bypasses the reverse order in the
  `place` macro
- `dollar`: expands to dollar sign `$`
- `string`: concats the contents into single string, see the doc
- `identifier`: concats the contents into sintle identifier in the same way as
  string
- `head`: expands to the first token
- `tail`: expands to all but the first token
- `start`: expands to all but the last token
- `last`: expands to the last token
- `reverse`: expands to the tokens in reverse order
- `stringify`: expands to string of the input
- `replace_newline`: replaces all newlines and folowing whitespace in literal
  with the given literal
- `str_replace`: replace in string literal
- `to_case`: change the case of a identifier

## Examples
For examples and more detailed info about how to use the macros in this crate
see [docs][docs.rs]

## Links
- **Author:** [BonnyAD9][author]
- **GitHub repository:** [BonnyAD/place_macro][github]
- **Package:** [crates.io][crates.io]
- **Documentation:** [docs.rs][docs.rs]
- **My Website:** [bonnyad9.github.io][website]

[author]: https://github.com/BonnyAD9
[crates.io]: https://crates.io/crates/place_macro
[crates.io-badge]: https://img.shields.io/crates/v/place_macro
[docs.rs]: https://docs.rs/place_macro/latest/place_macro/
[downloads-badge]: https://img.shields.io/crates/d/place_macro
[github]: https://github.com/BonnyAD9/place_macro
[releases]: https://github.com/BonnyAD9/place_macro/releases
[website]: https://bonnyad9.github.io/
