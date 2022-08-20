use std::fs;
use std::io::stdin;
use rand::seq::SliceRandom;

fn main() {
    //Get random word from words.txt
    let mut attempts = 10;
    let file = fs::read_to_string("words.txt").expect("Couldn't open file.");
    let words : Vec<&str> = file.split("\n").collect();
    let mut word : &str = *words.choose(&mut rand::thread_rng()).expect("Couldnt choose word cuz why the fuck not");
    word = word.trim();
    println!("Guess the word!");
    let mut guessed_chars: Vec<char> = Vec::new();
    let mut buff : String = String::from_utf8(vec![b'#';word.len()])?;
    println!("{}", buff);
    loop {
        if attempts == 0 {
            println!("You lost! The word was {}", word);
            break;
        }
        let mut guess = String::new();
        buff = String::new();
        stdin().read_line(&mut guess).expect("Failed to read input.");
        guess = guess.trim().to_string();
        for letter in guess.chars() {
            if word.contains(letter) && !guessed_chars.contains(&letter) {
                guessed_chars.push(letter);
            } else {
                attempts -= 1;
                println!("Try again! You have {} attempts left.", attempts);
                break;
            }
        }
        for letter in word.chars() {
            buff.push(if guessed_chars.contains(&letter) { letter } else { '#' });
        }
        println!("{}", buff);
        if guess == word || !buff.contains("#") {
            println!("You win!");
            break;
        }
    }
}