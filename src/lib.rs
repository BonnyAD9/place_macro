use proc_macro::{TokenStream, TokenTree, Punct, Spacing, token_stream, Literal};


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
/// let s = string!("hello" + , ", " {(agent)} ' ' 0x2F );
/// assert_eq!(s, "hello, agent 47");
/// ```
#[proc_macro]
pub fn string(input: TokenStream) -> TokenStream {
    let mut input = vec![input.into_iter()];
    let mut res = String::new();

    while let Some(i) = input.last_mut() {
        if let Some(t) = i.next() {
            match t {
                TokenTree::Group(g) => input.push(g.stream().into_iter()),
                TokenTree::Ident(i) => res += &i.to_string(),
                TokenTree::Punct(_) => {},
                TokenTree::Literal(l) => {
                    match litrs::Literal::from(l) {
                        litrs::Literal::Bool(v) => res += &v.value().to_string(),
                        litrs::Literal::Integer(v) => {
                            if let Some(v) = v.value::<u128>() {
                                res += &v.to_string()
                            } else {
                                panic!("Integer is too large");
                            }
                        },
                        litrs::Literal::Float(v) => {
                            let n: f64 = v.number_part().parse().unwrap();
                            res += &n.to_string()
                        },
                        litrs::Literal::Char(v) => res.push(v.value()),
                        litrs::Literal::String(v) => res += &v.into_value(),
                        litrs::Literal::Byte(v) => res += &v.to_string(),
                        litrs::Literal::ByteString(v) => res += &v.to_string(),
                    }
                },
            }
        } else {
            input.pop();
        }
    }

    let mut r = TokenStream::new();
    r.extend([
        TokenTree::Literal(Literal::string(res.as_str()))
    ].into_iter());
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
