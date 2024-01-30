use std::{fs::{File, OpenOptions}, io::{Error, BufRead, BufReader}, collections::HashMap};

use regex::Regex;

pub fn stripper(s : &str) -> String {
    let re = Regex::new(r"[!.,?\n]").unwrap();
    let result = re.replace_all(s, "");
    result.to_string()
}

pub fn open_file(filename: &str) -> Result<File, Error> {
    let file = OpenOptions::new()
                                .read(true)
                                .open(filename)?;
    Ok(file)
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
        open_file("notarealfile.txt").unwrap();
    }

    #[test]
    fn opens_file() {
        open_file("example.txt").unwrap();
    }

    #[test]
    fn successful_strip() {
        let file = open_file("test1.txt").unwrap();
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
        let file = open_file("test2.txt").unwrap();
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
}