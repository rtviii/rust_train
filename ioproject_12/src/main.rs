#[allow(dead_code)]
use std::fs;
use std::env;



fn main() {
    let args: Vec<String> = env::args().collect();



    let [ first, second ] = parse_ints(&args);
    println!("Int one is {}", first);
    println!("Int two is {}", second);
    // let query = &args[1];
    // let filename = &args[2];

    // println!("In file {}", filename);

    // let file_contents = fs::read_to_string(filename)
    // .expect("Something went wrong reading file.");

    // println!("File contents:\n{}", file_contents);
}




fn parse_config(arguments: &[String]) -> (&str, &str){
    let query    = &arguments[1];
    let filename = &arguments[2];
    (query, filename)
}


fn parse_ints(arguments: &Vec<String>) -> [i32;2]
{

    for n in 1..2 {

    }
    let first  = &arguments[1].parse::<i32>();
    let second = &arguments[2].parse::<i32>();
    let first =  match first{
        Ok(i32) =>i32,
        Err(error) => panic!("Had trouble parsing {:?}", error)
    };
    let second =  match second{
        Ok(i32) =>i32,
        Err(error) => panic!("Had trouble parsing {:?}", error)
    };
    [*first, *second]
}

