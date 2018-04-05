//! A card which intiates an action
//!
//! This should be used for all combat style cards

use super::card::Card;

/// A card which performs an action
pub struct ActionCard {
    name: String,
}

impl ActionCard {
    fn new(name: String) -> Self {
        ActionCard {
            name
        }
    }
}

impl Card { }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_card() {
        let card = ActionCard::new("foo".to_string());
        assert_eq!(card.name, "foo".to_string());
    }
}
