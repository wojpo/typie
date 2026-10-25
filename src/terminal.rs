use std::io;
use crossterm::cursor::{MoveLeft, RestorePosition, SavePosition};
use crossterm::execute;
use crossterm::style::{Color, Print, ResetColor, SetForegroundColor};

pub fn show_typed_char(character: char, correct: bool) -> io::Result<()> {
    let color = if correct {
        Color::Green
    } else { Color::Red };

    execute!(io::stdout(), SetForegroundColor(color), Print(character), ResetColor)
}

pub fn show_word(word: &str) -> io::Result<()> {
    execute!(io::stdout(), SavePosition, Print(word), RestorePosition)
}

pub fn new_line() -> io::Result<()> {
    execute!(io::stdout(), Print("\r\n"))
}

pub fn restore_char(character: char) -> io::Result<()> {
    execute!(io::stdout(), MoveLeft(1), ResetColor, Print(character), MoveLeft(1))
}