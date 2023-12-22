//! Macros you wish you had while you were writing your non-proc macro.
//!
//! This library privides some macros that make writing regural non-proc
//! macros much simpler, readable and with less dirty tricks.
//!
//! The main macro of this library is `place`. It is able to expand the macros in
//! this library in reverse expansion order.
//!
//! ## Macros
//! + `place`: expands the following macros in reverse order, see below
//! - `ignore`: expands to nothing
//! - `identity`: expands to what is given, it bypasses the reverse order in the
//!   `place` macro
//! - `dollar`: expands to dollar sign `$`
//! - `string`: concats the contents into single string, see the doc
//! - `identifier`: concats the contents into sintle identifier in the same way as
//!   string
//! - `head`: expands to the first token
//! - `tail`: expands to all but the first token
//! - `start`: expands to all but the last token
//! - `last`: expands to the last token
//! - `reverse`: expands to the tokens in reverse order
//! - `stringify`: expands to string of the input
//!
//! ### The macro `place`
//! Expands the other macros inside in reverse order. The macros inside that will
//! be expanded are used with a different sintax: instead of calling a macro as
//! `string!("hello" "there")` you call it as `__string__("hello" "there")`. One
//! exception is the macro `dollar` that is called without the parenthesis:
//! `__dollar__` instead of `dollar!()`.
//!
//! For some of the macros there are also shorter names:
//! - `__identity__` - `__id__`
//! - `__string__` - `__str__`
//! - `__dollar__` - `__s__`
//! - `__identifier__` - `__ident__`
//! - `__stringify__` - `__strfy__`
//!
//! #### Example
//! The following passes:
//! ```rust
//! use place_macro::place;
//!
//! let res = place!(__string__(1 __string__(2 __identity__(3 __string__(4)))));
//! assert_eq!(res, "123__string__4");
//! ```
//!
//! Why is this useful?
//!
//! - You can generate identifiers in your macros:
//! ```rust
//! use place_macro::place;
//!
//! macro_rules! my_cool_macro {
//!     ($name:ident -> $t:ty, $body:expr) => {
//!         place! {
//!             fn __identifier__(cool_ $name)() -> $t {
//!                 $body
//!             }
//!         }
//!     };
//! }
//!
//! my_cool_macro! { foo -> &'static str, "cool!" }
//! // Expands to:
//! // ```
//! // fn cool_foo() -> &'static str {
//! //     "cool!"
//! // }
//! // ```
//! ```
//! - You can generate strings as macro parameters in your macros:
//! ```rust
//! use place_macro::place;
//!
//! macro_rules! my_cool_macro {
//!     ($name:ident -> $t:ty, $body:expr) => {
//!         place! {
//!             #[doc =
//!                 __string__(
//!                     "cool function called " $name ". Returns `"
//!                     __stringify__($t) "`."
//!                 )
//!             ]
//!             fn __identifier__(cool_ $name)() -> $t {
//!                 $body
//!             }
//!         }
//!     };
//! }
//!
//! my_cool_macro! { foo -> &'static str, "cool!" }
//! // Expands to:
//! // ```
//! // #[doc = "cool function called foo. Returns `&'static str`."]
//! // fn cool_foo() -> &'static str {
//! //     "cool!"
//! // }
//! // ```
//! ```
//! - Or you can even generate macros in your macros
//! ```rust
//! use place_macro::place;
//!
//! macro_rules! my_cooler_macro {
//!     ($t:ident) => {
//!         place! {
//!             macro_rules! __identifier__(my_ $t _macro) {
//!                 (__dollar__ name:ident -> __dollar__ t:ty, __dollar__ body:expr) => {
//!                     place! {
//!                         #[doc =
//!                             __identity__(__string__)(
//!                                 $t " function called " __dollar__ name ". Returns `"
//!                                 __identity__(__stringify__)(__dollar__ t) "`."
//!                             )
//!                         ]
//!                         fn __identity__(__identifier__)($t __dollar__ name)() -> __dollar__ t {
//!                             __dollar__ body
//!                         }
//!                     }
//!                 };
//!             }
//!         }
//!     };
//! }
//!
//! my_cooler_macro! { cool };
//! my_cool_macro! { foo -> &'static str, "cool!" }
//! // now you have the same function as in the previous example
//! ```
//! The last example was a little less readable, but you can see that you can do
//! a lot with this macro.

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
pub use place_macro_proc::ignore;

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
pub use place_macro_proc::identity;

/// Expands to a single dollar sign, this has no use when it is used alone,
/// but it can be used in the `place` macro
pub use place_macro_proc::dollar;

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
pub use place_macro_proc::string;

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
pub use place_macro_proc::head;

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
pub use place_macro_proc::tail;

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
pub use place_macro_proc::start;

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
pub use place_macro_proc::last;

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
pub use place_macro_proc::reverse;

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
pub use place_macro_proc::identifier;

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
pub use place_macro_proc::stringify;

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
pub use place_macro_proc::replace_newline;

/// Replaces in string literal
///
/// # Examples
/// ```
/// use place_macro::str_replace;
///
/// let s = str_replace!("hello runtime replace", "runtime", "compile-time");
/// assert_eq!(s, "hello compile-time replace");
/// ```
pub use place_macro_proc::str_replace;

/// Converts the given identifier to the given case. Second argument is the
/// identifier and the first is string literal representing the target case.
/// The target case can be one of:
/// - `"TOCASE"`
/// - `"tocase"`
/// - `"toCase"`
/// - `"ToCase"`
/// - `"to_case"`
/// - `"TO_CASE"`
///
/// # Examples
/// ```
/// use place_macro::to_case;
///
/// let my_var = 5;
/// let MyVar = 10;
/// let n = to_case!(ToCase my_var);
/// assert_eq!(n, MyVar);
/// ```
pub use place_macro_proc::to_case;

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
pub use place_macro_proc::place;
