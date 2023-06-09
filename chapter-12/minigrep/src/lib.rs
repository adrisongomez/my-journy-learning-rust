use std::env;
use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let results: Vec<&str> = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search_case_sensitive(&config.query, &contents)
    };
    for line in results {
        println!("{line}")
    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // to remove the first args that is the binary name.
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a search string"),
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a search string"),
        };
        let ignore_case = args.collect::<Vec<String>>().contains(&"-i".to_string())
            || env::var("MINIGREP_IGNORE_CASE").is_ok();
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .into_iter()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .into_iter()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:   
safe, fast, productive.
Duct tape.
Pick three.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search_case_sensitive(query, contents)
        );
    }

    #[test]
    fn insesitve_result() {
        let query = "FiNe";
        let contents = "\
Rust Fine:
Limbo fine
FINE
Tito
Time
finE
Give you up, fiNe
        ";

        assert_eq!(
            vec![
                "Rust Fine:",
                "Limbo fine",
                "FINE",
                "finE",
                "Give you up, fiNe"
            ],
            search_case_insensitive(&query, &contents),
        )
    }
}
