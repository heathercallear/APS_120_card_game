use deterministic_card_game::Game;

fn main() {
    let mut game = Game::new();
    let mut result: usize;

    for _ in 0..10 {
        game.reset();
        result = game.play_game();
        if result == 0 {
            println!("Game won!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
        } else {
            println!("{result} cards left in the hand.")
        }
    }
}
