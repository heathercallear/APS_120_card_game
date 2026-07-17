# Deterministic Card Game

`deterministic-card-game` provides an implementation of the card game
described in episode 120 of "A Problem Squared"

## The rules of the game

This is a single-player game.

### Setup

Do these steps to start the game:

- Get a shuffled deck of 52 cards (4 suits, each with 13 different values).
- Prepare to have a hand of cards where the order of the cards must not change.
- Deal 4 cards to yourself
- Follow instructions below on how to remove cards from this hand

### How to remove cards from the hand

Whenever the hand changes (including at the start of the game):

- Check the last 4 cards in the hand:
  - If the value of the first and last card are equal, remove all 4 cards from the hand
  - If the suit of the first and last card are equal, remove the middle 2 cards from the hand
- If cards were removed, repeat the above checks and removals until no cards can be removed
- If the number of cards in the hand is now less than 4, add cards to it until it has 4 cards

### Action each turn

To advance the game by one turn:

- Add a card to the hand (removed from the top of the deck)
- Follow the instructions for how to remove cards from the hand

### When the game finishes

The game finishes when all cards in the deck have been dealt (the deck is empty).

The aim of the game is to have no cards remaining in the hand.

## Basic Usage

### Data generation

Data has already been generated and saved in the [card_game_data](/card_game_data/) folder,
so if you just want to see graphs, feel free to skip to the next section.

Any new data generated is saved as a new CSV file in this same folder.

