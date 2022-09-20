use kata::Kata;
use katas::test::Test;
use std::collections::HashMap;

mod cli;
mod kata;
mod katas;

fn main() {
    let entries = HashMap::from([(String::from("Test"), Test::new())]);

    loop {
        cli::select("Select Kata", &entries).1.start();
    }
}
