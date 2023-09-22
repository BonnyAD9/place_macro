use place_macro::place;

fn asd() {
    let i = place!(__ignore__ __tail__("hello"));
    println!("{}", i);
}
