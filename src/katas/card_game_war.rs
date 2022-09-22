// https://github.com/gigasquid/wonderland-clojure-katas/tree/master/card-game-war
use crate::kata::Kata;
use rand::Rng;
use std::collections::HashMap;

#[derive(Clone, Copy)]
enum Color {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Clone, Copy)]
enum Value {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Jack,
    Queen,
    King,
    Ace,
    Empty,
}

impl Value {
    fn to_i32(&self) -> i32 {
        match *self {
            Value::Empty => 0,
            Value::One => 1,
            Value::Two => 2,
            Value::Three => 3,
            Value::Four => 4,
            Value::Five => 5,
            Value::Six => 6,
            Value::Seven => 7,
            Value::Eight => 8,
            Value::Nine => 9,
            Value::Jack => 10,
            Value::Queen => 11,
            Value::King => 12,
            Value::Ace => 13,
        }
    }
}

#[derive(Clone, Copy)]
struct Card {
    color: Color,
    value: Value,
}

impl Card {}

struct Game {
    deck: Vec<Card>,
    player1: Vec<Card>,
    player2: Vec<Card>,
}

impl Game {
    fn new() -> Self {
        let mut game = Game {
            deck: Game::shuffle(vec![
                Card {
                    color: Color::Clubs,
                    value: Value::One,
                },
                Card {
                    color: Color::Clubs,
                    value: Value::Two,
                },
                Card {
                    color: Color::Clubs,
                    value: Value::Three,
                },
                Card {
                    color: Color::Clubs,
                    value: Value::Four,
                },
                Card {
                    color: Color::Clubs,
                    value: Value::Five,
                },
                Card {
                    color: Color::Clubs,
                    value: Value::Six,
                },
                Card {
                    color: Color::Clubs,
                    value: Value::Seven,
                },
                Card {
                    color: Color::Clubs,
                    value: Value::Eight,
                },
                Card {
                    color: Color::Clubs,
                    value: Value::Nine,
                },
                Card {
                    color: Color::Clubs,
                    value: Value::Jack,
                },
                Card {
                    color: Color::Clubs,
                    value: Value::Queen,
                },
                Card {
                    color: Color::Clubs,
                    value: Value::King,
                },
                Card {
                    color: Color::Clubs,
                    value: Value::Ace,
                },
                Card {
                    color: Color::Hearts,
                    value: Value::One,
                },
                Card {
                    color: Color::Hearts,
                    value: Value::Two,
                },
                Card {
                    color: Color::Hearts,
                    value: Value::Three,
                },
                Card {
                    color: Color::Hearts,
                    value: Value::Four,
                },
                Card {
                    color: Color::Hearts,
                    value: Value::Five,
                },
                Card {
                    color: Color::Hearts,
                    value: Value::Six,
                },
                Card {
                    color: Color::Hearts,
                    value: Value::Seven,
                },
                Card {
                    color: Color::Hearts,
                    value: Value::Eight,
                },
                Card {
                    color: Color::Hearts,
                    value: Value::Nine,
                },
                Card {
                    color: Color::Hearts,
                    value: Value::Jack,
                },
                Card {
                    color: Color::Clubs,
                    value: Value::Queen,
                },
                Card {
                    color: Color::Hearts,
                    value: Value::King,
                },
                Card {
                    color: Color::Hearts,
                    value: Value::Ace,
                },
                Card {
                    color: Color::Spades,
                    value: Value::One,
                },
                Card {
                    color: Color::Spades,
                    value: Value::Two,
                },
                Card {
                    color: Color::Spades,
                    value: Value::Three,
                },
                Card {
                    color: Color::Spades,
                    value: Value::Four,
                },
                Card {
                    color: Color::Spades,
                    value: Value::Five,
                },
                Card {
                    color: Color::Spades,
                    value: Value::Six,
                },
                Card {
                    color: Color::Spades,
                    value: Value::Seven,
                },
                Card {
                    color: Color::Spades,
                    value: Value::Eight,
                },
                Card {
                    color: Color::Spades,
                    value: Value::Nine,
                },
                Card {
                    color: Color::Spades,
                    value: Value::Jack,
                },
                Card {
                    color: Color::Spades,
                    value: Value::Queen,
                },
                Card {
                    color: Color::Spades,
                    value: Value::King,
                },
                Card {
                    color: Color::Spades,
                    value: Value::Ace,
                },
                Card {
                    color: Color::Diamonds,
                    value: Value::One,
                },
                Card {
                    color: Color::Diamonds,
                    value: Value::Two,
                },
                Card {
                    color: Color::Diamonds,
                    value: Value::Three,
                },
                Card {
                    color: Color::Diamonds,
                    value: Value::Four,
                },
                Card {
                    color: Color::Diamonds,
                    value: Value::Five,
                },
                Card {
                    color: Color::Diamonds,
                    value: Value::Six,
                },
                Card {
                    color: Color::Diamonds,
                    value: Value::Seven,
                },
                Card {
                    color: Color::Diamonds,
                    value: Value::Eight,
                },
                Card {
                    color: Color::Diamonds,
                    value: Value::Nine,
                },
                Card {
                    color: Color::Diamonds,
                    value: Value::Jack,
                },
                Card {
                    color: Color::Diamonds,
                    value: Value::Queen,
                },
                Card {
                    color: Color::Diamonds,
                    value: Value::King,
                },
                Card {
                    color: Color::Diamonds,
                    value: Value::Ace,
                },
            ]),
            player1: vec![],
            player2: vec![],
        };

        let mut deal_to_player1 = true;
        for card in &game.deck {
            if deal_to_player1 {
                game.player1.push(card.clone());
            } else {
                game.player2.push(card.clone());
            }
            deal_to_player1 = !deal_to_player1;
        }

        game
    }

