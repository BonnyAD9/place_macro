pub fn test() {
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
                            fn __identity__(__identifier__)($t _ __dollar__ name)() -> __dollar__ t {
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
}
