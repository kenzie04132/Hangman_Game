use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // text data setup
    
    // a list of possible secret words for the game to choose from
    let word_bank = ["strawberry", "vehicle", "ladybug", "sunflower", "astronaut", "discombobulated", "independence"];
    
    // fetches the current computer system time to generate a random index number
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis();
        
    // uses the math remainder symbol (%) to pick a random position in our word list
    let random_index = (current_time as usize) % word_bank.len();
    
    // the word chosen for this specific round of the game
    let secret_word = word_bank[random_index];
    
    // changing tracking variables that update as the game plays
    let mut guessed_letters: Vec<char> = Vec::new();
    let mut mistakes = 0;
    let max_mistakes = 6;

    println!("Welcome to the game of Hangman!");
    println!("Try to guess the secret word by guessing one letter at a time.");

    // main loop that runs the game turns over and over
    loop {
        println!("\n--------------------------------");
        
        // calls a helper function to print the hidden word blanks
        display_word(secret_word, &guessed_letters);
        println!("Mistakes: {}/{}", mistakes, max_mistakes);

        // checks if the player has won the game
        if has_won(secret_word, &guessed_letters) {
            println!("\n🎉 Congratulations! You guessed the word: '{}'!", secret_word);
            break;
        }

        // checks if the player has run out of turns
        if mistakes >= max_mistakes {
            println!("\nGame Over! You ran out of guesses.");
            println!("The secret word was: '{}'", secret_word);
            break;
        }

        // calls the keyboard function to get a letter from the player
        let guess = get_player_guess();

        // handles repeated guesses
        if guessed_letters.contains(&guess) {
            println!("You already guessed '{}'!", guess);
            continue;
        }

        // adds the new guess into the tracking list
        guessed_letters.push(guess);

        // checks if the guessed letter is a match or a mistake
        if secret_word.contains(guess) {
            println!("Good job! '{}' is in the word.", guess);
        } else {
            println!("Sorry! '{}' is not in the word.", guess);
            mistakes += 1; // adds 1 to the mistake counter
        }
    }
}


// screen display tools

// helper function that prints letters for correct guesses and underscores for hidden letters
fn display_word(word: &str, guesses: &[char]) {
    print!("Word: ");
    for c in word.chars() {
        if guesses.contains(&c) {
            print!("{} ", c);
        } else {
            print!("_ ");
        }
    }
    println!();
}

// win checker tool

// helper function that checks if every letter in the secret word matches the guessed list
fn has_won(word: &str, guesses: &[char]) -> bool {
    word.chars().all(|c| guesses.contains(&c))
}


// user input systems

// helper function that reads keyboard text and makes sure it is a single valid letter
fn get_player_guess() -> char {
    loop {
        print!("Guess a letter: ");
        io::stdout().flush().expect("Failed to flush stdout");

        // builds an empty text container to catch the keyboard input
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // cleans off spacing and converts upper case guesses to lower case
        let trimmed = input.trim().to_lowercase();
        
        // looks at the cleaned text and makes sure it is exactly 1 letter long
        match trimmed.chars().next() {
            Some(c) if c.is_alphabetic() && trimmed.len() == 1 => return c,
            _ => {
                println!("Invalid input. Please type only one letter.");
                continue;
            }
        };
    }
}