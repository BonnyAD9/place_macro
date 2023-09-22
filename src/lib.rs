use std::borrow::Cow;

use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

///!Macros you wish you had while you were writing your non-proc macro.
///!
///!This library privides some macros that make writing regural non-proc
///!macros much simpler, readable and with less dirty tricks.
///!
///!The main macro of this library is `place`. It is able to expand the macros in
///!this library in reverse expansion order.
///!
///!## Macros
///!+ `place`: expands the following macros in reverse order, see below
///!- `ignore`: expands to nothing
///!- `identity`: expands to what is given, it bypasses the reverse order in the
///!  `place` macro
///!- `dollar`: expands to dollar sign `$`
///!- `string`: concats the contents into single string, see the doc
///!- `identifier`: concats the contents into sintle identifier in the same way as
///!  string
///!- `head`: expands to the first token
///!- `tail`: expands to all but the first token
///!- `start`: expands to all but the last token
///!- `last`: expands to the last token
///!- `reverse`: expands to the tokens in reverse order
///!- `stringify`: expands to string of the input
///!
///!### The macro `place`
///!Expands the other macros inside in reverse order. The macros inside that will
///!be expanded are used with a different sintax: instead of calling a macro as
///!`string!("hello" "there")` you call it as `__string__("hello" "there")`. One
///!exception is the macro `dollar` that is called without the parenthesis:
///!`__dollar__` instead of `dollar!()`.
///!
///!For some of the macros there are also shorter names:
///!- `__identity__` - `__id__`
///!- `__string__` - `__str__`
///!- `__dollar__` - `__s__`
///!- `__identifier__` - `__ident__`
///!- `__stringify__` - `__strfy__`
///!
///!#### Example
///!The following passes:
///!```rust
///!use place_macro::place;
///!
///!let res = place!(__string__(1 __string__(2 __identity__(3 __string__(4)))));
///!assert_eq!(res, "123__string__4");
///!```
///!
///!Why is this useful?
///!
///!- You can generate identifiers in your macros:
///!```rust
///!use place_macro::place;
///!
///!macro_rules! my_cool_macro {
///!    ($name:ident -> $t:ty, $body:expr) => {
///!        place! {
///!            fn __identifier__(cool_ $name)() -> $t {
///!                $body
///!            }
///!        }
///!    };
///!}
///!
///!my_cool_macro! { foo -> &'static str, "cool!" }
///!/// Expands to:
///!fn cool_foo() -> &'static str {
///!    "cool!"
///!}
///!```
///!- You can generate strings as macro parameters in your macros:
///!```rust
///!use place_macro::place;
///!
///!macro_rules! my_cool_macro {
///!    ($name:ident -> $t:ty, $body:expr) => {
///!        place! {
///!            #[doc =
///!                __string__(
///!                    "cool function called " $name ". Returns `"
///!                    __stringify__($t) "`."
///!                )
///!            ]
///!            fn __identifier__(cool_ $name)() -> $t {
///!                $body
///!            }
///!        }
///!    };
///!}
///!
///!my_cool_macro! { foo -> &'static str, "cool!" }
///!/// Expands to:
///!#[doc = "cool function called foo. Returns `&'static str`."]
///!fn cool_foo() -> &'static str {
///!    "cool!"
///!}
///!```
///!- Or you can even generate macros in your macros
///!```rust
///!use place_macro::place;
///!
///!macro_rules! my_cooler_macro {
///!    ($t:ident) => {
///!        place! {
///!            macro_rules! __identifier__(my_ $t _macro) {
///!                (__dollar__ name:ident -> __dollar__ t:ty, __dollar__ body:expr) => {
///!                    place! {
///!                        #[doc =
///!                            __identity__(__string__)(
///!                                $t " function called " __dollar__ name ". Returns `"
///!                                __identity__(__stringify__)(__dollar__ t) "`."
///!                            )
///!                        ]
///!                        fn __identity__(__identifier__)($t __dollar__ name)() -> __dollar__ t {
///!                            __dollar__ body
///!                        }
///!                    }
///!                };
///!            }
///!        }
///!    };
///!}
///!
///!my_cooler_macro! { cool };
///!my_cool_macro! { foo -> &'static str, "cool!" }
///!/// now you have the same function as in the previous example
///!```
///!The last example was a little less readable, but you can see that you can do
///!a lot with this macro.

