use std::collections::HashMap;

use crate::cli;

pub trait Kata {
    fn start(&self) {
        let mut entries = Self::get_entries(&self);
        entries.insert(String::from("Exit"), &|| {});

        loop {
            let entry = cli::select("Select entry to execute", &entries);

            if entry.0 == "Exit" {
                break;
            }

            entry.1();
        }
    }

    fn get_entries(&self) -> HashMap<String, &dyn Fn()>;

    fn new() -> Self;
}
