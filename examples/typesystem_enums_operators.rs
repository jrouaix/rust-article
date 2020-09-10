#![allow(dead_code)]
// Commençons par définir 3 énumérations
// - Les 4 couleurs d'un jeu de cartes
// - Les valeurs possibles pour celles qui
//   ne sont pas des figures
// - Les cartes possibles : As, Roi, Dame ...
// Chaque carte ayant besoin des
// autres enum pour se définir

// Vous pouvez voir aussi l'utilisation de
// l'attribut `derive` pour implémenter
// automatiquement le trait `PartialEq`
// qui va nous permettre d'utiliser
// l'opérateur d'égalité `==` dans les tests
// il est parfaitement possible d'implémenter
// manuellement le trait `PartialEq`,
// mais #[derive(PartialEq)] le fait ici très bien

use std::cmp::{Ordering, PartialOrd};
#[rustfmt::skip]
#[derive(PartialEq)]
enum Color { Club, Diamond, Heart, Spade }

#[rustfmt::skip]
#[derive(PartialEq)]
enum N { _2, _3, _4, _5, _6, _7, _8, _9, _10 }

#[rustfmt::skip]
#[derive(PartialEq)]
enum Card { 
    Ace(Color), King(Color), Queen(Color), 
    Jack(Color), Number(Color, N) 
}

// Ici l'implementation du trait `PartialOrd` et de la methode
// `value()` nous permet de comparer les valeurs
// en nous basant sur leur représentation numérique

impl PartialOrd for N {
    fn partial_cmp(
        &self,
        other: &N,
    ) -> std::option::Option<std::cmp::Ordering> {
        self.value().partial_cmp(&other.value())
    }
}

#[rustfmt::skip]
impl N {
    // Donne la valeur numérique d'une carte
    fn value(&self) -> u8 {
        match self { 
            N::_10 => 10, N::_9 => 9, N::_8 => 8, 
            N::_7 => 7, N::_6 => 6, N::_5 => 5, 
            N::_4 => 4, N::_3 => 3, N::_2 => 2,
        }
    }
}

// Et enfin voici l'implémentation de la comparaison des cartes
// (comparaison `poker`, les couleurs n'ont pas d'importance)
// Le pattern matching est nécessairement complet,
// s'il ne l'est pas, ça ne compile pas

impl PartialOrd for Card {
    fn partial_cmp(
        &self,
        other: &Card,
    ) -> std::option::Option<std::cmp::Ordering> {
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
// Rien dans le main, mais voici les tests.

// cet attribut permet de ne compiler le module test
// que si on execute les tests, il ne seraient pas
// compilés et donc livré dans la config --release
#[cfg(test)]
mod tests {
    // pour ajouter au scope tout ce est au dessus
    use super::{Card::*, Color::*, *};

    #[test]
    fn equality_of_some_cards() {
        assert_eq!(Ace(Club) == King(Club), false);
        assert_eq!(Ace(Club) == Ace(Club), true);
        assert_eq!(Ace(Club) == Ace(Diamond), false);
        assert_eq!(
            Number(Club, N::_10) == Number(Spade, N::_10),
            false
        );
        assert_eq!(
            Number(Heart, N::_10) == Number(Heart, N::_2),
            false
        );
        assert_eq!(
            Number(Diamond, N::_5) == Number(Heart, N::_5),
            false
        );
    }

    #[test]
    fn compare_some_cards() {
        assert_eq!(Ace(Club) < King(Club), false);
        assert_eq!(Ace(Club) >= King(Club), true);
        assert_eq!(
            Number(Club, N::_2) >= Number(Diamond, N::_2),
            true
        );
        assert_eq!(
            Number(Club, N::_2) <= Number(Diamond, N::_2),
            true
        );
        assert_eq!(
            Number(Club, N::_3) < Number(Diamond, N::_4),
            true
        );
        assert_eq!(
            Number(Club, N::_7) > Number(Diamond, N::_2),
            true
        );
        assert_eq!(King(Spade) <= Ace(Heart), true);
        assert_eq!(Number(Spade, N::_10) > Jack(Heart), false);
    }
}
