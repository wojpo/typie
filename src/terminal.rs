use crossterm::cursor::MoveTo;
use crossterm::style::{Color, Print, ResetColor, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType};
use crossterm::queue;
use std::io;
use std::io::Write;
use std::time::Duration;

use crate::game_state::GameState;

pub fn render_game(game: &GameState) -> io::Result<()> {
    let mut stdout = io::stdout();

    queue!(
        stdout,
        MoveTo(4, 1),
        SetForegroundColor(Color::DarkYellow),
        Print("Typie"),
        ResetColor,
        MoveTo(4, 3),
        Clear(ClearType::CurrentLine),
        SetForegroundColor(Color::DarkGrey),
        Print(format!("Completed: {}", game.completed_words())),
        ResetColor,
        MoveTo(4, 5),
        Clear(ClearType::CurrentLine)
    )?;

    for (index, expected) in game.curr_word().chars().enumerate() {
        let (character, color) = match game.typed_chars().get(index) {
            Some(&typed) if typed == expected => (typed, Color::Green),
            Some(&typed) => (typed, Color::Red),
            None => (expected, Color::White),
        };

        queue!(
            stdout,
            SetForegroundColor(color),
            Print(character)
        )?;
    }

    queue!(
        stdout,
        ResetColor,
        MoveTo(4, 9),
        Print("Backspace: correct | Enter: finish"),
    )?;

    queue!(
    stdout,
    MoveTo(4, 7),
    Clear(ClearType::CurrentLine),
    )?;

    if let Some(word) = game.next_word() {
        queue!(stdout, Print(format!("Next: {word}")))?;
    }

    queue!(stdout, ResetColor)?;

    stdout.flush()
}

pub fn render_result(game: &GameState, duration: Duration) -> io::Result<()> {
    let mut stdout = io::stdout();

    let minutes = duration.as_secs_f64() / 60.0;
    let wpm = if minutes > 0.0 {
        (game.overall_chars_counter() as f64 / 5.0) / minutes
    } else {
        0.0
    };

    queue!(
        stdout,
        Clear(ClearType::All),
        MoveTo(4, 1),
        SetForegroundColor(Color::DarkYellow),
        Print("Typie - Results"),
        ResetColor,
        MoveTo(4, 3),
        Print(format!(
            "Correct: {}/{}",
            game.correct(),
            game.completed_words()
        )),
        MoveTo(4, 4),
        Print(format!("Time: {:.2} seconds", duration.as_secs_f64())),
        MoveTo(4, 5),
        Print(format!("WPM (words per minute): {wpm:.1}")),
        MoveTo(4, 8),
        Print("Enter: main menu | R: retry | Q: exit"),
        ResetColor
    )?;

    stdout.flush()
}

pub fn render_start_menu(selected: usize) -> io::Result<()> {
    let mut stdout = io::stdout();
    let options = [
        "Quick - 10 words",
        "Normal - 20 words",
        "Long - 50 words",
        "Custom",
    ];

    queue!(
        stdout,
        Clear(ClearType::All),
        MoveTo(4, 1),
        SetForegroundColor(Color::DarkYellow),
        Print("Typie - Start menu"),
        ResetColor
    )?;

    for (index, option) in options.iter().enumerate() {
        let color = if index == selected {
            Color::Yellow
        } else {
            Color::DarkGrey
        };

        let marker = if index == selected { "> " } else { "  " };

        queue!(
            stdout,
            MoveTo(4, 3 + index as u16),
            SetForegroundColor(color),
            Print(marker),
            Print(option)
        )?;
    }

    queue!(
        stdout,
        ResetColor,
        MoveTo(4, 9),
        Print("Up/Down: select | Enter: confirm | Q: exit")
    )?;

    stdout.flush()
}

pub fn render_custom_menu(input: &str, error: bool) -> io::Result<()> {
    let mut stdout = io::stdout();

    queue!(
        stdout,
        Clear(ClearType::All),
        MoveTo(4, 1),
        SetForegroundColor(Color::DarkYellow),
        Print("Typie - Custom session"),
        ResetColor,
        MoveTo(4, 3),
        Print("Number of words (1-1000): "),
        Print(input),
        MoveTo(4, 5)
    )?;

    if error {
        queue!(
            stdout,
            SetForegroundColor(Color::Red),
            Print("Enter a number between 1 and 1000."),
            ResetColor
        )?;
    }

    queue!(
        stdout,
        MoveTo(4, 7),
        Print("Enter: start | Q: back | Backspace: delete"),
        ResetColor
    )?;

    stdout.flush()
}