use std::collections::HashSet;
use std::io::{stdin, stdout, Write};

use colored::*;
use lazy_static::lazy_static;
use rayon::prelude::*;
use terminal_size::terminal_size;
use enigo::{
    Direction::{Press, Release},
    Enigo, Key, Keyboard, Settings,
};


lazy_static! {
    pub static ref DICTIONARY: HashSet<String> = {
        let file = std::include_str!("./assets/words_alpha.txt");
        let dictionary: HashSet<String> = file.lines().map(|line| line.to_string()).collect();
        dictionary
    };
}

/// Get a list of letters (comma, space separated, or letters in sequence) from the user.
///
/// ### Arguments
///
/// * `prompt` - The prompt to display to the user
/// * `length` - The number of letters to get from the user
/// * `must_be_unique` - If true, the letters must be unique
/// * `letters_not_in` - A list of letters that should not be included in the input
pub fn input_letters(
    prompt: String,
    length: Option<u32>,
    must_be_unique: Option<bool>,
    letters_not_in: Option<Vec<char>>,
) -> Vec<char> {
    loop {
        print!("{}", prompt);
        stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read input");

        input = input.trim().to_string();

        // Retain only alphabetic characters, convert to lowercase, and collect into a Vec<char>
        let letters: Vec<char> = input.chars().filter(|c| c.is_alphabetic()).collect();
        let mut letters: Vec<char> = letters
            .iter()
            .map(|c| c.to_lowercase().next().unwrap())
            .collect();

        // Exclude letters specified in letters_not_in
        if let Some(ref exclude_letters) = letters_not_in {
            letters.retain(|c| !exclude_letters.contains(c));
        }

        // If must_be_unique is true, remove duplicate letters
        if must_be_unique.unwrap_or(false) {
            let mut unique_letters = Vec::new();
            for letter in letters.into_iter() {
                if !unique_letters.contains(&letter) {
                    unique_letters.push(letter);
                }
            }
            letters = unique_letters;
        }

        // Handle matching the required length
        match length {
            Some(len) if letters.len() as u32 == len => return letters,
            None => return letters,
            _ => {
                let unique_msg = if must_be_unique.unwrap_or(false) {
                    " unique".cyan()
                } else {
                    "".normal()
                };

                let exclude_msg = if let Some(ref exclude_letters) = letters_not_in {
                    // join the letters in exclude_letters with a comma separator
                    let exclude_letters: Vec<String> = exclude_letters.iter().map(|c| c.to_string()).collect();
                    let exclude_letters = exclude_letters.join(", ");
                    format!("excluding [ {} ]", exclude_letters.red().bold())
                } else {
                    "".to_string()
                };

                let warn = format!(
                    "{} {}{} {}{}",
                    "Please enter exactly".yellow(),
                    length.unwrap_or(0).to_string().green().bold(),
                    unique_msg,
                    "letters in the English alphabet ".yellow(),
                    exclude_msg.underline()
                );
                println!("{}", warn);
            }
        }
    }
}


pub fn get_bool_input(prompt: String) -> bool {
    loop {
        print!("{}", prompt);
        stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read input");

        input = input.trim().to_string();

        match input.to_lowercase().as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => {
                println!("Please enter 'yes' or 'no'");
            }
        }
    }
}


/// Filter from the Dictionary for words within word_length range
pub fn filter_words_by_length(words: &HashSet<String>, word_length: (Option<usize>, Option<usize>)) -> HashSet<String> {
    // Check if both bounds are None and return the words as is
    if word_length.0.is_none() && word_length.1.is_none() {
        return words.clone();
    }

    // Filter the words based on the length range
    words
        .par_iter()
        .filter(|word| {
            let len = word.len();
            let lower_bound_met = word_length.0.map_or(true, |min| len >= min);
            let upper_bound_met = word_length.1.map_or(true, |max| len <= max);
            lower_bound_met && upper_bound_met
        })
        .cloned()
        .collect()
}


/// Filter from the Dictionary for words that only uses the provided letters (duplicate allowed)
pub fn filter_words_with_any_include_letters(words: &HashSet<String>, letters: &Vec<char>) -> HashSet<String> {
    // Filter the words based on the letters provided
    words
        .par_iter()
        .filter(|word| {
            let mut word_letters = word.chars().collect::<Vec<char>>();
            word_letters.retain(|c| letters.contains(c));
            word_letters.len() == word.len()
        })
        .cloned()
        .collect()
}

