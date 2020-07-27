use std::cmp::{Ordering, PartialOrd};

#[derive(Debug, PartialEq)]
enum Color {
    Club,
    Diamond,
    Heart,
    Spade,
}

#[derive(Debug, PartialEq)]
enum N {
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
    _10,
}

#[derive(Debug, PartialEq)]
enum Card {
    Ace(Color),
    King(Color),
    Queen(Color),
    Jack(Color),
    Number(Color, N),
}

impl N {
    // Donne la valeur numerique d'une carte
    fn value(&self) -> u8 {
        match self {
            N::_10 => 10,
            N::_9 => 9,
            N::_8 => 8,
            N::_7 => 7,
            N::_6 => 6,
            N::_5 => 5,
            N::_4 => 4,
            N::_3 => 3,
            N::_2 => 2,
        }
    }
}

impl PartialOrd for N {
    fn partial_cmp(&self, other: &N) -> std::option::Option<std::cmp::Ordering> {
        self.value().partial_cmp(&other.value())
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Card) -> std::option::Option<std::cmp::Ordering> {
        use Card::*;
        let result = match (self, other) {
            (s, o) if s == o => Ordering::Equal,
            (Ace(_), Ace(_)) => Ordering::Equal,
            (King(_), King(_)) => Ordering::Equal,
            (Queen(_), Queen(_)) => Ordering::Equal,
            (Jack(_), Jack(_)) => Ordering::Equal,
            (Number(_, s), Number(_, o)) => s.partial_cmp(o)?,
            (Ace(_), _) => Ordering::Greater,
            (_, Ace(_)) => Ordering::Less,
            (King(_), _) => Ordering::Greater,
            (_, King(_)) => Ordering::Less,
            (Queen(_), _) => Ordering::Greater,
            (_, Queen(_)) => Ordering::Less,
            (Jack(_), _) => Ordering::Greater,
            (_, Jack(_)) => Ordering::Less,
        };
        Some(result)
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::{Card::*, Color::*, *}; // pour ajouter tout ce est au dessus au scope

    #[test]
    fn equality_of_some_cards() {
        assert_eq!(Ace(Club) == King(Club), false)
    }

    #[test]
    fn compare_some_cards() {
        assert_eq!(Ace(Club) < King(Club), false);
        assert_eq!(Ace(Club) >= King(Club), true);
        assert_eq!(Number(Club, N::_2) >= Number(Diamond, N::_2), true);
        assert_eq!(Number(Club, N::_2) <= Number(Diamond, N::_2), true);
        assert_eq!(Number(Club, N::_3) < Number(Diamond, N::_4), true);
        assert_eq!(Number(Club, N::_7) > Number(Diamond, N::_2), true);
        assert_eq!(King(Spade) <= Ace(Heart), true);
        assert_eq!(Number(Spade, N::_10) > Jack(Heart), false);
    }
}
