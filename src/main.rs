use deterministic_card_game::Game;

fn main() {
    let mut game = Game::new();

    for _ in 0..10 {
        game.play_games(1_000_000);
        println!(
            "{} runs {:?}",
            game.total_runs,
            game.get_results_proportion()
        );
        println!();
    }
}
