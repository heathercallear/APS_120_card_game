use crate::card::{Card, Deck};

pub struct Game {
    deck: Deck,
    hand: Vec<Card>,
    finished: bool,
}

impl Game {
    pub fn new() -> Self {
        Game {
            deck: Deck::new(),
            hand: Vec::with_capacity(52),
            finished: false,
        }
    }

    pub fn reset(&mut self) {
        self.deck.reset();
        self.hand.clear();
        self.finished = false;
    }

    pub fn play_game(&mut self) -> usize {
        // deal first four cards
        self.ensure_four_cards();
        self.remove_cards();
        while !self.finished {
            self.play_turn();
        }
        self.hand.len()
    }

    fn draw_card(&mut self) {
        if let Some(card) = self.deck.pop() {
            self.hand.push(card);
        } else {
            // if attempted to draw from an empty deck, indicate deck is empty
            self.finished = true;
        }
    }

    fn ensure_four_cards(&mut self) {
        let length = self.hand.len();
        if length < 4 {
            match length {
                0 => {
                    self.draw_card();
                    self.draw_card();
                    self.draw_card();
                    self.draw_card();
                }
                1 => {
                    self.draw_card();
                    self.draw_card();
                    self.draw_card();
                }
                2 => {
                    self.draw_card();
                    self.draw_card();
                }
                3 => {
                    self.draw_card();
                }
                _ => panic!("Length should be 0, 1, 2, or 3 if length is less than four"),
            }
        }
    }

    /// Remove cards from the hand if needed
    ///
    /// If the 4th last and last card have the same rank, the last 4 cards are removed.
    /// If the 4th last and last card have the same suit, the 3rd last and 2nd last card are removed.
    /// Otherwise, no cards are removed.
    ///
    /// # Panics
    ///
    /// If there are not at least 4 cards in the hand.
    /// This can be ensured using `ensure_four_cards`
    fn remove_cards(&mut self) {
        let Some(last_four_cards) = self.hand.last_chunk::<4>() else {
            panic!("hand should have at least 4 cards after `ensure_four_cards` and not `finished`")
        };
        let first_card = last_four_cards[0];
        let last_card = last_four_cards[3];
        // if ranks are equal, remove all last cards
        if first_card.rank == last_card.rank {
            self.hand.pop();
            self.hand.pop();
            self.hand.pop();
            self.hand.pop();
            self.ensure_four_cards();
        } else if first_card.suit == last_card.suit {
            let Some(last_card) = self.hand.pop() else {
                panic!("hand should have at least 4 cards if not finished")
            };
            self.hand.pop();
            self.hand.pop();
            self.hand.push(last_card);
            self.ensure_four_cards();
        }
    }

    fn play_turn(&mut self) {
        self.draw_card();
        self.remove_cards();
    }
}
