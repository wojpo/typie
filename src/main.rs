use std::{fs, io};
use crossterm::{
    execute,
    terminal::{enable_raw_mode, disable_raw_mode},
    event::{read, Event, KeyCode},
    style::Print,
};
use crossterm::cursor::{Hide, RestorePosition, SavePosition, Show};
use crossterm::event::KeyEventKind;
use crossterm::style::{Color, ResetColor, SetForegroundColor};
use rand::prelude::IteratorRandom;

fn main() -> io::Result<()> {

    let content = fs::read_to_string("words.txt")?;
    let words: Vec<&str> = content.split_whitespace().collect();

    let mut rng = rand::rng();
    let mut typeword = *words.iter().choose(&mut rng).ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidData, "words.txt is empty")
    })?;
    let mut chars: Vec<char> = typeword.chars().collect();


    let mut i = 0;

    enable_raw_mode()?;
    execute!(io::stdout(), Hide)?;
    execute!(io::stdout(), SavePosition, Print(typeword), RestorePosition)?;

    loop {
        if let Event::Key(key) = read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }

            match key.code {
                KeyCode::Char(character) => {
                    let color = if character == chars[i] {
                        Color::Green
                    } else { Color::Red };

                    execute!(io::stdout(), SetForegroundColor(color), Print(chars[i]), ResetColor)?;
                    i += 1;
                    if i == chars.len() {
                        typeword = *words.iter().choose(&mut rng).unwrap();
                        chars = typeword.chars().collect();
                        i = 0;
                        execute!(io::stdout(), Print("\n"), SavePosition, Print(typeword), RestorePosition)?;
                    }

                }
                KeyCode::Enter => break,
                _ => {}
            }
        }
    }

    execute!(io::stdout(), Show, ResetColor)?;
    disable_raw_mode()?;
    Ok(())
}