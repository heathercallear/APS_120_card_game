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

Use `DataPlotter` in `plot_data.py` to view the convergence of the proportion of games
finished with a certain number of cards left in the hand to its final value.

To compare this convergence for won games to the appealing (and very close) value of 1/100sqrt(2), simply run `plot_data.py`:

```bash
$ python plot_data.py
```

![Log-log plot comparing convergence of the proportion of won games to its final result versus to 1/100sqrt(2). Until about 10^8 runs, both plots look very similar, steadily (if jaggedly) decreasing. After this, the convergence to 1/100sqrt(2) stays at about 10^-5 difference from the data. However, the convergence to the final result continues to jaggedly decrease down to roughly 10^-7 difference from the data.](/img/run-00001-plot-00-cards-left.svg)

To see the convergence for any final number of cards, use `DataPlotter.show_plot`.

For example:

```pycon
>>> from plot_data import DataPlotter
>>> # read latest run data from its csv
>>> dp = DataPlotter()
>>> # show convergence for 4 cards left in the hand
>>> dp.show_plot(4)
```

The above code opens a new window with the plot.

## Usage of the data generation code

Running the program prints the proportion of times a game ends with a certain number of cards in the hand,
for an exponentially increasing number of runs.

Each successive set of runs includes all the previous ones
(eg. to go from 100 runs to 1000 runs, the game is played 900 more times).

This format is intended to make convergence of the results more identifiable.

By default, it goes up to playing the game 10^9 times.
This can be changed using the `MAX_EXPONENT` constant in 'main.rs'.

If running using `cargo`, use the `--release` option for improved speed:

```bash
$ cargo run --release
```

