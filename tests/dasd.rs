use place_macro::place;

fn asd() {
    place! {
        let __to_case__(MyVariable) = "";
    }
    println!("{}", my_variable);
}
