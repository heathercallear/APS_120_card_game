use clap::Parser;
use std::time::Instant;

use deterministic_card_game::{Cli, DataSaver, Game};

fn main() {
    let cli = Cli::parse();
    // record time if needed for elapsed time printing
    let now: Option<Instant> = if cli.time_elapsed {
        Some(std::time::Instant::now())
    } else {
        None
    };

    let mut game = Game::new();
    if cli.save_data {
        save_data(cli.max_exponent, cli.quiet);
    } else if cli.quiet > 0 {
        game.play_games(10usize.pow(cli.max_exponent as u32));
        // if only asked for the first level of quiet, still print final results
        if cli.quiet == 1 {
            print_game_results(&game, &cli.max_exponent, cli.max_exponent + 1);
        }
    } else {
        // if no level of quiet requested, print results at each power of ten
        game.play_games(10);
        let mut already_ran: usize = 10;
        for exponent in 2..=cli.max_exponent {
            game.play_games(already_ran * 9);
            already_ran *= 10;
            print_game_results(&game, &exponent, cli.max_exponent + 1);
        }
    }

    // done now, print elapsed time if asked
    if cli.time_elapsed {
        println!("Time elapsed: {} ms", now.unwrap().elapsed().as_millis());
    }
}

fn print_game_results(game: &Game, exponent: &usize, max_runs_digits: usize) {
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
        exponent, game.total_runs, max_runs_digits, results_proportion_str,
    );
    println!();
}

fn save_data(max_exponent: usize, quiet: u8) {
    // set up data_saver (and make data folder if required and allowed)
    let mut data_saver = match DataSaver::new("./card_game_data") {
        Ok(data_saver) => data_saver,
        Err(err) => {
            eprintln!("Error: failed to find/make data folder: {err}");
            return;
        }
    };
    // set data saving to be quiet if any quiet flags have been given
    data_saver.quiet = 0 < quiet;
    // run games and write result data to csv file
    if let Err(err) = data_saver.write_data(2, max_exponent, 9) {
        eprintln!("Error: failed to write data to file: {err}");
        return;
    }
    if quiet < 2 {
        println!(
            "Data file {} finished.",
            data_saver
                .file_name
                .expect("write_data should set the `file_name` field")
        );
        println!("Total runs: {}", data_saver.game.total_runs);
    }
}
