use crate::card::{Card, Deck};

/// Struct to handle the playing of the deterministic card game.
///
/// # Examples
///
/// ```
/// use deterministic_card_game::Game;
///
/// let mut game = Game::new();
///
/// game.play_games(100);
///
/// // Index `results` by the number of cards left at the end of a game
/// //   to get the number of games that have ended that way.
/// assert_eq!(game.results.len(), 53);
///
/// // Index `get_results_proportion()` by the number of cards left at the end of a game
/// //   to get the proportion of games that have ended that way.
/// assert_eq!(game.get_results_proportion().len(), 53);
///
/// // Use `total_runs` to check how many times the games has been played.
/// assert_eq!(game.total_runs, 100);
///
/// // Use `reset_results` to reset the results and total runs.
/// game.reset_results();
/// assert_eq!(game.results, [0; 53]);
/// assert_eq!(game.get_results_proportion(), vec![0.0; 53]);
/// assert_eq!(game.total_runs, 0);
/// ```
pub struct Game {
    deck: Deck,
    hand: Vec<Card>,
    finished: bool,
    pub results: [usize; 53],
    pub total_runs: usize,
}

impl Game {
    /// Make a new game.
    ///
    /// Note that only a single instance of `Game` is required to play multiple games.
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

    /// Reset the game ready for a new game to be played.
    fn reset(&mut self) {
        self.deck.reset();
        self.hand.clear();
        self.finished = false;
    }

    /// Play a single game
    ///
    /// Returns the number of cards left in the hand at the end of the game.
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

    /// Play `n` games
    ///
    /// Records the result (number of cards left in the hand at the end of a game)
    /// of each game played in `results` field.
    ///
    /// The number of times the game has been played is recorded in the `total_runs` field.
    ///
    /// Note that calling this multiple times with smaller `n` is equivalent to
    /// calling this once with a larger `n` (that is equal to the sum of the smaller `n`s).
    pub fn play_games(&mut self, n: usize) {
        if n <= 1_000 {
            self.play_games_raw(n);
        } else {
            let half_n = n / 2;
            let handle_1 = std::thread::spawn(move || {
                let mut game = Game::new();
                game.play_games(half_n);
                game.results
            });
            let other_half_n = n - n / 2;
            let handle_2 = std::thread::spawn(move || {
                let mut game = Game::new();
                game.play_games(other_half_n);
                game.results
            });
            let results_1 = handle_1.join().unwrap();
            let results_2 = handle_2.join().unwrap();
            for i in 0..self.results.len() {
                self.results[i] += results_1[i] + results_2[i]
            }
            self.total_runs += n;
        }
    }

    fn play_games_raw(&mut self, n: usize) {
        for _ in 0..n {
            self.results[self.play_game()] += 1;
            self.total_runs += 1;
        }
    }

    /// Reset the counts of game results (and total runs) to 0.
    pub fn reset_results(&mut self) {
        self.results = [0; 53];
        self.total_runs = 0;
    }

    /// Get the results of all played games as a proportion.
    ///
    /// Returns a vector of the proportion of times a played game
    /// has ended with each number of cards in the hand.
    ///
    /// # Examples
    ///
    /// The proportion of times a played game has ended
    /// with no (0) cards in the hand would be:
    ///
    /// ```
    /// use deterministic_card_game::Game;
    ///
    /// let mut game = Game::new();
    /// game.play_games(100);
    /// println!("Proportion of games won: {}", game.get_results_proportion()[0]);
    /// ```
    pub fn get_results_proportion(&self) -> Vec<f64> {
        let mut results_proportion = Vec::with_capacity(53);
        let total_runs = match self.total_runs {
            0 => 1f64,
            _ => self.total_runs as f64,
        };
        for result in self.results {
            results_proportion.push(result as f64 / total_runs);
        }
        results_proportion
    }

    /// Draw a card off the deck into the hand.
    ///
    /// Does not return a card, just alters `deck` and `hand` fields.
    ///
    /// If the deck is already empty, sets the `finished` field to `true``
    fn draw_card(&mut self) {
        if let Some(card) = self.deck.pop() {
            self.hand.push(card);
        } else {
            // if attempted to draw from an empty deck, indicate deck is empty
            self.finished = true;
        }
    }

