use std::{error::Error, fs};

pub struct Config2 {
    pub query: String,
    pub file_path: String,
}

impl Config2 {
    pub fn build(args: &[String]) -> Result<Config2, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config2 { query, file_path })
    }
}

pub fn run(config: Config2) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{}", contents);

    Ok(())
}
