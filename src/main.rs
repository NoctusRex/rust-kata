use kata::Kata;
use katas::{alphabet_cipher::AlphabetCipher, test::Test};
use std::collections::HashMap;

mod cli;
mod kata;
mod katas;

fn main() {
    let mut entries: HashMap<String, &dyn Fn()> = HashMap::new();
    entries.insert(String::from("Test"), &(|| start_kata::<Test>()));
    entries.insert(
        String::from("Alphabet Cipher"),
        &(|| start_kata::<AlphabetCipher>()),
    );

    loop {
        cli::select("Select Kata", &entries).1();
    }
}

fn start_kata<T>()
where
    T: Kata,
{
    let kata = T::new();
    kata.start();
}
