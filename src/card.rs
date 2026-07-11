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
    full: Vec<Card>,
    current: Vec<Card>,
    rng: rand::rngs::ThreadRng,
}

impl Deck {
    pub fn new() -> Self {
        let mut full = Vec::with_capacity(52);
        for suit in SUITS {
            for rank in RANKS {
                full.push(Card { suit, rank });
            }
        }
        let current = Vec::with_capacity(52);
        let mut deck = Self {
            full,
            current,
            rng: rand::rng(),
        };
        deck.reset();
        deck
    }

    pub fn reset(&mut self) {
        self.current.clear();
        self.current.extend(&self.full);
        self.current.shuffle(&mut self.rng);
    }

    pub fn pop(&mut self) -> Option<Card> {
        self.current.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deck_new_test() {
        let deck = Deck::new();
        assert!(deck.full.len() == 52, "Full deck length should be 52");
        assert!(
            deck.current.len() == 52,
            "Current deck length should start as 52"
        );
        assert!(deck.full != deck.current, "Current deck should be shuffled");
    }

    #[test]
    fn deck_pop() {
        let mut deck = Deck::new();
        assert!(deck.full.len() == 52, "Full deck length should be 52");
        assert!(
            deck.current.len() == 52,
            "Current deck length should start as 52"
        );
        assert!(deck.full != deck.current, "Current deck should be shuffled");
        let mut popped_cards = Vec::with_capacity(52);
        for i in 1..=52 {
            popped_cards.push(deck.pop());
            assert!(
                deck.full.len() == 52,
                "Full deck length should not be changed by pop"
            );
            assert!(
                deck.current.len() == 52 - i,
                "Current deck length should be {} after popping {} times",
                52 - i,
                i
            );
        }
        // Double check current deck is empty
        assert!(deck.full.len() == 52, "Full deck length should not be changed after popping");
        assert!(deck.current.len() == 0, "Current deck length should be 0");
        // Double check that reset works
        deck.reset();
        assert!(deck.full.len() == 52, "Full deck length should still be 52 after resetting");
        assert!(deck.current.len() == 52, "Current deck length should be 52 again after resetting");
    }
}
