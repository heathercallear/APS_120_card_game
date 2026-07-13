# Deterministic Card Game

`deterministic_card_game` provides an implementation of the card game
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

The following requires the `matplotlib` module to be installed.

```bash
$ pip install matplotlib
```

The final value reached after all the runs is used as an estimate of the true probability
of finishing a game with a certain number of cards in the hand.

The convergence of the proportion of games ending with this certain number of cards in the hand
can be shown using a log-log plot of its difference from the estimate against the number of runs.

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

To compare this convergence for won games to the appealing (and very close) value of 1/100sqrt(2),
simply run `plot_data.py` with no arguments:

```bash
$ python plot_data.py
```

![Log-log plot comparing convergence of the proportion of won games to its final result versus to 1/100sqrt(2). Until about 10^8 runs, both plots look very similar, steadily (if jaggedly) decreasing. After this, the convergence to 1/100sqrt(2) stays at about 10^-5 difference from the data. However, the convergence to the final result continues to jaggedly decrease down to roughly 10^-7 difference from the data.](/img/run-00001-plot-00-cards-left.svg)

## Usage of the data generation code

Running the program first asks if you would like to run thoroughly and save the data to a file.
If you respond with 'y' or 'yes', a new csv file will be made in a folder called 'card_game_data'
in your current working directory
(if this folder does not exist, you will be asked if it may make the folder for you).
This csv will contain run information for 10^10 runs.

Any response other than 'y' or 'yes' (such as just pressing enter) will result in run information
only being printed to the terminal (i.e. no files will be made).
This prints the proportion of times a game ends with a certain number of cards in the hand,
for an exponentially increasing number of runs.

Each successive set of runs includes all the previous ones
(e.g. to go from 100 runs to 1000 runs, the game is played 900 more times).

This format is intended to make convergence of the results more identifiable.

By default, it goes up to playing the game 10^7 times.
This can be changed using the `MAX_EXPONENT` constant in 'main.rs'.

If running using `cargo`, use the `--release` option for improved speed:

```bash
$ cargo run --release
```

For example (note that exact results will differ due to the game's randomness):

```bash
$ deterministic_card_game
Run thoroughly and save data to file? ('y' or 'yes')

10^2 (     100) runs: 2.00000e-2, 0, 2.00000e-2, 0, 5.00000e-2, 0, 9.00000e-2, 0, 1.30000e-1, 0, 6.00000e-2, 0, 1.20000e-1, 0, 1.00000e-1, 0, 1.20000e-1, 0, 1.00000e-1, 0, 6.00000e-2, 0, 2.00000e-2, 0, 6.00000e-2, 0, 1.00000e-2, 0, 3.00000e-2, 0, 1.00000e-2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0

10^3 (    1000) runs: 5.00000e-3, 0, 5.50000e-2, 0, 7.80000e-2, 0, 8.60000e-2, 0, 9.60000e-2, 0, 1.06000e-1, 0, 9.40000e-2, 0, 8.60000e-2, 0, 8.80000e-2, 0, 8.10000e-2, 0, 5.70000e-2, 0, 4.70000e-2, 0, 4.40000e-2, 0, 2.00000e-2, 0, 2.60000e-2, 0, 1.60000e-2, 0, 9.00000e-3, 0, 4.00000e-3, 0, 1.00000e-3, 0, 1.00000e-3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0

10^4 (   10000) runs: 5.30000e-3, 0, 4.65000e-2, 0, 7.81000e-2, 0, 9.23000e-2, 0, 9.64000e-2, 0, 9.93000e-2, 0, 1.00500e-1, 0, 9.62000e-2, 0, 9.14000e-2, 0, 7.74000e-2, 0, 5.80000e-2, 0, 4.68000e-2, 0, 3.94000e-2, 0, 2.52000e-2, 0, 2.06000e-2, 0, 1.25000e-2, 0, 7.90000e-3, 0, 2.70000e-3, 0, 1.60000e-3, 0, 1.20000e-3, 0, 4.00000e-4, 0, 1.00000e-4, 0, 2.00000e-4, 0, 0, 0, 0, 0, 0, 0, 0

10^5 (  100000) runs: 6.96000e-3, 0, 4.51300e-2, 0, 7.94900e-2, 0, 9.40200e-2, 0, 9.82100e-2, 0, 1.01620e-1, 0, 9.94900e-2, 0, 9.36400e-2, 0, 8.58000e-2, 0, 7.45500e-2, 0, 6.20100e-2, 0, 4.82000e-2, 0, 3.79500e-2, 0, 2.69100e-2, 0, 1.86700e-2, 0, 1.21900e-2, 0, 7.52000e-3, 0, 3.76000e-3, 0, 2.12000e-3, 0, 1.06000e-3, 0, 5.00000e-4, 0, 1.30000e-4, 0, 6.00000e-5, 0, 1.00000e-5, 0, 0, 0, 0, 0, 0

10^6 ( 1000000) runs: 6.94300e-3, 0, 4.61860e-2, 0, 8.02360e-2, 0, 9.36100e-2, 0, 9.84480e-2, 0, 1.00011e-1, 0, 9.85740e-2, 0, 9.28810e-2, 0, 8.54230e-2, 0, 7.45180e-2, 0, 6.21930e-2, 0, 4.92040e-2, 0, 3.80080e-2, 0, 2.73280e-2, 0, 1.89710e-2, 0, 1.21500e-2, 0, 7.36900e-3, 0, 4.07900e-3, 0, 2.20300e-3, 0, 9.84000e-4, 0, 4.33000e-4, 0, 1.75000e-4, 0, 5.40000e-5, 0, 1.50000e-5, 0, 4.00000e-6, 0, 0, 0, 0

10^7 (10000000) runs: 7.03180e-3, 0, 4.59674e-2, 0, 8.03140e-2, 0, 9.38149e-2, 0, 9.83423e-2, 0, 9.98679e-2, 0, 9.85041e-2, 0, 9.31323e-2, 0, 8.51668e-2, 0, 7.43117e-2, 0, 6.22310e-2, 0, 4.97440e-2, 0, 3.78407e-2, 0, 2.74555e-2, 0, 1.89134e-2, 0, 1.21236e-2, 0, 7.29460e-3, 0, 4.14590e-3, 0, 2.14730e-3, 0, 9.87000e-4, 0, 4.27300e-4, 0, 1.62700e-4, 0, 5.47000e-5, 0, 1.55000e-5, 0, 3.20000e-6, 0, 4.00000e-7, 0, 0
```

This took about 7s to run (your speed may vary).

Saving 10 billion (10^10) runs took less than 1h51m10s.