/// Ignores all the input, as if there was nothing
///
/// # Examples
/// ```
/// use place_macro::ignore;
///
/// let mut i = 5;
/// ignore!(i = 10);
/// assert_eq!(i, 5);
/// ```
#[proc_macro]
pub fn ignore(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}

/// Returns exactly the given input
///
/// # Examples
/// ```
/// use place_macro::identity;
///
/// let mut i = 5;
/// identity!(i = 10);
/// assert_eq!(i, 10);
/// ```
#[proc_macro]
pub fn identity(input: TokenStream) -> TokenStream {
    input
}

/// Expands to a single dollar sign, this has no use when it is used alone,
/// but it can be used in the `place` macro
#[proc_macro]
pub fn dollar(input: TokenStream) -> TokenStream {
    let r = if input.is_empty() {
        TokenTree::Punct(Punct::new('$', Spacing::Alone))
    } else {
        panic!("No input was expected")
    };

    let mut res = TokenStream::new();
    res.extend([r].into_iter());
    res
}

/// Converts the input to string literal, literals are interpreted as their
/// values, punctuation and brackets are ignored and the rest is stringified.
///
/// # Examples
/// ```
/// use place_macro::string;
///
/// let s = string!("hello" + , ", " {(agent)} ' ' 0x2F);
/// assert_eq!(s, "hello, agent 47");
/// ```
#[proc_macro]
pub fn string(input: TokenStream) -> TokenStream {
    let res = token_concat(input);

    let mut r = TokenStream::new();
    r.extend([TokenTree::Literal(Literal::string(res.as_str()))].into_iter());
    r
}

/// Expans to the first token if present
///
/// # Examples
/// ```
/// use place_macro::head;
///
/// let n = head!(5 + 3 + 2);
/// assert_eq!(n, 5);
///
/// let n = head!((5 + 3) + 2);
/// assert_eq!(n, (5 + 3));
///
/// // expands to nothing
/// head!();
/// ```
#[proc_macro]
pub fn head(input: TokenStream) -> TokenStream {
    let mut res = TokenStream::new();
    if let Some(t) = input.into_iter().next() {
        res.extend([t].into_iter());
    }
    res
}

/// Expands to all but the first token
///
/// # Examples
/// ```
/// use place_macro::tail;
///
/// let n = tail!(-5);
/// assert_eq!(n, 5);
///
/// let n = tail!((5 + 3) - 5);
/// assert_eq!(n, -5);
///
/// // expands to nothing
/// tail!((-5));
/// ```
#[proc_macro]
pub fn tail(input: TokenStream) -> TokenStream {
    let mut res = TokenStream::new();
    res.extend(input.into_iter().skip(1));
    res
}

/// Expands to all but the last token
///
/// # Examples
/// ```
/// use place_macro::start;
///
/// let n = start!(5 + 3 +);
/// assert_eq!(n, 5 + 3);
///
/// // expands to nothing
/// start!();
/// ```
#[proc_macro]
pub fn start(input: TokenStream) -> TokenStream {
    let mut res = TokenStream::new();

    if input.is_empty() {
        return res;
    }

    let mut input = input.into_iter();
    let last = input.next().unwrap();

    res.extend(input.scan(last, |last, mut i| {
        core::mem::swap(last, &mut i);
        Some(i)
    }));

    res
}

