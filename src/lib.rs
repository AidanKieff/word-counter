use regex::Regex;

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
        assert_eq!(stripper("This website is for losers LOL!!,.\n"), "This website is for losers LOL");
    }
}