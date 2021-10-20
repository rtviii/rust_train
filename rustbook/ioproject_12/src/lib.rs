use std::fs;
use std::error::Error;
use std::env;

pub fn run (config:Config)->Result<(),Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename)?;
	if config.case_sensetive {
		println!("case sens");
		search(&config.query, &contents)
	}else{
		println!("case insens");
		search_case_insensetiv(&config.query, &contents)
	};

	for line in search(&config.query, &contents){
		println!("Found line :{}",line);
	}
    Ok(())
}

pub struct Config{
    pub query   : String,
    pub filename: String,
	pub case_sensetive: bool

}

impl Config{
   pub fn new(args: &[String]) -> Result<Config,&str> {
   if args.len() < 3 {
            return Err("not enough arguments");
        }

    let query          = args[1].clone();
    let filename       = args[2].clone();
    let case_sensetive = env::var("CASE_INSENSITIVE").is_err();
	println!("Case sensetive set ot {}",case_sensetive);
    Ok(Config{query,filename, case_sensetive})
    }
}

pub fn search<'a>(query:&str, contents:&'a str) ->Vec<&'a str>{

	let mut results:Vec<&str> = Vec::new();

	for line in contents.lines(){
		if line.contains(query){
			results.push(line);
		}
	}
	results
}

pub fn search_case_insensetiv<'a>(query:&str, contents:&'a str) ->Vec<&'a str>{
	let query                 = query.to_lowercase();
	let mut results:Vec<&str> = Vec::new();
	for line in contents.lines(){
		if line.to_lowercase().contains(&query){
			results.push(line);
		}
	}
	results
}

#[cfg(test)]
mod tests{
	use super::*;

	#[test]
	fn case_sensitive(){
		let query = "duct";
		let contents = "\
		safe, fast, productive.
				Pick three.";
		assert_eq!(vec!["safe, fast, productive."], search(query,contents))
		
	}
	#[test]
	fn case_insensitive(){
		let query = "DUCK";
		let contents = "\
		DUCK sh
		safe, 4uck fast, productive.
DuCK ss
						Pick three.";
		assert_eq!(vec!["DUCK sh", "DuCK ss"], search_case_insensetiv(query,contents))
	}
}

