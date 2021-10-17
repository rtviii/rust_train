pub mod input;
use input::read_line_cif;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::env;


fn main() {

    let x:String =read_line_cif();
    println!("{}",x);

    let args: Vec<String> = env::args().collect();

    let fpath = Path::new(&args[1]);
    let display = fpath.display();


    let mut file = match File::open(&fpath){
        Err(why) => panic!("couldn't open this {} : {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s){
        Err(why) => panic!("couldnt read {} :  {}", s, why),
        Ok (_)   => print!("{} contains: \n {}", display, s)
    }

    for ( i,arg ) in args.iter().enumerate(){
        println!("argument number {} : {}", i,arg)
    }
}
