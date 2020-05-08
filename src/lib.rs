use std::env;
use std::fs;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}


impl Config{
    
    pub fn new(args:&[String]) -> Result<Config, &'static str>{
        if args.len() < 3{
            return Err("not enough arguments!");
        }

        let mut case_sensitive = true;
        for arg in args{
            if arg == "-i" {
                case_sensitive = false;
            }
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = match env::var("CASE_INSENSITIVE"){
            Ok(n) => {eprintln!("Got {} for case_insensitive", n); false},
            Err(_) => {case_sensitive}
        };
        Ok(Config{query, filename, case_sensitive})
    }
}

pub fn search_case_insensitive<'a>(query: &str, text: &'a str) -> Vec<&'a str>{
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in text.lines(){
        if line.to_lowercase().contains(&query){
            results.push(line);
        }
    }
    results
}
pub fn search<'a>(query: &str, text: &'a str) -> Vec<&'a str>{
    let mut result : Vec<&str> = Vec::new();
    for line in text.lines(){
        if line.contains(query){
            result.push(line);
        }
    }
    result
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename)?;
    let results = if config.case_sensitive{
        search(&config.query, &contents)
    }else{
        search_case_insensitive(&config.query, &contents)
    };
    for line in results{
        println!("{}", line);
    }
    // println!("with text:\n{}", contents);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}

