use super::card::Card;

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
