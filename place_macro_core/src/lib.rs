use std::borrow::Cow;

use convert_case::{Case, Casing};
use proc_macro2::{
    Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream,
    TokenTree,
};

macro_rules! check_comma {
    ($stream:expr, $pos:expr) => {
        if let Some(tree) = $stream.next() {
            if !is_comma(&tree) {
                return error_at(tree.span(), "Expected comma.");
            }
        } else {
            return error_at($pos, "Expected more arguments");
        }
    };
}

fn spanned(mut tree: TokenTree, span: Span) -> TokenTree {
    tree.set_span(span);
    tree
}

fn error_at<S>(span: Span, msg: S) -> TokenStream
where
    S: AsRef<str>,
{
    [
        TokenTree::Ident(Ident::new("compile_error", span)),
        TokenTree::Punct(Punct::new('!', Spacing::Alone)),
        spanned(
            TokenTree::Group(Group::new(
                Delimiter::Parenthesis,
                [TokenTree::Literal(Literal::string(msg.as_ref()))]
                    .into_iter()
                    .collect(),
            )),
            span,
        ),
    ]
    .into_iter()
    .collect()
}

fn is_comma(tree: &TokenTree) -> bool {
    matches!(tree, TokenTree::Punct(p) if p.as_char() == ',')
}

pub fn ignore(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}

pub fn identity(input: TokenStream) -> TokenStream {
    input
}

pub fn dollar(input: TokenStream) -> TokenStream {
    let r = if let Some(t) = input.into_iter().next() {
        return error_at(t.span(), "Macro `dollar` has no arguments.");
    } else {
        TokenTree::Punct(Punct::new('$', Spacing::Alone))
    };

    let mut res = TokenStream::new();
    res.extend([r]);
    res
}

pub fn string(input: TokenStream) -> TokenStream {
    let res = token_concat(input);

    let mut r = TokenStream::new();
    r.extend([TokenTree::Literal(Literal::string(res.as_str()))]);
    r
}

pub fn head(input: TokenStream) -> TokenStream {
    let mut res = TokenStream::new();
    if let Some(t) = input.into_iter().next() {
        res.extend([t]);
    }
    res
}

pub fn tail(input: TokenStream) -> TokenStream {
    let mut res = TokenStream::new();
    res.extend(input.into_iter().skip(1));
    res
}

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

pub fn last(input: TokenStream) -> TokenStream {
    let mut res = TokenStream::new();

    if input.is_empty() {
        return res;
    }

    let mut i = input.into_iter();
    let f = i.next().unwrap();
    let last = i.fold(f, |_, c| c);
    res.extend([last]);

    res
}

pub fn reverse(input: TokenStream) -> TokenStream {
    let mut res = TokenStream::new();
    let tok: Vec<_> = input.into_iter().collect();
    res.extend(tok.into_iter().rev());
    res
}

pub fn identifier(input: TokenStream) -> TokenStream {
    let res = token_concat(input);

    let mut r = TokenStream::new();
    r.extend([TokenTree::Ident(Ident::new(&res, Span::call_site()))]);
    r
}

pub fn stringify(input: TokenStream) -> TokenStream {
    let mut res = TokenStream::new();
    res.extend([TokenTree::Literal(Literal::string(&input.to_string()))]);
    res
}

