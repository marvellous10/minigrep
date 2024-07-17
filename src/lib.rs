use std::error::Error;
use std::fs;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &content)
    }else {
        search(&config.query, &content)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}
impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {

        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();
    
        Ok(Config { query, file_path, ignore_case, })
    }
}


pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for lines in content.lines() {
        if lines.contains(query) {
            results.push(lines);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "safe";
        let content = "/
Rust:
safe, fast, productive.
pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }

    #[test]
    fn get_content() {
        let content = "I'm nobody! Who are you?\r
Are you nobody, too?\r
Then there's a pair of us - don't tell!\r
They'd banish us, you know.\r
\r
How dreary to be somebody!\r
How public, like a frog\r
To tell your name the livelong day\r
To an admiring bog!";
        let content_file_path = fs::read_to_string("poems.txt");
        let content_path_result = match content_file_path {
            Ok(content) => content,
            Err(err_value) => err_value.to_string(),
        };

        assert_eq!(content, content_path_result);
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let content = "/
Rust:
safe, fast, productive.
pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUst";
        let content = "/
Rust:
safe, fast, productive.
pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, content));
    }
}