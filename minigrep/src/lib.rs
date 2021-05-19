use std::error::Error;
use std::fs;
use std::env;

pub fn run(conf:Config)->Result<(), Box<dyn Error>>{
    let contents =  fs::read_to_string(conf.filename)?;

    let results = if conf.case_sensetive {
        search(&conf.query,&contents)
    }
    else {
        search_case_insensetive(&conf.query, &contents)
    };

    for line in results{
        println!(" {}",line);
    }
    Ok(())
}

impl Config{
// pub fn new(args:&[String]) -> Result<Config,&str>{
pub fn new(mut args: env::Args) -> Result<Config,&'static str>{
    // if args.len() < 3 {
    //     return Err("not yet")
    // }

    args.next();


    let query    = match args.next(){
        Some(arg) => arg,
        None => return Err("didnt get a query string")
    };
    let filename = match args.next(){
        Some(arg) => arg,
        None => return Err("didnt get a filename string")
    };


    let case_sensetive = env::var("CASE_INSENSETIVE").is_err();

    if case_sensetive {

    println!(" Inited with varialbe set <<<<<{}",case_sensetive);
    }
    else{
            println!(" Inited with varialbe unset <<<<<{}",case_sensetive);
    }

    Ok(Config{query,filename, case_sensetive})
}

}

pub struct Config {
    pub query   : String,
    pub filename: String,
    pub case_sensetive: bool
}
pub fn search<'a>(query:&str, contents:&'a str)-> Vec<&'a str>{


    contents
    .lines()
    .filter(|line| line.contains(query))
    .collect()

    // contents.lines().collect()

    // let mut results = Vec::new();

    // for line in contents.lines(){
    //     if line.contains(query){
    //         results.push(line);

    //     }
    // }
    // results
}
pub fn search_case_insensetive<'a>
(   query:&str,
    contents:&'a str
)-> Vec<&'a str>{
    let query       = query.to_lowercase();
    // let mut results = Vec::new();

    contents.lines()
    .filter(|line| line.to_lowercase().contains(&query))
    .collect()

    // for line in contents.lines(){
    //     if line.to_lowercase().contains(&query){
    //         results.push(line);

    //     }
    // }
    // results
}


#[cfg(test)]
mod tests{

    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
        Rust\nsafe, fast, productive.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }

    #[test]
        fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensetive(query, contents)
        );}
}