use std::collections::HashMap;

use crate::kata::Kata;

pub struct Test {}

impl Test {
    fn function_1() {
        println!("Hello World!");
    }

    fn function_2() {
        println!("Hello World 2!");
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
