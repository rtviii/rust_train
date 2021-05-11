#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

mod errors;

// use utils::foores::foo;
// use crate::utils::pubapi;

use std::fs::File;
use rand::Rng;
use std::io;
use std::error::Error;
use std::cmp::Ordering;


pub struct Guess{
    value: i32
}



impl Guess{
    pub fn new (value: i32) -> Guess{
        if value <1 || value> 100{
            panic!("Guess value must be in 1,100 range. Got : {}", value);
        }
        Guess{value}
    }
    pub fn value(&self) ->i32 {
        self.value
    }
}

fn main() -> Result<(), Box<dyn Error>> {

    // errors::recoverable::results();

    // let f = File::open("yello.txt")?;
    let secret_number = rand::thread_rng().gen_range(1,101);

    loop{


        let mut guess = String::new();

        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");


        let guess: i32  =match guess.trim().parse(){
            Ok(num)=>num,
            Err(_) =>continue
        };
        if  guess <1 || guess> 100 {
            println!("Outside 1 and 100");
            continue
        }
        match guess.cmp(&secret_number){
            Ordering::Less=>println!("too small"),
            Ordering::Greater=>println!("too large"),
            Ordering::Equal=>{
                 println!("Win");
                 break;
                }
        }

    }


    Ok(())
}

