use std::error::Error;
use std::fs;
use std::env;
pub struct Config{
    pub query: String,
    pub filename: String,
    pub case_insensitive: bool
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Self,&str> {        
        if args.len() < 3 {
            return Err("Not Enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_insensitive: bool = env::var("CASE_INSENSITIVE").is_err();
        return Ok(Self {query,filename,case_insensitive})
    }
}

pub fn run(config:Config) -> Result<(),Box<dyn Error>> {
    let content:String = fs::read_to_string(config.filename)?;

    println!("Case Insensitive: {}",config.case_insensitive);

    let result: Vec<&str> = if config.case_insensitive {
        search_case_insensitive(&config.query, &content)
    }else {
        search_case_sensitive(&config.query, &content)
    };

    for line in result{
        println!("Line {}",line);
    }

    Ok(())

}
pub fn search_case_sensitive<'a>(query:&str,contents:&'a str) -> Vec<& 'a str>{
    let mut result: Vec<&str> = Vec::new();
    for line in contents.lines(){
        if line.contains(query) {
            result.push(line)
        }
    }
    result
}

pub fn search_case_insensitive<'a>(query:&str,contents:&'a str) -> Vec<& 'a str>{
    let mut result: Vec<&str> = Vec::new();
    let query = query.to_lowercase();
    for line in contents.lines(){
        if line.to_lowercase().contains(&query) {
            result.push(line)
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case_sensitive(){
        let query = "duct";
        let contents = "\
Rust:
safe,fast,productive
Pick Three.
Duct Tape";
        println!("Contents:\n {}",contents);
        assert_eq!(vec!["safe,fast,productive"],search_case_sensitive(query,contents));
    }

    #[test]
    fn case_insensitive(){
        let query = "duct";
        let contents = "\
Rust:
safe,fast,productive
Pick Three.
ProducTive bunny";
        println!("Contents:\n {}",contents);
        assert_eq!(vec!["safe,fast,productive","ProducTive bunny"],search_case_insensitive(query,contents));
    }
}