    fn shuffle(mut arr: Vec<Card>) -> Vec<Card> {
        let length = arr.len();
        let mut rng = rand::thread_rng();
        let mid = length / 2;

        for x in 0..mid {
            let rand_index = rng.gen_range(0..length);
            let temp = arr[rand_index];
            arr[rand_index] = arr[x];
            arr[x] = temp;
        }

        arr
    }

    fn draw(&mut self, count: i32) -> Vec<(Card, Card)> {
        let mut draws: Vec<(Card, Card)> = Vec::new();

        for _ in 0..count {
            let mut card1 = Card {
                color: Color::Clubs,
                value: Value::Empty,
            };

            if self.player1.len() > 0 {
                card1 = self.player1.pop().unwrap().clone();
            }

            let mut card2 = Card {
                color: Color::Clubs,
                value: Value::Empty,
            };

            if self.player2.len() > 0 {
                card2 = self.player2.pop().unwrap().clone();
            }

            draws.push((card1, card2));
        }

        draws
    }
}

pub struct CardGameWar {}

impl CardGameWar {
    fn play() {
        let mut game = Game::new();

        let mut draws: Vec<(Card, Card)> = Vec::new();
        loop {
            let draw = game.draw(1)[0];
            let player1_value = Value::to_i32(&draw.0.value);
            let player2_value = Value::to_i32(&draw.1.value);
            draws.push(draw);

            println!("Draw {} | {}", player1_value, player2_value,);

            if player1_value == player2_value {
                println!("Even");
                draws.append(&mut game.draw(3));
            }

            for d in &draws {
                if Value::to_i32(&d.0.value) == Value::to_i32(&Value::Empty) {
                    println!("Player 1 won");
                    return;
                }
                if Value::to_i32(&d.1.value) == Value::to_i32(&Value::Empty) {
                    println!("Player 2 won");
                    return;
                }
            }

            if player1_value > player2_value {
                println!("Player 1 won round");
                for d in &draws {
                    game.player1.insert(0, d.0);
                    game.player1.insert(0, d.1);
                }
                draws.clear();
            } else if player2_value > player1_value {
                println!("Player 2 won round");
                for d in &draws {
                    game.player2.insert(0, d.0);
                    game.player2.insert(0, d.1);
                }
                draws.clear();
            }

            println!("Player 1 cards: {}", game.player1.len());
            println!("Player 2 cards: {}", game.player2.len());
            println!("");
        }
    }
}

impl Kata for CardGameWar {
    fn get_entries(&self) -> HashMap<String, &dyn Fn()> {
        let mut entries: HashMap<String, &dyn Fn()> = HashMap::new();
        entries.insert(String::from("Play"), &(|| CardGameWar::play()));

        entries
    }

    fn new() -> Self {
        CardGameWar {}
    }
}
