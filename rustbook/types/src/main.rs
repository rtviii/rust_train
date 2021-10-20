use std::io;


fn main() {

    println!("Enter a number:");

    
    let array = [5;5];
    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("Failed to read line");

    let index:usize = index.trim().parse().expect("Wasn't a number that u entered");


    let elem = array[index];


    println!("Pciked :{}", elem)






}