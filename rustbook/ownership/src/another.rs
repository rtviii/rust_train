fn main() {
    let s = String::from("hello");
    change(&s);
}

mod fn change(some_string: &String) {
    some_string.push_str(", world");
}
