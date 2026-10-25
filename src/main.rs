mod game_state;
mod terminal;

use crossterm::cursor::{Hide, Show};
use crossterm::event::KeyEventKind;
use crossterm::style::ResetColor;
use crossterm::{
    event::{Event, KeyCode, read},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use rand::prelude::IteratorRandom;
use std::{fs, io};
use std::time::{Duration, Instant};
use crate::terminal::{new_line, restore_char, show_typed_char, show_word};
use game_state::GameState;

fn game(counter: u16) -> io::Result<(GameState, Duration)> {
    let content = fs::read_to_string("words.txt")?;
    let words: Vec<&str> = content.split_whitespace().collect();

    let mut rng = rand::rng();
    let typeword = *words.iter().choose(&mut rng).ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidData, "words.txt is empty")
    })?;

    let mut start: Option<Instant> = None;
    let mut game = GameState::new(counter, typeword);

    execute!(io::stdout(), Hide)?;
    show_word(game.curr_word())?;

    loop {
        if let Event::Key(key) = read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }

            match key.code {
                KeyCode::Char(character) => {
                    start.get_or_insert_with(Instant::now);
                    show_typed_char(character, game.is_char_correct(character))?;

                    game.type_char(character);


                    if game.is_word_complete() {
                        game.finish_word();

                        if game.is_finished() {
                            break;
                        }

                        game.start_word(*words.iter().choose(&mut rng).unwrap());

                        show_word(game.curr_word())?;
                    }


                }
                KeyCode::Backspace => {
                    if !game.typed_chars().is_empty() {
                        game.backspace();

                        if let Some(character) = game.curr_word()
                            .chars()
                            .nth(game.typed_chars().len())
                        {
                            restore_char(character)?;
                        }
                    }
                },
                KeyCode::Enter => break,
                _ => {}
            }
        }
    }

    let elapsed = start.map(|time| time.elapsed()).unwrap_or_default();
    Ok((game, elapsed))
}
fn main() -> io::Result<()> {
    let counter = 20;
    enable_raw_mode()?;

    let result = game(counter);
    let cursor_result = execute!(io::stdout(), Show, ResetColor);
    let raw_result = disable_raw_mode();

    let (game_result, elapsed) = result?;
    cursor_result?;
    raw_result?;

    println!("\n\nCorrect: {}/{}", game_result.correct(), game_result.completed_words());
    println!("\nTime Elapsed: {:.2} seconds", elapsed.as_secs_f64());
    let minutes = elapsed.as_secs_f64() / 60.0;

    let wpm = if minutes > 0.0 {
        (game_result.overall_chars_counter() as f64 / 5.0) / minutes
    } else {
        0.0
    };

    println!("\nWPM: {:.1}", wpm);

    Ok(())
}