// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;
use rand::Rng;
use std::fs;
use std::io;
use std::io::Write;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "../words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    // println!("random word: {}", secret_word);

    // Your code here! :)
    println!("Welcome to CS110L Hangman!");
    let mut guessed_word: Vec<char> = vec!['-'; secret_word.len()];
    let mut guessed_letters: Vec<char> = Vec::new();
    let mut incorrect_guess_times = 0;
    loop {
        if incorrect_guess_times == NUM_INCORRECT_GUESSES {
            println!("Sorry, you ran out of guesses!");
            println!("The word was: {}", secret_word);
            break;
        }
        let mut guessed_word_complete = true;
        for i in 0..secret_word.len() {
            if guessed_word[i] == '-' {
                guessed_word_complete = false;
                break;
            }
        }
        if guessed_word_complete {
            println!("Congratulations, you guessed the word!");
            break;
        }

        print!("The word so far is: ");
        println!("{}", guessed_word.iter().collect::<String>());

        print!("You have guessed the following letters: ");
        println!("{}", guessed_letters.iter().collect::<String>());

        println!(
            "You have {} guesses left",
            NUM_INCORRECT_GUESSES - incorrect_guess_times
        );

        print!("Please guess a letter: ");
        io::stdout().flush().expect("Error flushing stdout.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading line.");

        let guessed_letter = guess.chars().next().unwrap();

        if guessed_letter.is_ascii_alphabetic() {
            if !guessed_letter.is_ascii_lowercase() {
                println!("Please enter a lowercase letter.\n");
                continue;
            }
        } else {
            println!("Please enter a letter.\n");
            continue;
        };

        let mut already_guessed = false;
        for letter in guessed_letters.iter() {
            if *letter == guessed_letter {
                already_guessed = true;
                break;
            }
        }
        if already_guessed {
            println!("You have already guessed that letter.\n");
            continue;
        }
        guessed_letters.push(guessed_letter);

        let mut correct_guess = false;
        for i in 0..secret_word.len() {
            if secret_word_chars[i] == guessed_letter {
                guessed_word[i] = guessed_letter;
                correct_guess = true;
            }
        }

        if correct_guess {
            incorrect_guess_times = 0;
        } else {
            incorrect_guess_times += 1;
            println!("Sorry, that letter is not in the word.");
        }
        println!();
    }
}
