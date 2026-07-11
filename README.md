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
- If the number of cards in the hand is now less than 4, add cards to it until it has 4 cards

### Action each turn

To advance the game by one turn:

- Add a card to the hand (removed from the top of the deck)
- Follow the instructions for how to remove cards from the hand

### When the game finishes

The game finishes when all cards in the deck have been dealt (the deck is empty).

The aim of the game is to have no cards remaining in the hand.

## Basic Usage

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
10^2 (       100) runs: 0, 0, 0, 0, 2.00000e-2, 0, 5.00000e-2, 0, 4.00000e-2, 0, 9.00000e-2, 0, 5.00000e-2, 0, 1.00000e-1, 0, 8.00000e-2, 0, 1.60000e-1, 0, 9.00000e-2, 0, 7.00000e-2, 0, 1.10000e-1, 0, 5.00000e-2, 0, 3.00000e-2, 0, 2.00000e-2, 0, 1.00000e-2, 0, 1.00000e-2, 0, 1.00000e-2, 0, 0, 0, 0, 0, 0, 0, 1.00000e-2, 0, 0, 0, 0, 0, 0, 0, 0

10^3 (      1000) runs: 2.00000e-3, 0, 4.00000e-3, 0, 1.70000e-2, 0, 3.30000e-2, 0, 4.30000e-2, 0, 7.00000e-2, 0, 7.30000e-2, 0, 8.90000e-2, 0, 9.20000e-2, 0, 1.10000e-1, 0, 9.30000e-2, 0, 8.00000e-2, 0, 7.70000e-2, 0, 6.00000e-2, 0, 5.60000e-2, 0, 5.00000e-2, 0, 1.80000e-2, 0, 1.90000e-2, 0, 8.00000e-3, 0, 2.00000e-3, 0, 3.00000e-3, 0, 0, 0, 1.00000e-3, 0, 0, 0, 0, 0, 0, 0, 0

10^4 (     10000) runs: 1.60000e-3, 0, 5.30000e-3, 0, 1.42000e-2, 0, 2.93000e-2, 0, 3.86000e-2, 0, 5.67000e-2, 0, 7.18000e-2, 0, 8.94000e-2, 0, 9.78000e-2, 0, 9.84000e-2, 0, 1.00700e-1, 0, 9.62000e-2, 0, 8.40000e-2, 0, 6.84000e-2, 0, 5.57000e-2, 0, 3.98000e-2, 0, 2.39000e-2, 0, 1.65000e-2, 0, 6.80000e-3, 0, 2.50000e-3, 0, 1.30000e-3, 0, 9.00000e-4, 0, 2.00000e-4, 0, 0, 0, 0, 0, 0, 0, 0

10^5 (    100000) runs: 1.03000e-3, 0, 6.58000e-3, 0, 1.32300e-2, 0, 2.84700e-2, 0, 4.21000e-2, 0, 5.66400e-2, 0, 7.19600e-2, 0, 8.72100e-2, 0, 9.82400e-2, 0, 1.03040e-1, 0, 1.03550e-1, 0, 9.61100e-2, 0, 8.47700e-2, 0, 6.95500e-2, 0, 5.25500e-2, 0, 3.64700e-2, 0, 2.28100e-2, 0, 1.39600e-2, 0, 6.52000e-3, 0, 3.13000e-3, 0, 1.33000e-3, 0, 5.90000e-4, 0, 1.20000e-4, 0, 4.00000e-5, 0, 0, 0, 0, 0, 0

10^6 (   1000000) runs: 9.66000e-4, 0, 7.07400e-3, 0, 1.34870e-2, 0, 2.84120e-2, 0, 4.16100e-2, 0, 5.65220e-2, 0, 7.18850e-2, 0, 8.72930e-2, 0, 9.72930e-2, 0, 1.03996e-1, 0, 1.03566e-1, 0, 9.67480e-2, 0, 8.46940e-2, 0, 6.89890e-2, 0, 5.24030e-2, 0, 3.61980e-2, 0, 2.30770e-2, 0, 1.37340e-2, 0, 6.92900e-3, 0, 3.20000e-3, 0, 1.28000e-3, 0, 4.88000e-4, 0, 1.18000e-4, 0, 3.30000e-5, 0, 5.00000e-6, 0, 0, 0, 0

10^7 (  10000000) runs: 9.40400e-4, 0, 7.04450e-3, 0, 1.35783e-2, 0, 2.83112e-2, 0, 4.13866e-2, 0, 5.64600e-2, 0, 7.21374e-2, 0, 8.66873e-2, 0, 9.77232e-2, 0, 1.04022e-1, 0, 1.03709e-1, 0, 9.69468e-2, 0, 8.49326e-2, 0, 6.89726e-2, 0, 5.20880e-2, 0, 3.63071e-2, 0, 2.31363e-2, 0, 1.34496e-2, 0, 7.00720e-3, 0, 3.20520e-3, 0, 1.31750e-3, 0, 4.65400e-4, 0, 1.33500e-4, 0, 3.21000e-5, 0, 6.00000e-6, 0, 3.00000e-7, 0, 0

10^8 ( 100000000) runs: 9.47950e-4, 0, 7.04261e-3, 0, 1.36269e-2, 0, 2.83571e-2, 0, 4.14369e-2, 0, 5.64030e-2, 0, 7.21664e-2, 0, 8.66457e-2, 0, 9.78197e-2, 0, 1.04025e-1, 0, 1.03692e-1, 0, 9.69704e-2, 0, 8.47420e-2, 0, 6.89883e-2, 0, 5.20898e-2, 0, 3.62807e-2, 0, 2.31280e-2, 0, 1.34221e-2, 0, 7.00240e-3, 0, 3.24877e-3, 0, 1.32622e-3, 0, 4.64910e-4, 0, 1.34640e-4, 0, 3.17500e-5, 0, 5.85000e-6, 0, 7.20000e-7, 0, 3.00000e-8

10^9 (1000000000) runs: 9.46018e-4, 0, 7.04336e-3, 0, 1.36315e-2, 0, 2.83435e-2, 0, 4.14494e-2, 0, 5.64143e-2, 0, 7.21469e-2, 0, 8.66543e-2, 0, 9.78393e-2, 0, 1.03947e-1, 0, 1.03699e-1, 0, 9.69608e-2, 0, 8.47761e-2, 0, 6.90048e-2, 0, 5.20864e-2, 0, 3.62732e-2, 0, 2.31305e-2, 0, 1.34215e-2, 0, 7.01068e-3, 0, 3.25574e-3, 0, 1.32810e-3, 0, 4.63906e-4, 0, 1.35650e-4, 0, 3.17900e-5, 0, 5.59100e-6, 0, 6.75000e-7, 0, 4.60000e-8
```
