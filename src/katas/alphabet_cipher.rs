// https://github.com/gigasquid/wonderland-clojure-katas/tree/master/alphabet-cipher

use std::collections::HashMap;

use crate::{cli, kata::Kata};

const ALPHABET: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

pub struct AlphabetCipher {}

pub struct SubstitutionChart {
    chart: [[char; 26]; 26],
}

impl SubstitutionChart {
    fn generate_substiution_chart() -> [[char; 26]; 26] {
        let mut chart: [[char; 26]; 26] = [[' '; 26]; 26];

        for set in 0..ALPHABET.len() {
            // set to unrotated set
            chart[set] = ALPHABET.clone();
            // rotate set
            for _ in 0..set {
                let first_char = chart[set][0];

                for i in 0..ALPHABET.len() - 1 {
                    chart[set][i] = chart[set][i + 1];
                }

                chart[set][ALPHABET.len() - 1] = first_char;
            }
        }

        chart
    }

    fn new() -> Self {
        SubstitutionChart {
            chart: Self::generate_substiution_chart(),
        }
    }
}

impl AlphabetCipher {
    fn encrypt() {
        let substitution_chart = SubstitutionChart::new();

        let message = cli::read::<String>(
            "Enter message",
            &|x| x.chars().all(|c| ALPHABET.contains(&c)),
            &format!("Message can only contain: {:?}", ALPHABET),
        );

        let mut keyword = cli::read::<String>(
            "Enter keyword",
            &|x| x.chars().all(|c| ALPHABET.contains(&c)),
            &format!("Keyword can only contain: {:?}", ALPHABET),
        );

        while message.len() > keyword.len() {
            keyword = [keyword.clone(), keyword.clone()].concat();
        }

        let keyword_chars: Vec<char> = keyword[0..message.len()].chars().collect();
        let message_chars: Vec<char> = message.chars().collect();
        let mut encoded_message = String::new();

        for i in 0..keyword_chars.len() {
            let column_index = ALPHABET
                .iter()
                .position(|x| x == &keyword_chars[i])
                .unwrap();

            let row_index = ALPHABET
                .iter()
                .position(|x| x == &message_chars[i])
                .unwrap();

            encoded_message.push(substitution_chart.chart[column_index][row_index]);
        }

        println!("{}", encoded_message);
    }

    fn decrypt() {
        let substitution_chart = SubstitutionChart::new();

        let message = cli::read::<String>(
            "Enter encrypted message",
            &|x| x.chars().all(|c| ALPHABET.contains(&c)),
            &format!("Message can only contain: {:?}", ALPHABET),
        );

        let mut keyword = cli::read::<String>(
            "Enter keyword",
            &|x| x.chars().all(|c| ALPHABET.contains(&c)),
            &format!("Keyword can only contain: {:?}", ALPHABET),
        );

        while message.len() > keyword.len() {
            keyword = [keyword.clone(), keyword.clone()].concat();
        }

        let keyword_chars: Vec<char> = keyword[0..message.len()].chars().collect();
        let message_chars: Vec<char> = message.chars().collect();
        let mut encoded_message = String::new();

        for i in 0..keyword_chars.len() {
            let column_index = ALPHABET
                .iter()
                .position(|x| x == &keyword_chars[i])
                .unwrap();

            for row in 0..substitution_chart.chart.len() {
                let found_char = message_chars
                    .iter()
                    .enumerate()
                    .filter(|&(index, _)| i == index)
                    .map(|(_, key)| key)
                    .next()
                    .unwrap();

                if &substitution_chart.chart[row][column_index] == found_char {
                    encoded_message.push(ALPHABET[row]);

                    break;
                }
            }
        }

        println!("{}", encoded_message);
    }
}

impl Kata for AlphabetCipher {
    fn get_entries(&self) -> HashMap<String, &dyn Fn()> {
        let mut entries: HashMap<String, &dyn Fn()> = HashMap::new();
        entries.insert(String::from("Encrypt"), &(|| AlphabetCipher::encrypt()));
        entries.insert(String::from("Decrypt"), &(|| AlphabetCipher::decrypt()));

        entries
    }

    fn new() -> Self {
        AlphabetCipher {}
    }
}