/// Expands to the last token
///
/// # Examples
/// ```
/// use place_macro::last;
///
/// let n = last!(5 + 3 + 2);
/// assert_eq!(n, 2);
///
/// // expands to nothing
/// last!();
/// ```
#[proc_macro]
pub fn last(input: TokenStream) -> TokenStream {
    let mut res = TokenStream::new();

    if input.is_empty() {
        return res;
    }

    let mut i = input.into_iter();
    let f = i.next().unwrap();
    let last = i.fold(f, |_, c| c);
    res.extend([last].into_iter());

    res
}

/// Reverses the passed tokens
///
/// # Examples
/// ```
/// use place_macro::reverse;
///
/// let n = reverse!(5 - 3);
/// assert_eq!(n, -2);
///
/// let n = reverse!((2 + 3) - 2);
/// assert_eq!(n, -3);
/// ```
#[proc_macro]
pub fn reverse(input: TokenStream) -> TokenStream {
    let mut res = TokenStream::new();
    let tok: Vec<_> = input.into_iter().collect();
    res.extend(tok.into_iter().rev());
    res
}

/// Creates a identifier in the same way as the string macro creates string
/// literals.
///
/// # Example
/// ```
/// use place_macro::identifier;
///
/// let my = 5;
/// let var = 10;
/// let myvar = 1;
/// let n = identifier!(my + var);
/// assert_eq!(n, myvar);
/// ```
#[proc_macro]
pub fn identifier(input: TokenStream) -> TokenStream {
    let res = token_concat(input);

    let mut r = TokenStream::new();
    r.extend([TokenTree::Ident(Ident::new(&res, Span::call_site()))].into_iter());
    r
}

/// Should be same to the rust macro stringify
///
/// # Example
/// ```
/// use place_macro;
///
/// let a = place_macro::stringify!("hello" + , ", " {(agent)} ' ' 0x2F);
/// let b = stringify!("hello" + , ", " {(agent)} ' ' 0x2F);
/// assert_eq!(a, b);
/// ```
#[proc_macro]
pub fn stringify(input: TokenStream) -> TokenStream {
    let mut res = TokenStream::new();
    res.extend([TokenTree::Literal(Literal::string(&input.to_string()))]);
    res
}

/// Replaces newlines and follwing whitespace in string literal with another
/// string.
///
/// # Example
/// ```
/// use place_macro::replace_newline;
///
/// let v = replace_newline!("hello
///     every body
/// ", ", ");
/// assert_eq!(v, "hello, every body, ");
/// ```
#[proc_macro]
pub fn replace_newline(input: TokenStream) -> TokenStream {
    let mut i = input.into_iter();
    let s = match i.next() {
        Some(s) => s,
        None => panic!("Expected two arguments, got 0"),
    };
    i.next();
    let r = match i.next() {
        Some(s) => s,
        None => panic!("Expected two arguments, got 1"),
    };

    let s = match get_str_lit(s) {
        Some(s) => s,
        None => panic!("First argument must be string literal"),
    };
    let r = match get_str_lit(r) {
        Some(r) => r,
        None => panic!("Second argument must be string literal"),
    };

    let mut res = String::new();
    let mut i = s.chars();
    while let Some(c) = i.next() {
        if c != '\n' {
            res.push(c);
            continue;
        }
        res += &r;
        while let Some(c) = i.next() {
            if !c.is_whitespace() {
                res.push(c);
                break;
            }
        }
    }

    let mut r = TokenStream::new();
    r.extend([TokenTree::Literal(Literal::string(&res))]);
    r
}

