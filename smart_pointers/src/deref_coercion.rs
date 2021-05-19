
use crate::MyBox;

fn hello(name:&str){
    println!("yello, main! {}", name);
}


pub fn deref_co(){
    let m =  MyBox::new(String::from("Rust"));
    hello("Rust");
    hello(&m);
}