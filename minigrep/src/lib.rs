use std::fs;
use std::env;
use std::error::Error;

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut matches = Vec::new();

  for line in contents.lines() {
    if line.contains(&query) {
      matches.push(line)
    }
  }

  matches
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
  let query = query.to_lowercase();
  let mut matches: Vec<&str> = Vec::new();

  for line in content.lines() {
    if line.to_lowercase().contains(&query) {
      matches.push(line);
    }
  }

  matches
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
  let contents: String = fs::read_to_string(&config.filename)?;

  let results = if config.case_sensitive {
    search(&config.query, &contents)
  } else {
    search_case_insensitive(&config.query, &contents)
  };

  for line in results {
    println!("{}", line);
  }

  Ok(())
}

pub struct Config {
  pub filename: String,
  pub query: String,
  pub case_sensitive: bool,
}

impl Config {
  pub fn new(args: & [String]) -> Result<Config, &'static str> {
      if args.len() < 3 {
          return Err("Not enough arguments");
      }
      let query: String = args[1].clone();
      let filename: String = args[2].clone();
      let case_sensitive: bool = env::var("CASE_INSENSITIVE").is_err();
      
      Ok(Config { query, filename, case_sensitive })
  }

}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn case_sensitive() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Oh, Duct tape";

  assert_eq!(
    vec!["safe, fast, productive."], 
    search(query, contents));
  }

  #[test]
  fn case_insensitive() {
    let query = "rUst";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Oh, Duct tape";

    assert_eq!(
      search_case_insensitive(query, contents),
      vec!["Rust:"]
    )
  }


}