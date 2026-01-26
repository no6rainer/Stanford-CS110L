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
const WORDS_PATH: &str = "words.txt";

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
    println!("random word: {}", secret_word);

    // Your code here! :)
    let mut remaining_guesses = NUM_INCORRECT_GUESSES;

    let mut guessed_vec = Vec::new();

    let len = secret_word_chars.len();
    let mut curr_word_chars: Vec<char> = vec!['-'; len];

    println!("Welcome to CS110L Hangman!");
    
    loop {
        println!();
        println!("The word so far is {}", curr_word_chars.iter().collect::<String>());
        println!("You have guessed the following letters: {}", guessed_vec.iter().collect::<String>());
        println!("You have {} guesses left", remaining_guesses);
        print!("Please guess a letter: ");

        io::stdout()
            .flush()
            .expect("Error flushing stdout.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading line.");

        let guess = guess.trim();

        if guess.len() != 1 {
            println!("Please enter a single letter!");
            continue;
        }

        let c = guess.chars().next().unwrap();

        if !c.is_ascii_lowercase() {
            println!("Please enter a lowercase letter!");
            continue;
        }

        if guessed_vec.contains(&c) {
            println!("You have guessed this letter!");
            continue;
        } else {
            guessed_vec.push(c);

            if secret_word_chars.contains(&c) {
                for i in 0..len {
                    if secret_word_chars[i] == c {
                        curr_word_chars[i] = c;
                    }
                }
            } else {
                remaining_guesses -= 1;
                println!("Sorry, that letter is not in the word");
            }
        }

        if curr_word_chars == secret_word_chars {
            println!();
            println!("Congratulations you guessed the secret word: {}!", secret_word);
            break;
        }

        if remaining_guesses == 0 {
            println!();
            println!("Sorry, you ran out of guesses!");
            break;
        }
    }
}