For example (note that exact results will differ due to the game's randomness):

```bash
$ deterministic_card_game
10^2 (       100) runs: 0, 0, 2.00000e-2, 0, 7.00000e-2, 0, 1.90000e-1, 0, 8.00000e-2, 0, 1.10000e-1, 0, 1.00000e-1, 0, 1.00000e-1, 0, 9.00000e-2, 0, 5.00000e-2, 0, 8.00000e-2, 0, 5.00000e-2, 0, 2.00000e-2, 0, 2.00000e-2, 0, 1.00000e-2, 0, 1.00000e-2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0

10^3 (      1000) runs: 8.00000e-3, 0, 4.60000e-2, 0, 8.40000e-2, 0, 1.10000e-1, 0, 8.10000e-2, 0, 1.04000e-1, 0, 1.09000e-1, 0, 9.40000e-2, 0, 7.60000e-2, 0, 8.10000e-2, 0, 5.90000e-2, 0, 4.80000e-2, 0, 3.60000e-2, 0, 2.20000e-2, 0, 1.60000e-2, 0, 1.30000e-2, 0, 7.00000e-3, 0, 3.00000e-3, 0, 2.00000e-3, 0, 1.00000e-3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0

10^4 (     10000) runs: 7.00000e-3, 0, 4.47000e-2, 0, 8.69000e-2, 0, 9.33000e-2, 0, 9.55000e-2, 0, 9.65000e-2, 0, 1.04400e-1, 0, 9.31000e-2, 0, 8.52000e-2, 0, 7.51000e-2, 0, 6.32000e-2, 0, 4.95000e-2, 0, 3.42000e-2, 0, 2.52000e-2, 0, 1.82000e-2, 0, 1.16000e-2, 0, 6.70000e-3, 0, 5.40000e-3, 0, 1.60000e-3, 0, 1.30000e-3, 0, 9.00000e-4, 0, 4.00000e-4, 0, 1.00000e-4, 0, 0, 0, 0, 0, 0, 0, 0

10^5 (    100000) runs: 7.41000e-3, 0, 4.61800e-2, 0, 8.13500e-2, 0, 9.31500e-2, 0, 9.64900e-2, 0, 9.86000e-2, 0, 9.81000e-2, 0, 9.44100e-2, 0, 8.54500e-2, 0, 7.56000e-2, 0, 6.21200e-2, 0, 5.02600e-2, 0, 3.75500e-2, 0, 2.68100e-2, 0, 1.87100e-2, 0, 1.20200e-2, 0, 7.66000e-3, 0, 4.18000e-3, 0, 2.26000e-3, 0, 1.05000e-3, 0, 4.30000e-4, 0, 1.60000e-4, 0, 4.00000e-5, 0, 0, 0, 1.00000e-5, 0, 0, 0, 0

10^6 (   1000000) runs: 7.10400e-3, 0, 4.60100e-2, 0, 8.03720e-2, 0, 9.39170e-2, 0, 9.79690e-2, 0, 1.00276e-1, 0, 9.82660e-2, 0, 9.30480e-2, 0, 8.46310e-2, 0, 7.41580e-2, 0, 6.24310e-2, 0, 5.00690e-2, 0, 3.80800e-2, 0, 2.75190e-2, 0, 1.85690e-2, 0, 1.22970e-2, 0, 7.36800e-3, 0, 4.07200e-3, 0, 2.15100e-3, 0, 1.02300e-3, 0, 4.29000e-4, 0, 1.72000e-4, 0, 5.70000e-5, 0, 9.00000e-6, 0, 3.00000e-6, 0, 0, 0, 0

10^7 (  10000000) runs: 7.06770e-3, 0, 4.59064e-2, 0, 8.02432e-2, 0, 9.37978e-2, 0, 9.81959e-2, 0, 1.00044e-1, 0, 9.85035e-2, 0, 9.33813e-2, 0, 8.48418e-2, 0, 7.43178e-2, 0, 6.22366e-2, 0, 4.99325e-2, 0, 3.80264e-2, 0, 2.74640e-2, 0, 1.87287e-2, 0, 1.21715e-2, 0, 7.30580e-3, 0, 4.06890e-3, 0, 2.10500e-3, 0, 1.00590e-3, 0, 4.21600e-4, 0, 1.63900e-4, 0, 5.49000e-5, 0, 1.19000e-5, 0, 2.90000e-6, 0, 1.00000e-7, 0, 0

10^8 ( 100000000) runs: 7.06264e-3, 0, 4.59574e-2, 0, 8.02609e-2, 0, 9.38420e-2, 0, 9.81936e-2, 0, 9.98603e-2, 0, 9.85144e-2, 0, 9.33372e-2, 0, 8.49269e-2, 0, 7.43250e-2, 0, 6.22665e-2, 0, 4.98010e-2, 0, 3.79805e-2, 0, 2.74727e-2, 0, 1.88435e-2, 0, 1.21476e-2, 0, 7.31589e-3, 0, 4.10188e-3, 0, 2.12376e-3, 0, 1.01067e-3, 0, 4.22980e-4, 0, 1.61630e-4, 0, 5.31100e-5, 0, 1.43100e-5, 0, 3.32000e-6, 0, 4.70000e-7, 0, 4.00000e-8

10^9 (1000000000) runs: 7.06459e-3, 0, 4.59787e-2, 0, 8.02625e-2, 0, 9.38146e-2, 0, 9.82023e-2, 0, 9.98707e-2, 0, 9.85444e-2, 0, 9.33136e-2, 0, 8.49548e-2, 0, 7.42945e-2, 0, 6.22337e-2, 0, 4.98085e-2, 0, 3.79704e-2, 0, 2.75018e-2, 0, 1.88283e-2, 0, 1.21405e-2, 0, 7.32155e-3, 0, 4.10572e-3, 0, 2.12539e-3, 0, 1.00545e-3, 0, 4.26104e-4, 0, 1.61514e-4, 0, 5.25480e-5, 0, 1.42910e-5, 0, 3.03600e-6, 0, 4.89000e-7, 0, 3.70000e-8
```

This took a little under 12 minutes to run (your speed may vary).

Saving 10 billion (10^10) runs took less than 1h51m10s.