/// Replaces in string literal
///
/// # Examples
/// ```
/// use place_macro::str_replace;
///
/// let s = str_replace!("hello runtime replace", "runtime", "compile-time");
/// assert_eq!(s, "hello compile-time replace");
/// ```
#[proc_macro]
pub fn str_replace(input: TokenStream) -> TokenStream {
    let mut i = input.into_iter();
    let s = match i.next() {
        Some(s) => s,
        None => panic!("Expected 3 arguments, got 0"),
    };
    i.next();
    let f = match i.next() {
        Some(f) => f,
        None => panic!("Expected 3 arguments, got 1"),
    };
    i.next();
    let t = match i.next() {
        Some(t) => t,
        None => panic!("Expected 3 arguments, got 2"),
    };

    let s = match get_str_lit(s) {
        Some(s) => s,
        None => panic!("First argument must be string literal"),
    };
    let f = match get_str_lit(f) {
        Some(f) => f,
        None => panic!("Second argument must be string literal"),
    };
    let t = match get_str_lit(t) {
        Some(t) => t,
        None => panic!("Second argument must be string literal"),
    };

    let res = s.replace(&f.to_string(), &t);

    let mut r = TokenStream::new();
    r.extend([TokenTree::Literal(Literal::string(&res))]);
    r
}

fn token_concat(input: TokenStream) -> String {
    let mut input = vec![input.into_iter()];
    let mut res = String::new();

    while let Some(i) = input.last_mut() {
        if let Some(t) = i.next() {
            match t {
                TokenTree::Group(g) => input.push(g.stream().into_iter()),
                TokenTree::Ident(i) => res += &i.to_string(),
                TokenTree::Punct(_) => {}
                TokenTree::Literal(l) => match litrs::Literal::from(l) {
                    litrs::Literal::Bool(v) => res += &v.value().to_string(),
                    litrs::Literal::Integer(v) => {
                        if let Some(v) = v.value::<u128>() {
                            res += &v.to_string()
                        } else {
                            panic!("Integer is too large");
                        }
                    }
                    litrs::Literal::Float(v) => {
                        let n: f64 = v.number_part().parse().unwrap();
                        res += &n.to_string()
                    }
                    litrs::Literal::Char(v) => res.push(v.value()),
                    litrs::Literal::String(v) => res += &v.into_value(),
                    litrs::Literal::Byte(v) => res += &v.to_string(),
                    litrs::Literal::ByteString(v) => res += &v.to_string(),
                },
            }
        } else {
            input.pop();
        }
    }

    res
}

fn get_str_lit<'a>(tt: TokenTree) -> Option<Cow<'a, str>> {
    match tt {
        TokenTree::Group(g) => {
            let mut i = g.stream().into_iter();
            let t1 = i.next();
            let t2 = i.next();
            if t1.is_none() || t2.is_some() {
                None
            } else {
                get_str_lit(t1.unwrap())
            }
        },
        TokenTree::Literal(l) => litrs::StringLit::try_from(l).map(|l| l.into_value()).ok(),
        _ => None,
    }
}

