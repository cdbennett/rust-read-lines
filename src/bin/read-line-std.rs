//! Shows how you can loop and accept user input from the keyboard
//! until the user Enter alone to type an empty line.
//!
//! This does not use any 3rd party crates, just the Rust std library.

use std::io::{stdin, stdout, Write};

fn main() -> std::io::Result<()> {
    println!("Enter some phrases. Hit Enter after each one. Hit Enter alone to finish.");

    let phrases = read_phrases();
    match phrases {
        Ok(phrases) => show_phrases(phrases),
        Err(e) => println!("Error: {:?}\r", e),
    }

    Ok(())
}

fn show_phrases(phrases: Vec<String>) {
    println!("Here are the phrases you entered:");
    for phrase in phrases {
        println!(">>> {phrase} <<<");
    }
}

/// Read and return a list of phrases.
fn read_phrases() -> std::io::Result<Vec<String>> {
    let mut phrases = vec![];
    loop {
        match read_line()? {
            Some(phrase) => phrases.push(phrase),
            None => return Ok(phrases),
        }
    }
}

/// Read a single line, terminated by Enter.
/// If Enter alone is hit, return None. Else, return the line of text.
pub fn read_line() -> std::io::Result<Option<String>> {
    print!("Enter a phrase: ");
    stdout().flush()?;

    let mut line = String::new();
    let _ = stdin().read_line(&mut line)?;
    let line = line.trim_end_matches("\r");
    let line = line.trim_end_matches("\n");
    if line.is_empty() {
        Ok(None)
    } else {
        Ok(Some(line.to_string()))
    }
}
