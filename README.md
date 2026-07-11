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