/// Evaluates the macros in this crate in reverse order
///
/// to minimize conflicts, the macros are refered to as `__macro__` where
/// macro is the name of the macro. Special case is the macro `dollar` that
/// doesn't have any arguments.
/// # Examples
/// ```
/// use place_macro::place;
///
/// place! {
///     pub fn __identifier__(my "_function")() -> bool {
///         true
///     }
/// }
/// assert!(my_function());
///
/// place! {
///     macro_rules! mac {
///         (__dollar__ var: literal) => {
///             __dollar__ var
///         }
///     }
/// }
/// assert_eq!("hi", mac!("hi"));
///
/// let res = place!(__string__(1 __string__(2 __identity__(3 __string__(4)))));
/// assert_eq!(res, "123__string__4");
/// ```
#[proc_macro]
pub fn place(input: TokenStream) -> TokenStream {
    let mut input: Vec<(_, Option<Macro>, _)> = vec![(input.into_iter(), None, Delimiter::None)];
    let mut res = vec![TokenStream::new()];

    while let Some((i, m, d)) = input.last_mut() {
        let t = match (i.next(), m) {
            (Some(t), _) => t,
            (_, m) => {
                if let Some(m) = m {
                    let t = res.pop().expect("1");
                    res.last_mut().expect("2").extend(m.invoke(t));
                } else {
                    if res.len() != 1 {
                        let t = res.pop().expect("3");
                        res.last_mut()
                            .expect("4")
                            .extend([TokenTree::Group(Group::new(*d, t))])
                    }
                }
                input.pop();
                continue;
            }
        };

        let id = match t {
            TokenTree::Group(g) => {
                input.push((g.stream().into_iter(), None, g.delimiter()));
                res.push(TokenStream::new());
                continue;
            }
            TokenTree::Ident(id) => id,
            t => {
                res.last_mut().expect("5").extend([t]);
                continue;
            }
        };

        let name = id.to_string();
        let m = match Macro::from_name(&name) {
            None => {
                res.last_mut().expect("6").extend([TokenTree::Ident(id)]);
                continue;
            }
            Some(Macro::Dollar) => {
                res.last_mut()
                    .expect("7")
                    .extend(dollar(TokenStream::new()));
                continue;
            }
            Some(m) => m,
        };

        let g = match i.next() {
            Some(TokenTree::Group(g)) => g,
            Some(TokenTree::Ident(id)) => {
                if m != Macro::Ignore {
                    panic!("Expected a group after {name}");
                }

                let iname = id.to_string();
                if let Some(m) =  Macro::from_name(&iname) {
                    if m == Macro::Dollar {
                        continue;
                    }
                } else {
                    panic!("Expected a group or builtin macro after {name}");
                }

                if let Some(TokenTree::Group(g)) = i.next() {
                    let l = input.pop().unwrap();
                    let mut s = g.stream();
                    s.extend(l.0);
                    input.push((s.into_iter(), l.1, l.2));
                    continue;
                }

                panic!("Expected a group after {iname}");
            }
            _ => panic!("Expected a group after {name}"),
        };

        if m == Macro::Identity {
            res.last_mut().expect("7").extend(g.stream())
        } else {
            input.push((g.stream().into_iter(), Some(m), g.delimiter()));
            res.push(TokenStream::new());
        }
    }

    res.pop().expect("8")
}

#[derive(PartialEq, Clone, Copy)]
enum Macro {
    Ignore,
    Identity,
    Dollar,
    String,
    Head,
    Tail,
    Start,
    Last,
    Reverse,
    Identifier,
    Stringify,
    ReplaceNewline,
    StrReplace,
}

impl Macro {
    fn from_name(s: &str) -> Option<Macro> {
        match s {
            "__ignore__" => Some(Self::Ignore),
            "__identity__" | "__id__" => Some(Self::Identity),
            "__dollar__" | "__s__" => Some(Self::Dollar),
            "__string__" | "__str__" => Some(Self::String),
            "__head__" => Some(Self::Head),
            "__tail__" => Some(Self::Tail),
            "__start__" => Some(Self::Start),
            "__last__" => Some(Self::Last),
            "__reverse__" => Some(Self::Reverse),
            "__identifier__" | "__ident__" => Some(Self::Identifier),
            "__stringify__" | "__strfy__" => Some(Self::Stringify),
            "__replace_newline__" | "__repnl__" => Some(Self::ReplaceNewline),
            "__str_replace__" | "__repstr__" => Some(Self::StrReplace),
            _ => None,
        }
    }

    fn invoke(&self, input: TokenStream) -> TokenStream {
        match self {
            Macro::Ignore => ignore(input),
            Macro::Identity => identity(input),
            Macro::Dollar => dollar(input),
            Macro::String => string(input),
            Macro::Head => head(input),
            Macro::Tail => tail(input),
            Macro::Start => start(input),
            Macro::Last => last(input),
            Macro::Reverse => reverse(input),
            Macro::Identifier => identifier(input),
            Macro::Stringify => stringify(input),
            Macro::ReplaceNewline => replace_newline(input),
            Macro::StrReplace => str_replace(input),
        }
    }
}