The code can either be run directly from an executable
(see [the latest release](/releases/latest)),
or by installing [Rust](https://rust-lang.org/) and
using `cargo run --release -- ` from within this repository's folder.

To generate data for $`10^m`$ played games, run:

```bash
$ deterministic-card-game -s m
```

If on Windows, the executable will instead be called "deterministic-card-game.exe".

For example, to generate data for 10,000,000 (10^7) runs:

```bash
$ deterministic-card-game -s 7
```

Or, equivalently, if running from within this repository after installing Rust:

```bash
$ cargo run --release -- -s 7
```

For more details, see the
[Usage of the data generation code](#usage-of-the-data-generation-code)
section.

### Graph plotting

The graph plotting code requires [Python](https://www.python.org/) to be installed.
It also requires the [`matplotlib`](https://matplotlib.org/stable/index.html) module to be installed.
`matplotlib` can be installed using python's pip module.

```bash
$ pip install matplotlib
```

Plots a log-log graph showing the convergence of the proportion of games that
end with a certain number of cards in the hand

The final value reached after all the runs is used as an estimate of the true probability
of finishing a game with a certain number of cards in the hand.

The convergence of the proportion of games ending with this certain number of cards in the hand
can be shown using a log-log plot of its difference from the estimate against the number of runs.

For more details on how the graph is plotted, see the
[How the graph is plotted](#how-the-graph-is-plotted) section.

Use `plot_data.py` to view such a log-log plot.
The graph will open in a new window.
For example, to show a plot for games ending with 5 cards in the hand:

```bash
$ python plot_data.py 5
```

Or to show a plot for the games that were won (ended with no cards left in the hand):

```bash
$ python plot_data.py 0
```

To compare this convergence for won games to the appealing (and very close) value of
$`\frac{1}{100\sqrt{2}}`$ (a tentative value suggested during the podcast episode),
simply run `plot_data.py` with no arguments:

```bash
$ python plot_data.py
```

![Log-log plot comparing convergence of the proportion of won games to its final result versus to 1/100sqrt(2). Until about 10^6 to 10^8 runs, both plots look very similar, steadily (if jaggedly) decreasing. After this, the convergence to 1/100sqrt(2) stays at about 10^-5 difference from the data. However, the convergence to the final result continues to jaggedly decrease down to roughly 10^-7.5 difference from the data after 10^12 runs.](/img/run-00017-plot-__-cards-left.svg)

The difference between $`\frac{1}{100\sqrt{2}}`$ and the final value eventually flattens off at about 10^-5.
This shows that the proportion of won games does not converge to $`\frac{1}{100\sqrt{2}}`$,
but instead to a number that is about 10^-5 different from it.

This particular plot also shows how the precision of the final estimate is indicated.
Up until 10^7 runs, both estimates were steadily being converged to,
and were roughly as accurate as each other.
After this, when the precision of the estimate from run data exceeded 10^-5,
the difference between $`\frac{1}{100\sqrt{2}}`$ and the true proportion of games won became apparent.

Similarly, the final result of 7.06314e-03 can be judged to have a precision of about ±10^-7,
so should perhaps be written as 7.0631e-03 or 0.070631%.

## How the graph is plotted

A single graph only shows results for finishing the game with a certain number of cards, $`k`$, in the hand.
It shows how the estimated probability of a such a game changes as the number of games played increases.
The estimated probability is calculated as the number of games played that ended
with $`k`$ cards left in the hand divided by the total number of games played.

```math
\text{estimated probability of a game ending in }k\text{ cards after }n\text{ runs} =
p(k, n) =
\frac
    {\text{number of games that ended in }k\text{ cards}}
    {\text{number of games played (}n\text{)}}
```

The final estimated probability, $`p(k, n_{max})`$, is the estimated probability after all the games have been played.
The convergence (or lack of it) of the estimated probability to its final value can be seen by
plotting the difference between them.

```math
\text{difference} = d(k, n) = \lvert p(k, n) - p(k, n_{max}) \rvert
```

The graph is a log-log plot of $`d(k, n)`$ against $`n`$ for all recorded $`n`$ except for $`n_{max}`$.
$`n_{max}`$ is not included because showing how close a value is to itself is not useful.

A log scale is used for $`d(k, n)`$ because it starts out large (around the size of the final estimate) and
ends very small (orders of magnitude smaller),
so on a linear scale the difference after many runs would be indistinguishable from 0.

A log scale is also used for $`n`$ because the number of runs used to generate early estimates
is orders of magnitude smaller than the final number of runs,
so on a linear scale early estimates would all be squished against the y axis.

The graph is intended to show the convergence (or lack of it) to a particular value
of the estimated probability of finishing the game with a certain number of cards in the hand
as the number of games played to generate that estimate increases.

By default, the data read from the CSV file in [card_game_data](/card_game_data/) with:
- the highest number of runs
- the most splits (if number of runs is equal)
- the most recent run (if all else is equal)

For the data in this repository, data from
[run number 17 with 10^12 runs](/card_game_data/run-00017-exp-12-splits-90.csv)
is used by default.

The `--run-number` or `-r` option can be used to specify which CSV file should be read from.
For example, the data from
[run number 10 with 10^11 runs](/card_game_data/run-00010-exp-11-splits-9.csv)
could be specified as follows:

```bash
$ python plot_data.py -r=10
```

## Usage of the data generation code

Data from runs is saved in the [card_game_data](/card_game_data) folder.
There are some CSVs in that folder in this repository that were generated using this code.
This is the data used by plot_data.py when making graphs.

All of the data generation code is written in [Rust](https://rust-lang.org/).

Roughly speaking, each run takes 160–180 ns.
So 10^7 runs takes about 1.7 seconds,
10^8 runs takes about 17 seconds,
10^9 runs takes about 2 minutes 50 seconds,
10^10 runs takes about 28 minutes,
10^11 runs takes about 4¾ hours,
and 10^12 runs takes about 1 day and 23 hours.
These times are based on running this program on an alright but not especially fast computer with 4 CPU cores.

### Running the executable

If running using `cargo`, use the `--release` option for improved speed:

```bash
$ cargo run --release --
```

The rest of the examples will instead directly run the executable that this generates.

```bash
$ deterministic-card-game
```

These two methods of running the program are equivalent.

### Default behaviour

If no arguments are given, 10^7 runs of the game are played,
with the proportion of games that have ended in each number of hands printed at every power of 10 runs
(other than 10, since this is far too small a number of results to mean anything).
For example:

```bash
$ deterministic-card-game
10^2 (     100) runs: 1.00000e-2, 0, 3.00000e-2, 0, 5.00000e-2, 0, 1.00000e-1, 0, 1.10000e-1, 0, 1.20000e-1, 0, 5.00000e-2, 0, 7.00000e-2, 0, 1.90000e-1, 0, 5.00000e-2, 0, 4.00000e-2, 0, 6.00000e-2, 0, 4.00000e-2, 0, 2.00000e-2, 0, 3.00000e-2, 0, 1.00000e-2, 0, 2.00000e-2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0

10^3 (    1000) runs: 5.00000e-3, 0, 5.40000e-2, 0, 7.20000e-2, 0, 9.50000e-2, 0, 9.50000e-2, 0, 1.12000e-1, 0, 9.70000e-2, 0, 8.00000e-2, 0, 1.01000e-1, 0, 7.50000e-2, 0, 6.30000e-2, 0, 4.50000e-2, 0, 3.60000e-2, 0, 2.70000e-2, 0, 1.30000e-2, 0, 1.10000e-2, 0, 1.20000e-2, 0, 2.00000e-3, 0, 3.00000e-3, 0, 2.00000e-3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0

10^4 (   10000) runs: 6.90000e-3, 0, 4.46000e-2, 0, 7.70000e-2, 0, 9.56000e-2, 0, 1.03100e-1, 0, 9.58000e-2, 0, 9.92000e-2, 0, 9.51000e-2, 0, 8.64000e-2, 0, 7.32000e-2, 0, 5.82000e-2, 0, 4.97000e-2, 0, 4.13000e-2, 0, 2.60000e-2, 0, 2.00000e-2, 0, 1.09000e-2, 0, 7.50000e-3, 0, 4.50000e-3, 0, 2.90000e-3, 0, 1.40000e-3, 0, 4.00000e-4, 0, 3.00000e-4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0

10^5 (  100000) runs: 7.25000e-3, 0, 4.62800e-2, 0, 7.89700e-2, 0, 9.38500e-2, 0, 9.88400e-2, 0, 1.00900e-1, 0, 9.92200e-2, 0, 9.32300e-2, 0, 8.43100e-2, 0, 7.37600e-2, 0, 6.13600e-2, 0, 5.01100e-2, 0, 3.83200e-2, 0, 2.75600e-2, 0, 1.79300e-2, 0, 1.21900e-2, 0, 7.46000e-3, 0, 4.46000e-3, 0, 2.10000e-3, 0, 1.16000e-3, 0, 5.00000e-4, 0, 1.60000e-4, 0, 4.00000e-5, 0, 3.00000e-5, 0, 1.00000e-5, 0, 0, 0, 0

10^6 ( 1000000) runs: 7.11800e-3, 0, 4.56370e-2, 0, 7.99470e-2, 0, 9.36100e-2, 0, 9.84850e-2, 0, 9.98770e-2, 0, 9.87600e-2, 0, 9.32490e-2, 0, 8.53550e-2, 0, 7.44420e-2, 0, 6.21760e-2, 0, 4.99410e-2, 0, 3.77830e-2, 0, 2.75140e-2, 0, 1.88340e-2, 0, 1.22060e-2, 0, 7.33800e-3, 0, 3.99700e-3, 0, 2.05000e-3, 0, 1.01800e-3, 0, 4.42000e-4, 0, 1.52000e-4, 0, 4.80000e-5, 0, 1.60000e-5, 0, 5.00000e-6, 0, 0, 0, 0

10^7 (10000000) runs: 7.09130e-3, 0, 4.58752e-2, 0, 8.02648e-2, 0, 9.37406e-2, 0, 9.82049e-2, 0, 9.98977e-2, 0, 9.85509e-2, 0, 9.33529e-2, 0, 8.51095e-2, 0, 7.42697e-2, 0, 6.21975e-2, 0, 4.97821e-2, 0, 3.78442e-2, 0, 2.75308e-2, 0, 1.88655e-2, 0, 1.21856e-2, 0, 7.33110e-3, 0, 4.11240e-3, 0, 2.12690e-3, 0, 1.01870e-3, 0, 4.25000e-4, 0, 1.51600e-4, 0, 5.36000e-5, 0, 1.30000e-5, 0, 4.00000e-6, 0, 5.00000e-7, 0, 0
```

So the above output says that after 10^7 runs:
  - 7.09130e-3 (~0.7%) of games ended with 0 cards left in the hand (the winning condition)
  - 4.58752e-2 (~4.6%) of games ended with 1 card left in the hand
  - 8.02648e-2 (~8.0%) of games ended with 2 cards left in the hand
  - 9.98977e-2 (~10.0%) of games ended with 10 cards left in the hand
  - 3.78442e-2 (~3.8%) of games ended with 24 cards left in the hand
  - 2.12690e-3 (~0.2%) of games ended with 36 cards left in the hand
  - 4.00000e-6 (40 of the 10^7) of games ended with 48 cards left in the hand
  - 0 of the games ended with 52 cards left in the hand

There are truly never any odd number of cards left in the hand,
since the game starts with an even number of cards in the deck (52),
all cards in the deck are eventually transferred to the hand,
and only an even number of cards can be removed from the hand (2 or 4).

#### Changing the number of runs

If an integer n is provided, only 10^n runs of the game are played:

```bash
$ deterministic-card-game 4
10^2 (  100) runs: 0, 0, 5.00000e-2, 0, 8.00000e-2, 0, 8.00000e-2, 0, 9.00000e-2, 0, 1.50000e-1, 0, 8.00000e-2, 0, 8.00000e-2, 0, 1.00000e-1, 0, 7.00000e-2, 0, 7.00000e-2, 0, 2.00000e-2, 0, 6.00000e-2, 0, 1.00000e-2, 0, 3.00000e-2, 0, 1.00000e-2, 0, 0, 0, 2.00000e-2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0

10^3 ( 1000) runs: 9.00000e-3, 0, 3.90000e-2, 0, 7.10000e-2, 0, 1.09000e-1, 0, 9.80000e-2, 0, 1.07000e-1, 0, 1.06000e-1, 0, 8.50000e-2, 0, 9.80000e-2, 0, 7.40000e-2, 0, 5.90000e-2, 0, 4.30000e-2, 0, 3.30000e-2, 0, 2.40000e-2, 0, 2.20000e-2, 0, 7.00000e-3, 0, 7.00000e-3, 0, 6.00000e-3, 0, 2.00000e-3, 0, 1.00000e-3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0

10^4 (10000) runs: 7.00000e-3, 0, 4.36000e-2, 0, 8.17000e-2, 0, 9.46000e-2, 0, 9.31000e-2, 0, 1.01600e-1, 0, 9.73000e-2, 0, 9.50000e-2, 0, 8.78000e-2, 0, 7.78000e-2, 0, 5.95000e-2, 0, 4.84000e-2, 0, 3.94000e-2, 0, 2.70000e-2, 0, 2.19000e-2, 0, 1.00000e-2, 0, 8.20000e-3, 0, 2.60000e-3, 0, 2.10000e-3, 0, 1.10000e-3, 0, 2.00000e-4, 0, 1.00000e-4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
```

#### Quiet run

If run with the `--quiet` or `-q` flag, the results are only printed after all runs have been played.
For example:

```bash
$ deterministic-card-game -q
10^7 (10000000) runs: 7.09280e-3, 0, 4.59337e-2, 0, 8.03776e-2, 0, 9.37450e-2, 0, 9.80941e-2, 0, 9.99757e-2, 0, 9.84476e-2, 0, 9.33234e-2, 0, 8.48621e-2, 0, 7.42482e-2, 0, 6.23260e-2, 0, 4.98247e-2, 0, 3.80385e-2, 0, 2.75391e-2, 0, 1.88051e-2, 0, 1.21905e-2, 0, 7.28830e-3, 0, 4.12490e-3, 0, 2.11520e-3, 0, 1.00060e-3, 0, 4.20300e-4, 0, 1.58000e-4, 0, 5.19000e-5, 0, 1.26000e-5, 0, 3.60000e-6, 0, 5.00000e-7, 0, 0
```

### Save data

If run with the `--save-data` or `-s` flag, results are saved to a CSV.

The number of runs at each recorded data point goes up in powers of 10,
with each power of 10 getting 10 data points.
For example: 100, 200, 300, 400, 500, 600, 700, 800, 900, 1000, 2000, 3000, ...

The CSVs in this repository (in the [card_game_data folder](/card_game_data/))
were all produced using this flag.
For example, [this CSV](/card_game_data/run-00010-exp-11-splits-9.csv)
was made using this command with the following output:

```bash
$ deterministic-card-game 11 -s -e
Using data file run-00010-exp-11-splits-9.csv
Doing up to 10^11 runs.
Done 10^2 runs. Continuing...
Done 10^3 runs. Continuing...
Done 10^4 runs. Continuing...
Done 10^5 runs. Continuing...
Done 10^6 runs. Continuing...
Done 10^7 runs. Continuing...
Done 10^8 runs. Continuing...
Done 10^9 runs. Continuing...
Done 10^10 runs. Continuing...
Finished doing all 10^11 runs.
Data file run-00010-exp-11-splits-9.csv finished.
Total runs: 100000000000
Time elapsed: 17350896 ms
```

Whenever this program is run:

- a new file is made in the 'card_game_data' folder of the current working directory.
  If there is no such folder, it will request permission to create such a folder.
  If this permission is given, it will do so and continue as normal, otherwise it will exit.
- the new file will have a run number incremented by one.
  For example, if the above code were run again it would save a new file called
  "run-00002-exp-10-splits-9.csv".
  This means that data files are never overwritten by the program.
- each row of data in the CSV is saved to the file whenever it is calculated.
  This means that if the program is cancelled before completion,
  data that was already been calculated will still be in the file.
  You can view this file even while the program is running to see what data it has outputted.

#### Quiet run

If run with the `--quiet` or `-q` flag, only the print if the runs finishing occurs.
For example, the above run would have had this output if run with the `-q` flag:

```bash
$ deterministic-card-game 10 -s -q
Data file run-00002-exp-10-splits-9.csv finished.
```

### Time elapsed

The `--elapsed-time` or `-e` flag can be added to any of the other options
to print how long the program took (in milliseconds) to run all the games that it played.
For example:

```bash
$ deterministic-card-game 7 -s -q -e
Data file run-00003-exp-7-splits-9.csv finished.
Time elapsed: 1340 ms
```

Actual time taken will vary a little, and may be different on other computers
(this example was not run on an especially fast computer).
Also, of course, higher numbers of runs will take longer.
Remember that the input is an exponent of a power of 10 on the number of runs to perform,
so the time taken to complete will increase exponentially with this input value.

```bash
$ deterministic-card-game 3 -q -e
10^3 (1000) runs: 9.00000e-3, 0, 4.90000e-2, 0, 6.80000e-2, 0, 1.04000e-1, 0, 8.40000e-2, 0, 1.00000e-1, 0, 1.00000e-1, 0, 8.50000e-2, 0, 9.10000e-2, 0, 6.70000e-2, 0, 6.20000e-2, 0, 5.10000e-2, 0, 4.50000e-2, 0, 3.30000e-2, 0, 2.30000e-2, 0, 1.40000e-2, 0, 4.00000e-3, 0, 3.00000e-3, 0, 4.00000e-3, 0, 3.00000e-3, 0, 1.00000e-3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0

Time elapsed: 0 ms
$ deterministic-card-game 4 -q -e
10^4 (10000) runs: 5.70000e-3, 0, 4.38000e-2, 0, 8.34000e-2, 0, 9.58000e-2, 0, 9.28000e-2, 0, 9.71000e-2, 0, 1.00000e-1, 0, 8.91000e-2, 0, 8.56000e-2, 0, 7.48000e-2, 0, 6.27000e-2, 0, 5.24000e-2, 0, 4.03000e-2, 0, 2.97000e-2, 0, 1.84000e-2, 0, 1.19000e-2, 0, 8.20000e-3, 0, 4.80000e-3, 0, 2.00000e-3, 0, 7.00000e-4, 0, 5.00000e-4, 0, 2.00000e-4, 0, 1.00000e-4, 0, 0, 0, 0, 0, 0, 0, 0

Time elapsed: 2 ms
$ deterministic-card-game 5 -q -e
10^5 (100000) runs: 6.75000e-3, 0, 4.58800e-2, 0, 7.97800e-2, 0, 9.40600e-2, 0, 9.83600e-2, 0, 1.00690e-1, 0, 9.82500e-2, 0, 9.26800e-2, 0, 8.44700e-2, 0, 7.40100e-2, 0, 6.26000e-2, 0, 4.97000e-2, 0, 3.84800e-2, 0, 2.71300e-2, 0, 1.95100e-2, 0, 1.22900e-2, 0, 7.19000e-3, 0, 4.05000e-3, 0, 2.33000e-3, 0, 1.11000e-3, 0, 4.50000e-4, 0, 1.50000e-4, 0, 7.00000e-5, 0, 1.00000e-5, 0, 0, 0, 0, 0, 0

Time elapsed: 13 ms
$ deterministic-card-game 6 -q -e
10^6 (1000000) runs: 7.08200e-3, 0, 4.55340e-2, 0, 8.00610e-2, 0, 9.39230e-2, 0, 9.83450e-2, 0, 1.00396e-1, 0, 9.92630e-2, 0, 9.36400e-2, 0, 8.47670e-2, 0, 7.41300e-2, 0, 6.17220e-2, 0, 4.96080e-2, 0, 3.79550e-2, 0, 2.73020e-2, 0, 1.89090e-2, 0, 1.22140e-2, 0, 7.28600e-3, 0, 4.09100e-3, 0, 2.09200e-3, 0, 1.05000e-3, 0, 3.99000e-4, 0, 1.69000e-4, 0, 4.20000e-5, 0, 1.60000e-5, 0, 4.00000e-6, 0, 0, 0, 0

Time elapsed: 116 ms
$ deterministic-card-game 7 -q -e
10^7 (10000000) runs: 7.10440e-3, 0, 4.59431e-2, 0, 8.02463e-2, 0, 9.37884e-2, 0, 9.82457e-2, 0, 9.98218e-2, 0, 9.84547e-2, 0, 9.33949e-2, 0, 8.49514e-2, 0, 7.42454e-2, 0, 6.23422e-2, 0, 4.98972e-2, 0, 3.79582e-2, 0, 2.74222e-2, 0, 1.87997e-2, 0, 1.21221e-2, 0, 7.37740e-3, 0, 4.11220e-3, 0, 2.10620e-3, 0, 9.97300e-4, 0, 4.33300e-4, 0, 1.63400e-4, 0, 5.57000e-5, 0, 1.31000e-5, 0, 3.50000e-6, 0, 2.00000e-7, 0, 0

Time elapsed: 1170 ms
$ deterministic-card-game 8 -q -e
10^8 (100000000) runs: 7.07265e-3, 0, 4.59764e-2, 0, 8.02515e-2, 0, 9.38202e-2, 0, 9.82613e-2, 0, 9.98508e-2, 0, 9.85087e-2, 0, 9.32873e-2, 0, 8.49642e-2, 0, 7.42973e-2, 0, 6.22273e-2, 0, 4.97982e-2, 0, 3.79857e-2, 0, 2.75130e-2, 0, 1.88273e-2, 0, 1.21406e-2, 0, 7.32141e-3, 0, 4.10431e-3, 0, 2.12581e-3, 0, 1.00836e-3, 0, 4.24210e-4, 0, 1.61910e-4, 0, 5.31900e-5, 0, 1.45100e-5, 0, 3.36000e-6, 0, 4.50000e-7, 0, 4.00000e-8

Time elapsed: 11686 ms
```

### Number of threads

The `--threads` or `-t` option sets the number of threads to use.
If no value is given, the program defaults to running in 7 threads.
It must be used with an equals sign (e.g. `-t=8`).

It is very likely that using some small number of threads will increase the speed of the program.
The optimal number of threads will vary depending on the computer that the program is run on,
so you may want to try a few different numbers of threads along with the `--elapsed-time` flag
to see which number of threads gives the best performance on your computer.

In general, the speed of the program will roughly be multiplied by the number of threads,
up until the number of threads exceeds the number of CPU cores that your computer has
(as each CPU core is sort of its own little computer running its own games
at the same time as the other CPU cores),
as long as the computer isn't busy using its computing power for something else.
For a "normal" computer, a good number of threads might be between about 4 and 20.

Below are examples of different program run times for different numbers of threads.
On the computer in this example (which had 4 CPU cores),
adding 7 threads more than quadrupled the speed of the program.
Up to about 70 threads, no additional benefit was seen,
and above this the speed of the program started slowing again.
With more than 8000 threads, the program was slower than with only 1 thread.

```bash
$ deterministic-card-game 6 -qq -e -t=1
Time elapsed: 478 ms
$ deterministic-card-game 6 -qq -e -t=2
Time elapsed: 244 ms
$ deterministic-card-game 6 -qq -e -t=3
Time elapsed: 212 ms
$ deterministic-card-game 6 -qq -e -t=4
Time elapsed: 165 ms
$ deterministic-card-game 6 -qq -e -t=5
Time elapsed: 141 ms
$ deterministic-card-game 6 -qq -e -t=6
Time elapsed: 124 ms
$ deterministic-card-game 6 -qq -e -t=7
Time elapsed: 108 ms
$ deterministic-card-game 6 -qq -e -t=8
Time elapsed: 114 ms
$ deterministic-card-game 6 -qq -e -t=9
Time elapsed: 120 ms
$ deterministic-card-game 6 -qq -e -t=10
Time elapsed: 111 ms
$ deterministic-card-game 6 -qq -e -t=11
Time elapsed: 110 ms
$ deterministic-card-game 6 -qq -e -t=12
Time elapsed: 110 ms
$ deterministic-card-game 6 -qq -e -t=13
Time elapsed: 116 ms
$ deterministic-card-game 6 -qq -e -t=14
Time elapsed: 113 ms
$ deterministic-card-game 6 -qq -e -t=15
Time elapsed: 118 ms
$ deterministic-card-game 6 -qq -e -t=16
Time elapsed: 108 ms
$ deterministic-card-game 6 -qq -e -t=17
Time elapsed: 108 ms
$ deterministic-card-game 6 -qq -e -t=18
Time elapsed: 112 ms
$ deterministic-card-game 6 -qq -e -t=19
Time elapsed: 114 ms
$ deterministic-card-game 6 -qq -e -t=20
Time elapsed: 107 ms
$ deterministic-card-game 6 -qq -e -t=30
Time elapsed: 113 ms
$ deterministic-card-game 6 -qq -e -t=40
Time elapsed: 112 ms
$ deterministic-card-game 6 -qq -e -t=50
Time elapsed: 109 ms
$ deterministic-card-game 6 -qq -e -t=60
Time elapsed: 115 ms
$ deterministic-card-game 6 -qq -e -t=70
Time elapsed: 107 ms
$ deterministic-card-game 6 -qq -e -t=80
Time elapsed: 112 ms
$ deterministic-card-game 6 -qq -e -t=90
Time elapsed: 110 ms
$ deterministic-card-game 6 -qq -e -t=100
Time elapsed: 113 ms
$ deterministic-card-game 6 -qq -e -t=200
Time elapsed: 116 ms
$ deterministic-card-game 6 -qq -e -t=300
Time elapsed: 113 ms
$ deterministic-card-game 6 -qq -e -t=400
Time elapsed: 116 ms
$ deterministic-card-game 6 -qq -e -t=500
Time elapsed: 118 ms
$ deterministic-card-game 6 -qq -e -t=600
Time elapsed: 126 ms
$ deterministic-card-game 6 -qq -e -t=700
Time elapsed: 131 ms
$ deterministic-card-game 6 -qq -e -t=800
Time elapsed: 130 ms
$ deterministic-card-game 6 -qq -e -t=900
Time elapsed: 128 ms
$ deterministic-card-game 6 -qq -e -t=1000
Time elapsed: 131 ms
$ deterministic-card-game 6 -qq -e -t=2000
Time elapsed: 164 ms
$ deterministic-card-game 6 -qq -e -t=3000
Time elapsed: 189 ms
$ deterministic-card-game 6 -qq -e -t=4000
Time elapsed: 242 ms
$ deterministic-card-game 6 -qq -e -t=5000
Time elapsed: 279 ms
$ deterministic-card-game 6 -qq -e -t=6000
Time elapsed: 325 ms
$ deterministic-card-game 6 -qq -e -t=7000
Time elapsed: 387 ms
$ deterministic-card-game 6 -qq -e -t=8000
Time elapsed: 431 ms
$ deterministic-card-game 6 -qq -e -t=9000
Time elapsed: 486 ms
$ deterministic-card-game 6 -qq -e -t=10000
Time elapsed: 549 ms
```

### Printing the help information

The `-h` option prints the concise help information for the program.

```bash
$ deterministic-card-game -h
Run Morgan's game from episode 120 of A Problem Squared.

Usage: deterministic-card-game.exe [OPTIONS] [MAX_EXPONENT]

Arguments:
  [MAX_EXPONENT]  Exponent of 10 (10^?) for the number of runs to perform in total [default: 7]

Options:
  -s, --save-data          Save csv file of game run data
  -q, --quiet...           Print less information (only show final results / show no result at all)
  -e, --elapsed-time       Print the time elapsed during calulation
  -t, --threads=<THREADS>  Number of threads to spawn [default: 7]
  -h, --help               Print help (see more with '--help')
  -V, --version            Print version
```

The `--help` option prints a less compact and longer help.

```bash
$ deterministic-card-game --help
Run Morgan's game from episode 120 of A Problem Squared.

Usage: deterministic-card-game.exe [OPTIONS] [MAX_EXPONENT]

Arguments:
  [MAX_EXPONENT]
          Exponent of 10 (10^?) for the number of runs to perform in total

          [default: 7]

Options:
  -s, --save-data
          Save csv file of game run data

  -q, --quiet...
          Print less information (only show final results / show no result at all)

          Give once for mostly quiet, and twice for completely quiet.

          When printing results, only print results after all runs, or nothing.

          When saving data, only print that data has been saved to file, or nothing.

  -e, --elapsed-time
          Print the time elapsed during calulation

  -t, --threads=<THREADS>
          Number of threads to spawn

          [default: 7]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
