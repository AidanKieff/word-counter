use std::{fs::{File, OpenOptions}, io::{Error, BufRead, BufReader}, collections::HashMap};

use regex::Regex;

#[derive(Debug)]
pub struct Filename {
    pub file_path: String,
}

impl Filename {
    pub fn build(args: &[String]) -> Result<Filename, &'static str> {
        if args.len() > 2 {
            return Err("too many arguments passed in");
        } else if args.len() == 1 {
            return Err("a filename must be passed in as an argument");
        }

        let file_path = args[1].clone();

        Ok(Filename {file_path})
    }

}

pub fn run (file_name: Filename) -> Result<(), Error> {
    let file = open_file(file_name.file_path)?;
    let mut map = HashMap::new();

    word_counter(&file, &mut map);
    dbg!(&mut map);
    Ok(())
}


pub fn open_file(filename: String) -> Result<File, Error> {
    let file = OpenOptions::new()
    .read(true)
    .open(filename)?;
Ok(file)
}

pub fn word_counter(file: &File, map: &mut HashMap<String, i32>) {
    let reader = BufReader::new(file);
    for line in reader.lines() {
        if let Ok(line) = line {
            for word in line.split_whitespace().map(|s| stripper(s)){
                *map.entry(word.to_lowercase()).or_insert(0) +=1;
            };
        }
    }
}

pub fn stripper(s : &str) -> String {
    let re = Regex::new(r"[!.,?\n]").unwrap();
    let result = re.replace_all(s, "");
    result.to_string()
}

#[cfg(test)]
mod tests {


    use super::*;

    #[test]
    fn example_test() {
        assert_eq!(stripper("This!!,.\n"), "This");
    }

    #[test]
    #[should_panic]
    fn test_failed_to_open_file() {
        open_file(String::from("notarealfile.txt")).unwrap();
    }

    #[test]
    fn opens_file() {
        open_file(String::from("example.txt")).unwrap();
    }

    #[test]
    fn successful_strip() {
        let file = open_file(String::from("test1.txt")).unwrap();
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line) = line {
                let v: Vec<String> = line.split_whitespace().map(|s| stripper(s)).collect();
                dbg!(&v);
            }
        }
    }

    #[test]
    fn hash_it() {
        let file = open_file(String::from("test2.txt")).unwrap();
        let reader = BufReader::new(file);
        let mut map: HashMap<String, i32> = HashMap::new();
        for line in reader.lines() {
            if let Ok(line) = line {
                for word in line.split_whitespace().map(|s| stripper(s)){
                    *map.entry(word.to_lowercase()).or_insert(0) +=1;
                };
                dbg!(&map);
            }
        }
    }

    #[test]
    fn build_fails_no_path_arg() {
        let args: Vec<String> = vec![String::from("test")];
        assert_eq!(Filename::build(&args).unwrap_err(), "a filename must be passed in as an argument")
    }

    #[test]
    fn build_fails_too_many_args() {
        let args: Vec<String> = vec![String::from("test"), String::from("test"), String::from("test")];
        assert_eq!(Filename::build(&args).unwrap_err(), "too many arguments passed in")
    }
}