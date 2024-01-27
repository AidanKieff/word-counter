use regex::Regex;

pub fn stripper(s : &str) -> String {
    let re = Regex::new(r"[!.,?\n]");
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_test() {
        assert_eq!(disemvowel("This website is for losers LOL!"), "Ths wbst s fr lsrs LL!");
    }
}