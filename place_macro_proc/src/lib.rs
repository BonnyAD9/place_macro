use proc_macro::TokenStream;

#[proc_macro]
pub fn ignore(input: TokenStream) -> TokenStream {
    place_macro_core::ignore(input.into()).into()
}

#[proc_macro]
pub fn identity(input: TokenStream) -> TokenStream {
    place_macro_core::identity(input.into()).into()
}

#[proc_macro]
pub fn dollar(input: TokenStream) -> TokenStream {
    place_macro_core::dollar(input.into()).into()
}

#[proc_macro]
pub fn string(input: TokenStream) -> TokenStream {
    place_macro_core::string(input.into()).into()
}

#[proc_macro]
pub fn head(input: TokenStream) -> TokenStream {
    place_macro_core::head(input.into()).into()
}

#[proc_macro]
pub fn tail(input: TokenStream) -> TokenStream {
    place_macro_core::tail(input.into()).into()
}

#[proc_macro]
pub fn start(input: TokenStream) -> TokenStream {
    place_macro_core::start(input.into()).into()
}

#[proc_macro]
pub fn last(input: TokenStream) -> TokenStream {
    place_macro_core::last(input.into()).into()
}

#[proc_macro]
pub fn reverse(input: TokenStream) -> TokenStream {
    place_macro_core::reverse(input.into()).into()
}

#[proc_macro]
pub fn identifier(input: TokenStream) -> TokenStream {
    place_macro_core::identifier(input.into()).into()
}

#[proc_macro]
pub fn stringify(input: TokenStream) -> TokenStream {
    place_macro_core::stringify(input.into()).into()
}

#[proc_macro]
pub fn replace_newline(input: TokenStream) -> TokenStream {
    place_macro_core::replace_newline(input.into()).into()
}

#[proc_macro]
pub fn str_replace(input: TokenStream) -> TokenStream {
    place_macro_core::str_replace(input.into()).into()
}

#[proc_macro]
pub fn to_case(input: TokenStream) -> TokenStream {
    place_macro_core::to_case(input.into()).into()
}

#[proc_macro]
pub fn place(input: TokenStream) -> TokenStream {
    place_macro_core::place(input.into()).into()
}
