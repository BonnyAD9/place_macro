use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

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

        let g = if let Some(TokenTree::Group(g)) = i.next() {
            g
        } else {
            panic!("Expected a group after {name}");
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
        }
    }
}
