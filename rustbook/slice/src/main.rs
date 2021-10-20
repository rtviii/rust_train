
fn main() {

    let mut s = String::from("hey there");
    // first_word(&m);
    // let hello = &s[0..5];
    // let world = &s[6..11];
    let word = first_word(&s);
    // s.clear();

    println!("{}",first_word(&s))
}


fn first_word (s:&str) -> &str {



    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[0..i];
        }
    }


    &s[..]


}