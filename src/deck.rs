use cards::Card;

pub struct Deck<T: Card + Clone> {
    cards: Vec<T>
}

impl<T: Card + Clone>  Deck<T> {
    fn new(cards: &[T]) -> Self {
        Deck {
            cards: cards.to_vec(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Eq, Clone)]
    struct NullCard {
        id: i32,
    }

    impl Card for NullCard { }

    #[test]
    fn new_deck() {
        let mut cards = Vec::with_capacity(10);
        for id in 0..10 {
            cards.push(NullCard { id });
        }
        let deck = Deck::new(&cards);
        assert_eq!(cards, deck.cards);
    }
}
