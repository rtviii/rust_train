fn longeststring<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub fn driver(){

    let string1 = String::from("String is logn");

let result;
{ 
    let string2 = String::from("xyz");

    result = longeststring(string1.as_str(), string2.as_str());
 }
    println!("The longest string is {}", result);

}