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
        SetForegroundColor(Color::DarkGrey),
        Print("Backspace: correct | Enter: finish"),
        ResetColor
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
        Print(format!("WPM: {wpm:.1}")),
        MoveTo(4, 8),
        SetForegroundColor(Color::DarkGrey),
        Print("Press Enter to exit"),
        ResetColor
    )?;

    stdout.flush()
}