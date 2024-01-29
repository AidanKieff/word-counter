use std::{fs::{File, OpenOptions}, io::Error};

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
        assert_eq!(stripper("This website is for losers LOL!!,.\n"), "This website is for losers LOL");
    }

    #[test]
    #[should_panic]
    fn test_failed_to_open_file() {
        open_file("notarealfile.txt").unwrap();
    }
    
}