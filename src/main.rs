use std::{env, fs};
fn main() {
    match Config::new(&env::args().collect::<Vec<String>>()) {
        Ok(config) => {
            run(config)
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}

fn run(Config { query, file }: Config) {
    let contents = fs::read_to_string(file);
    match contents {
        Ok(contents) => {
            let lines = contents.lines().filter(|line| line.contains(&query)).collect::<Vec<&str>>();
            match lines.len() {
                len if len > 0 => {
                    println!("Found {} lines", len);
                    for line in lines {
                        println!("{}", line);
                    }
                }
                _ => println!("No lines found"),
            }
        }
        Err(e) => println!("Error reading file: {}", e),
    }
}

struct Config {
    query: String,
    file: String,
}
impl Config {
    fn new(args: &[String]) -> Result<Config, String> {
        match args.len() {
            len if len > 2 => {
                Result::Ok(Config { query: args[1].to_owned(), file: args[2].to_owned() })
            }
            _ => Result::Err("Usage rust-grep <query> <file>".to_string()),
        }
    }
}
