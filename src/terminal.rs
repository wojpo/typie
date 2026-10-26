use std::{
    io::{self, Write},
    time::Duration,
};

use crossterm::{
    cursor::MoveTo,
    queue,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};

use crate::game_state::GameState;

fn render_header(
    stdout: &mut impl Write,
    elapsed: Duration,
) -> io::Result<()> {
    let cursor = if (elapsed.as_millis() / 500) % 2 == 0 {
        "_"
    } else {
        " "
    };

    queue!(
        stdout,
        MoveTo(4, 1),
        Clear(ClearType::CurrentLine),
        SetForegroundColor(Color::DarkYellow),
        Print(format!("Typie{cursor}")),
        ResetColor
    )
}

pub fn render_game(
    game: &GameState,
    animation_elapsed: Duration,
) -> io::Result<()> {
    let mut stdout = io::stdout();

    render_header(&mut stdout, animation_elapsed)?;

    queue!(
        stdout,
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
        MoveTo(4, 7),
        Clear(ClearType::CurrentLine),
        SetForegroundColor(Color::DarkGrey)
    )?;

    if let Some(word) = game.next_word() {
        queue!(stdout, Print(format!("Next: {word}")))?;
    }

    queue!(
        stdout,
        ResetColor,
        MoveTo(4, 9),
        Clear(ClearType::CurrentLine),
        Print("Backspace: correct | Enter: finish")
    )?;

    stdout.flush()
}

pub fn render_result(
    game: &GameState,
    duration: Duration,
    animation_elapsed: Duration,
) -> io::Result<()> {
    let mut stdout = io::stdout();

    let minutes = duration.as_secs_f64() / 60.0;
    let wpm = if minutes > 0.0 {
        (game.overall_chars_counter() as f64 / 5.0) / minutes
    } else {
        0.0
    };

    render_header(&mut stdout, animation_elapsed)?;

    queue!(
        stdout,
        MoveTo(4, 3),
        Clear(ClearType::CurrentLine),
        SetForegroundColor(Color::DarkGrey),
        Print("Results"),
        ResetColor,
        MoveTo(4, 4),
        Clear(ClearType::CurrentLine),
        Print(format!(
            "Correct: {}/{}",
            game.correct(),
            game.completed_words()
        )),
        MoveTo(4, 5),
        Clear(ClearType::CurrentLine),
        Print(format!("Time: {:.2} seconds", duration.as_secs_f64())),
        MoveTo(4, 6),
        Clear(ClearType::CurrentLine),
        Print(format!("WPM (words per minute): {wpm:.1}")),
        MoveTo(4, 9),
        Clear(ClearType::CurrentLine),
        Print("Enter: main menu | R: retry | Q: exit"),
        ResetColor
    )?;

    stdout.flush()
}

pub fn render_start_menu(
    selected: usize,
    animation_elapsed: Duration,
) -> io::Result<()> {
    let mut stdout = io::stdout();

    let options = [
        "Quick - 10 words",
        "Normal - 20 words",
        "Long - 50 words",
        "Custom",
    ];

    render_header(&mut stdout, animation_elapsed)?;

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
            Clear(ClearType::CurrentLine),
            SetForegroundColor(color),
            Print(marker),
            Print(option)
        )?;
    }

    queue!(
        stdout,
        ResetColor,
        MoveTo(4, 9),
        Clear(ClearType::CurrentLine),
        Print("Up/Down: select | Enter: confirm | Q: exit")
    )?;

    stdout.flush()
}

pub fn render_custom_menu(
    input: &str,
    error: bool,
    animation_elapsed: Duration,
) -> io::Result<()> {
    let mut stdout = io::stdout();

    render_header(&mut stdout, animation_elapsed)?;

    queue!(
        stdout,
        MoveTo(4, 3),
        Clear(ClearType::CurrentLine),
        SetForegroundColor(Color::DarkGrey),
        Print("Custom session"),
        ResetColor,
        MoveTo(4, 4),
        Clear(ClearType::CurrentLine),
        Print("Number of words (1-1000): "),
        Print(input),
        MoveTo(4, 6),
        Clear(ClearType::CurrentLine)
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
        MoveTo(4, 8),
        Clear(ClearType::CurrentLine),
        Print("Enter: start | Q: back | Backspace: delete"),
        ResetColor
    )?;

    stdout.flush()
}