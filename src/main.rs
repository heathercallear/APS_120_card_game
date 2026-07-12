use deterministic_card_game::{DataSaver, Game};

const MAX_EXPONENT: usize = 9;

fn main() {
    if DataSaver::ask_for_permission("Run thoroughly and save data to file? ('y' or 'yes')") {
        let mut data_saver = match DataSaver::new("./card_game_data") {
            Ok(data_saver) => data_saver,
            Err(err) => {
                eprintln!("Error: failed to find/make data folder: {err}");
                return;
            }
        };
        if let Err(err) = data_saver.write_data(2, 10, 9) {
            eprintln!("Error: failed to write data to file: {err}");
            return;
        }
        println!(
            "Data file {} finished.",
            data_saver
                .file_name
                .expect("write_data should set the `file_name` field")
        );
        println!("Total runs: {}", data_saver.game.total_runs);
        // do not do any of the later printing of making a data file
        return;
    }

    // otherwise, just print some results

    let mut game = Game::new();

    game.play_games(10);
    let mut already_ran: usize = 10;
    for exponent in 2..=MAX_EXPONENT {
        game.play_games(already_ran * 9);
        already_ran *= 10;
        // convert results array items to "0" if 0, else scientific notation
        let mut results_proportion_str =
            String::from_iter(game.get_results_proportion().iter().map(|f| {
                if *f == 0f64 {
                    format!("{f}, ")
                } else {
                    format!("{:.5e}, ", f)
                }
            }));
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