pub fn replace_newline(input: TokenStream, pos: Span) -> TokenStream {
    let mut i = input.into_iter();
    let s = match i.next() {
        Some(s) => s,
        None => return error_at(pos, "Expected two arguments, got 0"),
    };
    check_comma!(i, pos);
    let r = match i.next() {
        Some(s) => s,
        None => return error_at(pos, "Expected two arguments, got 1"),
    };
    if let Some(n) = i.next() {
        if is_comma(&n) {
            if let Some(t) = i.next() {
                return error_at(t.span(), "Macro takes only 2 arguments");
            }
        } else {
            return error_at(n.span(), "Unexpected token in macro invocation");
        }
    }

    let s = match get_str_lit(s.clone()) {
        Some(s) => s,
        None => return error_at(s.span(), "Expected string literal"),
    };
    let r = match get_str_lit(r.clone()) {
        Some(r) => r,
        None => return error_at(r.span(), "Expected string literal"),
    };

    let mut res = String::new();
    let mut i = s.chars();
    while let Some(c) = i.next() {
        if c != '\n' {
            res.push(c);
            continue;
        }
        res += &r;
        for c in i.by_ref() {
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

pub fn str_replace(input: TokenStream, pos: Span) -> TokenStream {
    let mut i = input.into_iter();
    let s = match i.next() {
        Some(s) => s,
        None => return error_at(pos, "Expected 3 arguments, got 0"),
    };
    check_comma!(i, pos);
    let f = match i.next() {
        Some(f) => f,
        None => return error_at(pos, "Expected 3 arguments, got 1"),
    };
    check_comma!(i, pos);
    let t = match i.next() {
        Some(t) => t,
        None => return error_at(pos, "Expected 3 arguments, got 2"),
    };
    if let Some(n) = i.next() {
        if is_comma(&n) {
            if let Some(t) = i.next() {
                return error_at(t.span(), "Macro takes only 3 arguments");
            }
        } else {
            return error_at(n.span(), "Unexpected token in macro invocation");
        }
    }

    let s = match get_str_lit(s.clone()) {
        Some(s) => s,
        None => return error_at(s.span(), "Expected string literal"),
    };
    let f = match get_str_lit(f.clone()) {
        Some(f) => f,
        None => return error_at(f.span(), "Expected string literal"),
    };
    let t = match get_str_lit(t.clone()) {
        Some(t) => t,
        None => return error_at(t.span(), "Expected string literal"),
    };

    let res = s.replace(&f.to_string(), &t);

    let mut r = TokenStream::new();
    r.extend([TokenTree::Literal(Literal::string(&res))]);
    r
}

pub fn to_case(input: TokenStream, pos: Span) -> TokenStream {
    let mut i = input.into_iter();

    let dst = match i.next() {
        Some(s) => s,
        None => return error_at(pos, "Expected 2 arguments."),
    };
    check_comma!(i, pos);
    let src = match i.next() {
        Some(s) => s,
        None => return error_at(pos, "Expected 2 arguments"),
    };
    if let Some(n) = i.next() {
        if is_comma(&n) {
            if let Some(t) = i.next() {
                return error_at(t.span(), "Macro takes only 2 arguments");
            }
        } else {
            return error_at(n.span(), "Unexpected token in macro invocation");
        }
    }

    let dst = match get_str_lit(dst.clone()) {
        Some(s) => s,
        None => return error_at(dst.span(), "Expected string literal"),
    };
    let src = if let TokenTree::Ident(l) = src {
        l.to_string()
    } else {
        return error_at(src.span(), "Expected identifier");
    };

    let s = get_case(&dst, &src);
    let mut res = TokenStream::new();
    res.extend([TokenTree::Ident(Ident::new(&s, Span::call_site()))]);
    res
}

fn get_case(spec: &str, i: &str) -> String {
    match spec {
        "TOCASE" => i.to_case(Case::UpperFlat),
        "tocase" => i.to_case(Case::Flat),
        "toCase" => i.to_case(Case::Camel),
        "ToCase" => i.to_case(Case::Pascal),
        "to_case" => i.to_case(Case::Snake),
        "TO_CASE" => i.to_case(Case::UpperSnake),
        _ => panic!("Unknown case specifier: '{spec}'"),
    }
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
            match (t1, t2) {
                (Some(t1), None) => get_str_lit(t1),
                _ => None,
            }
        }
        TokenTree::Literal(l) => {
            litrs::StringLit::try_from(l).map(|l| l.into_value()).ok()
        }
        _ => None,
    }
}

pub fn place(input: TokenStream) -> TokenStream {
    let mut input: Vec<(_, Option<Macro>, _)> =
        vec![(input.into_iter(), None, Delimiter::None)];
    let mut res = vec![TokenStream::new()];

    while let Some((i, m, d)) = input.last_mut() {
        let t = match (i.next(), m) {
            (Some(t), _) => t,
            (_, m) => {
                if let Some(m) = m {
                    let t = res.pop().expect("1");
                    res.last_mut().expect("2").extend(m.invoke(t));
                } else if res.len() != 1 {
                    let t = res.pop().expect("3");
                    res.last_mut()
                        .expect("4")
                        .extend([TokenTree::Group(Group::new(*d, t))])
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
        let m = match Macro::from_name(&name, id.span()) {
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
                if !matches!(m, Macro::Ignore) {
                    return error_at(id.span(), "Expected '('");
                }

                let iname = id.to_string();
                if let Some(m) = Macro::from_name(&iname, id.span()) {
                    if matches!(m, Macro::Dollar) {
                        continue;
                    }
                } else {
                    return error_at(
                        id.span(),
                        "Expected '(' or builtin macro",
                    );
                }

                let mut pos = id.span();
                if let Some(g) = i.next() {
                    if let TokenTree::Group(g) = g {
                        let l = input.pop().unwrap();
                        let mut s = g.stream();
                        s.extend(l.0);
                        input.push((s.into_iter(), l.1, l.2));
                        continue;
                    } else {
                        pos = g.span();
                    }
                }

                return error_at(pos, "Expected '('");
            }
            Some(t) => return error_at(t.span(), "Expected '('"),
            None => {
                return error_at(id.span(), "Expected '(' after builtin macro");
            }
        };

        if matches!(m, Macro::Identity) {
            res.last_mut().expect("7").extend(g.stream())
        } else if matches!(m, Macro::ToCase(_)) {
            let mut s = TokenStream::new();
            s.extend([
                TokenTree::Literal(Literal::string(name.trim_matches('_'))),
                TokenTree::Punct(Punct::new(',', Spacing::Alone)),
            ]);
            s.extend(g.stream().into_iter());
            input.push((s.into_iter(), Some(m), g.delimiter()));
            res.push(TokenStream::new());
        } else {
            input.push((g.stream().into_iter(), Some(m), g.delimiter()));
            res.push(TokenStream::new());
        }
    }

    res.pop().expect("8")
}

#[derive(Clone, Copy)]
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
    ReplaceNewline(Span),
    StrReplace(Span),
    ToCase(Span),
}

impl Macro {
    fn from_name(s: &str, pos: Span) -> Option<Macro> {
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
            "__replace_newline__" | "__repnl__" => {
                Some(Self::ReplaceNewline(pos))
            }
            "__str_replace__" | "__repstr__" => Some(Self::StrReplace(pos)),
            s if s.starts_with("__") && s.ends_with("__") => {
                let lc = s.to_lowercase();
                if lc == "__tocase__" || lc == "__to_case__" {
                    Some(Self::ToCase(pos))
                } else {
                    None
                }
            }
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
            Macro::ReplaceNewline(pos) => replace_newline(input, *pos),
            Macro::StrReplace(pos) => str_replace(input, *pos),
            Macro::ToCase(pos) => to_case(input, *pos),
        }
    }
}
