use deterministic_card_game::Game;

const MAX_EXPONENT: usize = 9;

fn main() {
    let mut game = Game::new();

    game.play_games(10);
    let mut already_ran: usize = 10;
    for exponent in 2..=MAX_EXPONENT {
        game.play_games(already_ran * 9);
        already_ran *= 10;
        // convert results array items to "0" if 0, else scientific notation
        let mut results_proportion_str = String::from_iter(
            game.get_results_proportion().iter().map(
                |f| if *f == 0f64 {format!("{f}, ")} else {format!("{:.5e}, ", f)}
            )
        );
        // removing trailing commma and space
        results_proportion_str.pop();
        results_proportion_str.pop();
        // print output
        println!(
            "10^{0} ({1:>2$}) runs: {3}",
            exponent,
            game.total_runs,
            MAX_EXPONENT + 1,
            results_proportion_str,
        );
        println!();
    }
}
