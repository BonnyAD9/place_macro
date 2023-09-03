pub fn test() {
    use place_macro::place;

    macro_rules! my_cooler_macro {
        ($t:ident) => {
            place! {
                macro_rules! __ident__(my_ $t _macro) {
                    (__s__ name:ident -> __s__ t:ty, __s__ body:expr) => {
                        place! {
                            #[doc =
                                __id__(__str__)(
                                    $t " function called " __s__ name ". Returns `"
                                    __id__(__strfy__)(__s__ t) "`."
                                )
                            ]
                            fn __id__(__ident__)($t _ __s__ name)() -> __s__ t {
                                __s__ body
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
