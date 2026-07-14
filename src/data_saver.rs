use crate::game::Game;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::Path;

/// Runs the game lots of times and saves the results to a file.
///
/// By default, writing data prints to stdin whenever a new power of 10 number of runs is reached.
/// To disable this, set the `quiet` field to `true`.
pub struct DataSaver<'a> {
    pub game: Game,
    data_folder: &'a Path,
    pub file_name: Option<String>,
    pub quiet: bool,
}

impl<'a> DataSaver<'a> {
    pub fn new(data_folder_name: &'a str, threads: usize) -> io::Result<Self> {
        let data_folder: &'a Path = Path::new(data_folder_name);
        // check that data folder exists, make it if not
        match data_folder.try_exists() {
            Ok(true) => {
                // data folder verified to exist
                if !data_folder.is_dir() {
                    return Err(io::Error::new(
                        io::ErrorKind::NotADirectory,
                        format!(
                            "{} already exists but is not a directory",
                            data_folder.display(),
                        ),
                    ));
                }
            }
            Ok(false) => {
                // data folder verified to not exist
                // ask user whether they want data folder to be automatically created
                if !Self::ask_for_permission(
                    format!(
                        "Data folder {} does not exist. Create this folder? ('y' or 'yes')",
                        data_folder.display(),
                    )
                    .as_str(),
                ) {
                    // user denied automatic creation of the data folder
                    return Err(io::Error::new(
                        io::ErrorKind::Interrupted,
                        "user refused to create the data folder",
                    ));
                };
                // attempt to create the data folder
                // give more information with directory creation error if it occurs
                if let Err(err) = fs::create_dir(data_folder) {
                    return Err(io::Error::new(
                        err.kind(),
                        format!(
                            "Data folder {} does not exist and could not be created: {}",
                            data_folder.display(),
                            err,
                        ),
                    ));
                };
                println!("Data folder {} created.", data_folder.display());
            }
            Err(err) => {
                return Err(io::Error::new(
                    err.kind(),
                    format!(
                        "Data folder {} could not be verified to exist: {}",
                        data_folder.display(),
                        err,
                    ),
                ));
            }
        };
        Ok(Self {
            game: Game::new(threads),
            data_folder,
            file_name: None,
            quiet: false,
        })
    }

    // Ask the user for permission to perform an action
    pub fn ask_for_permission(message: &str) -> bool {
        println!("{}", message);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("stdin should be readable");
        if ["y", "yes"].contains(&input.trim()) {
            return true;
        }
        false
    }

    pub fn write_data(
        &mut self,
        min_exponent: usize,
        max_exponent: usize,
        number_of_splits: usize,
    ) -> io::Result<()> {
        // get new file in append mode with automatically increased run number
        let mut file = self.get_new_file(max_exponent, number_of_splits)?;

        // run game exponentially increasing number of times

        if !self.quiet {
            println!("Doing up to 10^{max_exponent} runs.");
        }
        // write header line of the csv
        writeln!(file, "{}", self.get_header_line())?;
        // play some number of games to start with
        if (u32::MAX as usize) < min_exponent {
            panic!("10^{min_exponent} runs is massively too many to attempt")
        }
        self.game.play_games(10usize.pow(min_exponent as u32));
        writeln!(file, "{}", self.get_data_line())?;
        for exponent in min_exponent..max_exponent {
            if !self.quiet {
                println!("Done 10^{exponent} runs. Continuing...");
            }
            // doing powers of 10 runs, so doing 9 times as many runs -> multiply total runs by 10
            let runs_to_reach_next_exponent = self.game.total_runs * 10;
            let runs_in_each_split =
                (runs_to_reach_next_exponent - self.game.total_runs) / number_of_splits;
            for _ in 1..number_of_splits {
                self.game.play_games(runs_in_each_split);
                writeln!(file, "{}", self.get_data_line())?;
            }
            // make sure that the last split includes any straggling remaining runs
            self.game
                .play_games(runs_to_reach_next_exponent - self.game.total_runs);
            writeln!(file, "{}", self.get_data_line())?;
        }
        if !self.quiet {
            println!("Finished doing all 10^{max_exponent} runs.")
        }
        Ok(())
    }

    fn get_new_file(&mut self, max_exponent: usize, number_of_splits: usize) -> io::Result<File> {
        // find file name to use for data
        let mut max_run_number: usize = 0;
        // loop through files in directory, ignoring any files not made by this program
        for existing_file in fs::read_dir(self.data_folder)? {
            let existing_file = existing_file?;
            // ignore anything that isn't a file (eg. directory, symlink)
            if !existing_file.file_type()?.is_file() {
                continue;
            };
            let existing_file_path = existing_file.path();
            let Some(existing_file_os_str) = existing_file_path.file_name() else {
                continue;
            };
            let Some(existing_file_name) = existing_file_os_str.to_str() else {
                continue;
            };
            // only consider file if its name has the expected format
            if !existing_file_name.ends_with(".csv") {
                continue;
            }
            let existing_file_name_parts: Vec<&str> = existing_file_name
                [..existing_file_name.len() - 4] // ignore file ending
                .split('-')
                .collect();
            if existing_file_name_parts.len() != 6 {
                continue;
            }
            if existing_file_name_parts[0] != "run" {
                continue;
            }
            if existing_file_name_parts[2] != "exp" {
                continue;
            }
            if existing_file_name_parts[4] != "splits" {
                continue;
            }
            let Ok(run_number) = existing_file_name_parts[1].parse::<usize>() else {
                continue;
            };
            if max_run_number < run_number {
                max_run_number = run_number;
            }
        }
        // use highest run number of existing data to make new file name
        let file_name = format!(
            "run-{run_number:05}-exp-{max_exponent}-splits-{number_of_splits}.csv",
            run_number = max_run_number + 1,
            max_exponent = max_exponent,
            number_of_splits = number_of_splits,
        );
        // make new file with this name within self.data_folder
        let file_name_str = file_name.as_str();
        let mut data_file = self.data_folder.to_path_buf();
        data_file.push(file_name_str);
        if !self.quiet {
            println!("Using data file {}", file_name);
        }
        self.file_name = Some(file_name);
        // create file
        File::create(data_file.clone())?;
        // open the file in append mode
        OpenOptions::new().append(true).open(data_file)
    }

    fn get_data_line(&self) -> String {
        let mut line = self.game.total_runs.to_string();
        for result in self.game.results {
            line.push_str(", ");
            line += &result.to_string();
        }
        line
    }

    fn get_header_line(&self) -> String {
        let mut line = String::from("runs");
        for cards_left_in_hand in 0..=52 {
            line += format!(", {}", cards_left_in_hand).as_str();
        }
        line
    }
}
