use std::{env, process};
use word_counter::Filename;
fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = Filename::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem with argumernt input: {}", err);
        eprintln!("run example: cargo run -- file.txt");
        process::exit(1);
    });

    if let Err(e) = word_counter::run(filename) {
        eprintln!("Problem with argumernt output: {}",e);
        eprintln!("run example: cargo run -- file.txt");
        process::exit(1);
    } 
}
