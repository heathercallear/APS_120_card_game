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
    pub current: Vec<Card>,
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
