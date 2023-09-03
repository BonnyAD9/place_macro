# Place-macro
Macros you wish you had while you were writing your non-proc macro.

This library privides some macros that make writing regural non-proc
macros much simpler, readable and with less dirty tricks.

The main macro of this library is `place`. It is able to expand the macros in
this library in reverse expansion order.

## Macros
+ `place`: expands the following macros in reverse order, see below
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

### The macro `place`
Expands the other macros inside in reverse order. The macros inside that will
be expanded are used with a different sintax: instead of calling a macro as
`string!("hello" "there")` you call it as `__string__("hello" "there")`. One
exception is the macro `dollar` that is called without the parenthesis:
`__dollar__` instead of `dollar!()`.

For some of the macros there are also shorter names:
- `__identity__` - `__id__`
- `__string__` - `__str__`
- `__dollar__` - `__s__`
- `__identifier__` - `__ident__`
- `__stringify__` - `__strfy__`
- `__replace_newline__` - `__repnl__`
- `__str_replace__` - `__repstr__`

#### Example
The following passes:
```rust
use place_macro::place;

let res = place!(__string__(1 __string__(2 __identity__(3 __string__(4)))));
assert_eq!(res, "123__string__4");
```

Why is this useful?

- You can generate identifiers in your macros:
```rust
use place_macro::place;

macro_rules! my_cool_macro {
    ($name:ident -> $t:ty, $body:expr) => {
        place! {
            fn __identifier__(cool_ $name)() -> $t {
                $body
            }
        }
    };
}

my_cool_macro! { foo -> &'static str, "cool!" }
/// Expands to:
fn cool_foo() -> &'static str {
    "cool!"
}
```
- You can generate strings as macro parameters in your macros:
```rust
use place_macro::place;

macro_rules! my_cool_macro {
    ($name:ident -> $t:ty, $body:expr) => {
        place! {
            #[doc =
                __string__(
                    "cool function called " $name ". Returns `"
                    __stringify__($t) "`."
                )
            ]
            fn __identifier__(cool_ $name)() -> $t {
                $body
            }
        }
    };
}

my_cool_macro! { foo -> &'static str, "cool!" }
/// Expands to:
#[doc = "cool function called foo. Returns `&'static str`."]
fn cool_foo() -> &'static str {
    "cool!"
}
```
- Or you can even generate macros in your macros
```rust
use place_macro::place;

macro_rules! my_cooler_macro {
    ($t:ident) => {
        place! {
            macro_rules! __identifier__(my_ $t _macro) {
                (__dollar__ name:ident -> __dollar__ t:ty, __dollar__ body:expr) => {
                    place! {
                        #[doc =
                            __identity__(__string__)(
                                $t " function called " __dollar__ name ". Returns `"
                                __identity__(__stringify__)(__dollar__ t) "`."
                            )
                        ]
                        fn __identity__(__identifier__)($t __dollar__ name)() -> __dollar__ t {
                            __dollar__ body
                        }
                    }
                };
            }
        }
    };
}

my_cooler_macro! { cool };
my_cool_macro! { foo -> &'static str, "cool!" }
/// now you have the same function as in the previous example
```
The last example was a little less readable, but you can see that you can do
a lot with this macro.

## Links
- **Author:** [BonnyAD9](https://github.com/BonnyAD9)
- **GitHub repository:** [BonnyAD/raplay](https://github.com/BonnyAD9/place_macro)
- **Package:** [crates.io](https://crates.io/crates/place_macro)
- **Documentation:** [docs.rs](https://docs.rs/place_macro/latest/place_macro/)
- **My Website:** [bonnyad9.github.io](https://bonnyad9.github.io/)
