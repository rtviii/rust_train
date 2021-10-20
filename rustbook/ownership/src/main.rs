fn main() {
    let reference_to_nothing = dangle("das");
}

fn dangle(&String)->&String {
    let s = String::from("hello");
    &s
}
