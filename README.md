# Number-guessing-game-using-Rust
## Description-
This Rust program implements a simple number guessing game where the user tries to guess a randomly generated secret number between 1 and 100. The program prompts the user to input their guess, provides feedback on whether the guess is too small or too big, and congratulates the user when they correctly guess the secret number. It utilizes the standard library's input/output module (io) for user input and the rand crate to generate random numbers.
## Explanation-
  1, Imports the io module from the standard library to handle input/output operations.

  2, Imports the Rng trait and gen_range function from the rand crate to generate random numbers.

  3, Generates a random number between 1 and 100 using the gen_range function.

  4, Prompts the user to guess the secret number and continuously asks for guesses until the correct number is guessed.

  5, Compares the user's guess with the secret number and provides feedback on whether the guess is too small, too big, or correct.

  6, Prints a congratulatory message when the user correctly guesses the secret number and exits the loop.
