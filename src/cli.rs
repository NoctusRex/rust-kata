use std::{collections::HashMap, fmt::Debug, io::stdin, str::FromStr, string::String};

fn print(text: &str) {
    println!("{}", text);
}

fn new_line() {
    println!();
}

pub fn read<T: FromStr + Debug>(text: &str, validator: &dyn Fn(&T) -> bool, error_text: &str) -> T
where
    <T as FromStr>::Err: Debug,
{
    loop {
        print(&text);

        let mut input = String::new();
        stdin().read_line(&mut input).expect("");
        let input: Result<T, _> = input.trim().parse();

        if input.is_ok() {
            let input: T = input.unwrap();

            if validator(&input) {
                break input;
            }
        }

        print(&error_text);
        new_line();
    }
}

pub fn select<'a, T>(text: &'a str, options: &'a HashMap<String, T>) -> (&'a String, &'a T) {
    loop {
        print(text);
        let options_count: usize = options.len();

        for i in 0..options_count {
            let option = options
                .keys()
                .enumerate()
                .filter(|&(index, _)| index == i)
                .map(|(_, key)| key)
                .next()
                .unwrap();

            print(&format!("{}) {}", i + 1, option));
        }

        let mut input = String::new();
        stdin().read_line(&mut input).expect("");
        let input: Result<usize, _> = input.trim().parse();

        if input.is_ok() {
            let input: usize = input.unwrap();

            if input > 0 && input - 1 < options_count {
                let value = options
                    .into_iter()
                    .enumerate()
                    .filter(|&(index, _)| index == input - 1)
                    .map(|(_, key)| key)
                    .next()
                    .unwrap();

                break value;
            }
        }
    }
}
