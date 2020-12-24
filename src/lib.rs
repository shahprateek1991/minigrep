use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &content) {
        println!("{}", line);
    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments found");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_search_zero_result() {
        let query = "test-string";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(0, search(query, content).len());
    }

    #[test]
    fn should_search_one_result() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }

    #[test]
    fn should_search_multiple_result() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three duct.";
        assert_eq!(
            vec!["safe, fast, productive.", "Pick three duct."],
            search(query, content)
        );
    }
}
