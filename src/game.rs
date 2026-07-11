use crate::card::{Card, Deck};

pub struct Game {
    deck: Deck,
    hand: Vec<Card>,
    finished: bool,
    pub results: [usize; 53],
    pub total_runs: usize,
}

impl Game {
    pub fn new() -> Self {
        Game {
            deck: Deck::new(),
            hand: Vec::with_capacity(52),
            finished: false,
            // array if number of times a game has had a certain result
            // index with the number of cards remaining in the hand at the end of the game
            results: [0; 53],
            total_runs: 0,
        }
    }

    fn reset(&mut self) {
        self.deck.reset();
        self.hand.clear();
        self.finished = false;
    }

    pub fn play_game(&mut self) -> usize {
        self.reset();
        // deal first four cards
        self.ensure_four_cards();
        self.remove_cards();
        while !self.finished {
            self.play_turn();
        }
        self.hand.len()
    }

    pub fn play_games(&mut self, n: usize) {
        for _ in 0..n {
            self.results[self.play_game()] += 1;
            self.total_runs += 1;
        }
    }

    pub fn reset_results(&mut self) {
        self.results = [0; 53];
        self.total_runs = 0;
    }

    pub fn get_results_proportion(&self) -> Vec<f64> {
        let mut results_proportion = Vec::with_capacity(53);
        let total_runs = self.total_runs as f64;
        for result in self.results {
            results_proportion.push(result as f64 / total_runs);
        }
        results_proportion
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
