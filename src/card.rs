use rand::seq::SliceRandom;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Suit {
    Clubs,
    Spades,
    Hearts,
    Diamonds,
}

const SUITS: [Suit; 4] = [Suit::Clubs, Suit::Spades, Suit::Hearts, Suit::Diamonds];

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

const RANKS: [Rank; 13] = [
    Rank::Ace,
    Rank::Two,
    Rank::Three,
    Rank::Four,
    Rank::Five,
    Rank::Six,
    Rank::Seven,
    Rank::Eight,
    Rank::Nine,
    Rank::Ten,
    Rank::Jack,
    Rank::Queen,
    Rank::King,
];

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub struct Deck {
    cards: [Card; 52],
    rng: rand::rngs::ThreadRng,
    index: usize,
}

impl Deck {
    pub fn new() -> Self {
        let mut deck = Self {
            cards: Deck::get_sorted_deck(),
            rng: rand::rng(),
            index: 52,
        };
        deck.reset();
        deck
    }

    pub fn get_sorted_deck() -> [Card; 52] {
        let mut cards = [Card {
            suit: Suit::Clubs,
            rank: Rank::Ace,
        }; 52];
        let mut card_index = 0;
        for suit in SUITS {
            for rank in RANKS {
                cards[card_index] = Card { suit, rank };
                card_index += 1;
            }
        }
        cards
    }

    pub fn reset(&mut self) {
        self.index = 52;
        self.cards.shuffle(&mut self.rng);
    }

    pub fn pop(&mut self) -> Option<Card> {
        if self.index == 0 {
            None
        } else {
            self.index -= 1;
            Some(self.cards[self.index])
        }
    }
}

#[cfg(test)]
impl Deck {
    /// Set hand to be the same as a full sorted deck.
    pub fn reset_to_sorted(&mut self) {
        let cards = Deck::get_sorted_deck();
        for i in 0..self.cards.len() {
            self.cards[i] = cards[i];
        }
    }
}

#[cfg(test)]
pub mod all_cards_backwards {
    use super::*;
    pub struct AllCardsBackwards {
        iter: Box<dyn Iterator<Item = Card>>,
    }

    impl AllCardsBackwards {
        pub fn new() -> Self {
            AllCardsBackwards {
                iter: Box::new(
                    SUITS
                        .clone()
                        .into_iter()
                        .rev()
                        .map(|suit| {
                            RANKS
                                .clone()
                                .into_iter()
                                .rev()
                                .map(move |rank| Card { rank, suit })
                        })
                        .flatten(),
                ),
            }
        }
    }

    impl Iterator for AllCardsBackwards {
        type Item = Card;

        fn next(&mut self) -> Option<Self::Item> {
            self.iter.next()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deck_new_test() {
        let deck = Deck::new();
        assert_eq!(deck.cards.len(), 52, "Deck length should be 52");
        assert_ne!(
            deck.cards,
            Deck::get_sorted_deck(),
            "Deck should be shuffled"
        );
    }

    #[test]
    fn deck_pop() {
        let mut deck = Deck::new();
        assert_eq!(deck.cards.len(), 52, "Deck length should be 52");
        assert!(
            deck.cards != Deck::get_sorted_deck(),
            "Current deck should be shuffled"
        );
        let mut popped_cards = Vec::with_capacity(52);
        for i in 1..=52 {
            popped_cards.push(
                deck.pop()
                    .expect("Deck should have 52 cards and so keep popping"),
            );
            assert_eq!(
                deck.cards.len(),
                52,
                "Deck length should not be changed by pop"
            );
            assert_eq!(
                deck.index,
                52 - i,
                "Current deck index should be {} after popping {} times",
                52 - i,
                i
            );
        }
        // Double check current deck is empty
        assert_eq!(
            deck.cards.len(),
            52,
            "Deck length should not be changed after popping"
        );
        assert_eq!(deck.index, 0, "Current deck index should be 0");
        assert_eq!(
            deck.pop(),
            None,
            "Popping fully dealt deck should return `None`"
        );
        // Double check that reset works
        deck.reset();
        assert_eq!(
            deck.cards.len(),
            52,
            "Full deck length should still be 52 after resetting"
        );
        assert_ne!(
            Vec::from(&deck.cards),
            popped_cards.into_iter().rev().collect::<Vec<Card>>(),
            "Current deck length should be 52 again after resetting"
        );
    }
}