    /// Ensure hand has at least 4 cards, or game marked as finished
    ///
    /// If the hand has less than 4 cards, draw cards until it has 4 cards.
    ///
    /// If the deck becomes empty, the hand may have less than 4 cards still.
    /// In this case, the `finished` field will be set to `true`.
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
            if !self.finished {
                self.remove_cards();
            }
        } else if first_card.suit == last_card.suit {
            let Some(last_card) = self.hand.pop() else {
                panic!("hand should have at least 4 cards if not finished")
            };
            self.hand.pop();
            self.hand.pop();
            self.hand.push(last_card);
            self.ensure_four_cards();
            if !self.finished {
                self.remove_cards();
            }
        }
    }

    /// Play a single turn of the game.
    ///
    /// Draws a new card and then removes cards from the hand if possible.
    fn play_turn(&mut self) {
        self.draw_card();
        self.remove_cards();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::all_cards_backwards::AllCardsBackwards;

    #[test]
    fn game_draw_card_test() {
        let mut game = Game::new();
        game.deck.reset_to_sorted();
        assert_eq!(game.hand.len(), 0, "Game hand should start as empty");
        assert_eq!(
            game.finished, false,
            "Game `finished` field should start as `false`",
        );
        // draw each card in the deck in turn
        let mut hand_size: usize = 0;
        for card in AllCardsBackwards::new() {
            println!("{card:?}");
            hand_size += 1;
            game.draw_card();
            assert_eq!(
                game.hand.len(),
                hand_size,
                "Game hand should have a length of {hand_size}"
            );
            assert_eq!(
                game.hand[hand_size - 1],
                card,
                "Drawn Card number {hand_size} should be {card:?}",
            );
        }
        // check that game has not yet been marked as finished
        assert_eq!(
            game.finished, false,
            "Game `finished` field should still be `false`",
        );
        // double check that hand is the correct length
        assert_eq!(game.hand.len(), 52, "Game hand should end with full deck");
        // draw card when no cards left in the deck
        game.draw_card();
        assert_eq!(
            game.finished, true,
            "Game `finished` field should be `true` after attempting to draw from an empty deck",
        );
    }

    #[test]
    fn ensure_four_cards_test() {
        let mut game = Game::new();
        game.deck.reset_to_sorted();

        assert_eq!(game.hand.len(), 0, "Hand should start off empty");
        // try from an empty hand
        game.ensure_four_cards();
        assert_eq!(game.hand.len(), 4, "Hand should now have 4 cards");
        // try from a 4 card hand
        let old_hand = game.hand.clone();
        game.ensure_four_cards();
        assert_eq!(
            game.hand, old_hand,
            "Hand should not be changed when it has 4 cards already"
        );
        // try from a 3 card hand
        game.hand.pop();
        assert_eq!(game.hand.len(), 3, "Hand should now have 3 cards");
        game.ensure_four_cards();
        assert_eq!(game.hand.len(), 4, "Hand should now have 4 cards");
        assert_eq!(
            game.hand[..3],
            old_hand[..3],
            "First 3 cards in hand should not have changed"
        );
        // try from a 2 card hand
        game.hand.pop();
        game.hand.pop();
        assert_eq!(game.hand.len(), 2, "Hand should now have 2 cards");
        game.ensure_four_cards();
        assert_eq!(game.hand.len(), 4, "Hand should now have 4 cards");
        assert_eq!(
            game.hand[..2],
            old_hand[..2],
            "First 2 cards in hand should not have changed"
        );
        // try from a 1 card hand
        game.hand.pop();
        game.hand.pop();
        game.hand.pop();
        assert_eq!(game.hand.len(), 1, "Hand should now have 3 cards");
        game.ensure_four_cards();
        assert_eq!(game.hand.len(), 4, "Hand should now have 4 cards");
        assert_eq!(
            game.hand[..1],
            old_hand[..1],
            "First card in hand should not have changed"
        );

        // check marking of game as finished

        game.reset();
        for _ in 0..(52 / 4) {
            game.ensure_four_cards();
            for _ in 0..4 {
                game.hand.pop();
            }
        }
        // an empty deck on its own should not mean that the game is finished
        // another turn should still be played to remove cards if possible
        assert_eq!(game.finished, false, "game should not have finished yet");
        // attempt to fill hand when deck is already empty
        // hand no longer able to have at least 4 cards, so game must be finished
        game.ensure_four_cards();
        assert_eq!(game.finished, true, "game should be marked as finished now");
    }

    #[test]
    fn game_remove_cards_test() {
        let mut game = Game::new();
        game.deck.reset_to_sorted();
        // make hand Clubs (A–K), Spades (A–K), Hearts (A–K), Diamonds (A–K)
        for _ in 0..52 {
            game.draw_card();
        }
        assert_eq!(game.hand.len(), 52, "Hand should be 52 long at the start");

        game.remove_cards();
        assert_eq!(
            game.hand.len(),
            16,
            "Recursion should remove 36 = 2*4 + 14*2 cards (now Clubs A–K, Spades A,2,J)"
        );
        // pop King and Queen of Clubs off of the hand so this can continue
        game.hand.remove(12);
        game.hand.remove(12);
        assert_eq!(
            game.hand.len(),
            14,
            "Hand should have been artificially reduced by 2 (now Clubs A–J, Spades A,2,J)"
        );
        game.remove_cards();
        assert_eq!(
            game.hand.len(),
            2,
            "Recursion should remove 12 = 1*4 + 4*2 cards (now Clubs A,10)"
        );
        assert_eq!(
            game.finished, true,
            "Game should be marked as finished (now Clubs A,10)"
        );
    }

    #[test]
    fn game_play_turn_test() {
        let mut game = Game::new();
        // make array of hand sizes in each turn of the known game
        let hand_lengths = [4, 5, 6, 5, 6, 5, 6, 5, 6, 5, 6, 5, 2];
        // turn 1 is the first turn, before `play_turn` has been called
        let mut turn: usize = 1;
        // make known deck (Clubs (A–K), Spades (A–K), Hearts (A–K), Diamonds (A–K))
        game.deck.reset_to_sorted();
        // do rest of the setup and play as in the `play_game` method
        // deal first four cards
        game.ensure_four_cards();
        game.remove_cards();
        // play turns until done
        while !game.finished {
            assert_eq!(
                game.hand.len(),
                hand_lengths[turn - 1],
                "Hand length {} was not the expected {} on turn {}: hand is {:?}",
                game.hand.len(),
                hand_lengths[turn - 1],
                turn,
                game.hand,
            );
            game.play_turn();
            turn += 1;
        }
        assert_eq!(
            game.hand.len(),
            2,
            "This known game should end with 2 cards in the hand"
        );
    }
}