/// Filter from the Dictionary for words that must include all of the provided letters (duplicate allowed)
pub fn filter_words_with_all_include_letters(words: &HashSet<String>, letters: &Vec<char>) -> HashSet<String> {
    words
        .par_iter()
        .filter(|word| {
            let mut word_letters = word.chars().collect::<Vec<char>>();
            letters.iter().all(|c| {
                if let Some(pos) = word_letters.iter().position(|x| x == c) {
                    word_letters.remove(pos);
                    true
                } else {
                    false
                }
            })
        })
        .cloned()
        .collect()
}

/// Rank the words by points based on the letters used
/// Returns a Vec of tuples containing the word and its points:
/// ### Points calculation:
/// - 4-letters word (minimum length) = 1 point
/// - If the word is longer than 4 letters, every letter = 1 point
/// - 7 extra points if all 7 letters are used
/// 
pub fn rank_words(words: &HashSet<String>, letters: &Vec<char>) -> Vec<(String, u32)> {
    let mut ranked_words: Vec<(String, u32)> = words
        .par_iter()
        .map(|word| {
            let mut word_letters = word.chars().collect::<Vec<char>>();
            let mut points:u32;

            // Check if all letters are used
            let mut all_letters_used = true;
            for letter in letters.iter() {
                if let Some(pos) = word_letters.iter().position(|x| x == letter) {
                    word_letters.remove(pos);
                } else {
                    all_letters_used = false;
                    break;
                }
            }

            // Calculate points
            if word.len() == 4 {
                points = 1;
            } else {
                points = word.len() as u32;
            }

            if all_letters_used {
                points += 7;
            }

            (word.clone(), points)
        })
        .collect();

    // Sort the ranked words by points in descending order
    ranked_words.sort_by(|a, b| b.1.cmp(&a.1));

    ranked_words
}


/// Draw a Table of the ranked words with two columns: Word and Points
pub fn draw_table(ranked_words: &Vec<(String, u32)>) {
    // Find the longest word length
    let longest_word_length = ranked_words.iter().map(|(word, _)| word.len()).max().unwrap_or(0);

    // Table Header
    println!(
        "{:<width$} | Points",
        "Word",
        width = longest_word_length
    );
    println!(
        "{:-<width$} | {:-<points_width$}",
        "",
        "",
        width = longest_word_length,
        points_width = 6
    );

    // Table Rows
    for (word, points) in ranked_words.iter() {
        println!("{:<width$} | {:>6}", word, points, width = longest_word_length);
    }

    // Footer?
    println!(
        "{:-<width$} | {:-<points_width$}",
        "",
        "",
        width = longest_word_length,
        points_width = 6
    );

    println!("Total words: {}", ranked_words.len());
    let total_points: u32 = ranked_words.iter().map(|(_, points)| points).sum();
    println!("Max possible points: {}", total_points);

    println!("\n{}\n", "** Some of the words here may not exist in the Spelling Bee game dictionary **".italic());
}


/// Type the words in the list with a delay between each word
pub fn type_words(word: &Vec<&String>, delay_ms: &u64) {
    let word_count = word.len();
    println!();
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    for (i, word) in word.iter().enumerate() {
        // Progress
        let term_width = terminal_size().map(|(w, _)| w.0 as usize).unwrap_or(64);
        let progress = format!("[{}/{}] Typing word: {}", i+1, word_count, word.green());
        print!("\r{}{}", progress, " ".repeat(term_width - progress.len()));
        let _ = stdout().flush();

        // Type the word
        for c in word.chars() {
            // Key down, then key up
            let _ = enigo.key(Key::Unicode(c), Press);
            std::thread::sleep(std::time::Duration::from_millis(10));
            let _ = enigo.key(Key::Unicode(c), Release);
            std::thread::sleep(std::time::Duration::from_millis(10));
        }

        // Press Enter to submit the word
        let _ = enigo.key(Key::Return, Press);
        std::thread::sleep(std::time::Duration::from_millis(25));
        let _ = enigo.key(Key::Return, Release);
        std::thread::sleep(std::time::Duration::from_millis(25));
        // Repeat just in case the first Return key press didn't register
        let _ = enigo.key(Key::Return, Press);
        std::thread::sleep(std::time::Duration::from_millis(25));
        let _ = enigo.key(Key::Return, Release);
        std::thread::sleep(std::time::Duration::from_millis(*delay_ms));
    }
    println!(); // Make sure the previous line is cleared
}