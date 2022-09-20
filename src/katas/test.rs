use std::collections::HashMap;

use crate::{cli, kata::Kata};

pub struct Test {}

impl Test {
    fn function_1() {
        cli::print("Hello World!");
    }

    fn function_2() {
        cli::print("Hello World 2!");
    }
}

impl Kata for Test {
    fn get_entries(&self) -> HashMap<String, &dyn Fn()> {
        let mut entries: HashMap<String, &dyn Fn()> = HashMap::new();
        entries.insert(String::from("Function 1"), &(|| Test::function_1()));
        entries.insert(String::from("Function 2"), &(|| Test::function_2()));

        entries
    }

    fn new() -> Self {
        Test {}
    }
}
