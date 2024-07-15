mod utils;
use colored::*;
use terminal_size::terminal_size;

fn main() {
    let dictionary_count = utils::DICTIONARY.len();
    println!("Dictionary loaded with {} words", dictionary_count);
    let words_more_than_4 = utils::filter_words_by_length(&utils::DICTIONARY, (Some(4), None));
    println!("There are {} words with 4 or more letters", words_more_than_4.len());
    
    let term_width = terminal_size().map(|(w, _)| w.0 as usize).unwrap_or(64);
    println!("\n{}\n", "-".repeat(term_width).bright_white());


    let center_letter_prompt = format!("{} {} {}", "Enter the", "center".yellow().bold(), "letter: ");
    let center_letter = utils::input_letters(center_letter_prompt, Some(1), Some(true), None)[0];

    let other_letters_prompt = format!("{} {} {}", "Enter the", "other 6 letters".bright_white().bold(), ", separated by commas: ");
    let other_letters = utils::input_letters(other_letters_prompt, Some(6), Some(true), Some(vec![center_letter]));

    let term_width = terminal_size().map(|(w, _)| w.0 as usize).unwrap_or(64);
    println!("\n{}\n", "-".repeat(term_width).bright_white());


    println!("Center letter: {}", center_letter.to_string().bright_yellow().bold());
    let other_letters_str: Vec<String> = other_letters.iter().map(|c| c.to_string()).collect();
    println!("Other letters: [ {} ]", other_letters_str.join(", ").bright_white().bold());

    let mut all_letters = vec![center_letter];
    all_letters.extend(other_letters);


    // Find all words that can be made from the 7 letters
    let mut possible_words = utils::filter_words_with_any_include_letters(&words_more_than_4, &all_letters);
    println!();
    println!("> There are {} words that can be made from any of the 7 letters", possible_words.len());
    possible_words = utils::filter_words_with_all_include_letters(&possible_words, &vec![center_letter]);
    println!("> From there, there are {} possible words that contain the center letter", possible_words.len());


    // Rank the words by points
    let ranked_words = utils::rank_words(&possible_words, &all_letters);
    println!();
    utils::draw_table(&ranked_words);

    let term_width = terminal_size().map(|(w, _)| w.0 as usize).unwrap_or(64);
    println!("\n{}\n", "-".repeat(term_width).bright_white());


    // Ask the user if they want to auto-type the words
    let auto_type = utils::get_bool_input("Do you want to auto-type the words? (y/n): ".to_string());
    if !auto_type {
        let term_width = terminal_size().map(|(w, _)| w.0 as usize).unwrap_or(64);
        println!("\n{}\n", "-".repeat(term_width).bright_white());
        clearscreen::clear().expect("Failed to clear screen!");

        println!("Restarting the game...\n");
        return main();
    }


    // Optional: Auto-type the words
    println!("\nReady the game screen. After that, press Enter to continue this script, then immediately click to switch focus to the game screen.\nThe script will count down for 3 seconds and start typing the words.\n");
    println!("Press Enter to continue...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");

    // Countdown
    for i in (1..=3).rev() {
        println!("Starting in {}...", i);
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    // Auto-type the words
    let words = &ranked_words.iter().map(|(word, _)| word).collect::<Vec<&String>>();
    println!("Auto-typing the words...");
    utils::type_words(&words, &750);
    println!("Auto-typing complete.");

    let term_width = terminal_size().map(|(w, _)| w.0 as usize).unwrap_or(64);
    println!("\n{}\n", "-".repeat(term_width).bright_white());


    println!("Press Enter to restart the game, or (Ctrl+C) to exit...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");

    clearscreen::clear().expect("Failed to clear screen!");
    println!("Restarting the game...\n");
    main();
}