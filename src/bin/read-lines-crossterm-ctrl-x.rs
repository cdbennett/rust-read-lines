//! Shows how you can loop and accept user input from the keyboard
//! until the user hits Ctrl+X.
//!
//! This uses the Crossterm crate to do raw terminal input.

use std::io::{stdout, Write};

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
    Result,
};

fn main() -> Result<()> {
    println!("Enter some phrases. Hit Enter after each one. Hit Ctrl+X to finish.");

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

/// Read and return a list of phrases. Type Ctrl+X to stop and return the list.
fn read_phrases() -> Result<Vec<String>> {
    let mut phrases = vec![];
    loop {
        match read_line()? {
            Some(phrase) => phrases.push(phrase),
            None => return Ok(phrases),
        }
    }
}

/// Read a single line, terminated by Enter.
/// If Ctrl+X is hit, return None. Else, return the line of text.
pub fn read_line() -> Result<Option<String>> {
    print!("Enter a phrase: ");
    stdout().flush()?;
    let _raw_mode_guard = RawMode::enable();

    let mut line = String::new();
    while let Event::Key(KeyEvent {
        code, modifiers, ..
    }) = event::read()?
    {
        match code {
            KeyCode::Enter => {
                print!("\r\n");
                return Ok(Some(line));
            }
            KeyCode::Char('x') if modifiers == KeyModifiers::CONTROL => {
                print!("\r\nYou hit Ctrl+X, done!\r\n");
                return Ok(None);
            }
            KeyCode::Char(c) => {
                // Some other character was typed, add it to the line.
                print!("{c}");
                stdout().flush()?;
                line.push(c);
            }
            _ => {}
        }
    }
    println!("\r\nGot no input.\r\n");
    Ok(None)
}

struct RawMode;

impl RawMode {
    fn enable() -> Self {
        enable_raw_mode().expect("Should be able to enable raw mode");
        Self
    }
}

impl Drop for RawMode {
    fn drop(&mut self) {
        disable_raw_mode().expect("Unable to disable raw mode")
    }
}
