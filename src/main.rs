mod game_state;
mod terminal;

use crate::terminal::{render_game, render_result};
use crossterm::cursor::{Hide, Show};
use crossterm::event::KeyEventKind;
use crossterm::style::ResetColor;
use crossterm::terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{
    event::{Event, KeyCode, read},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use game_state::GameState;
use rand::prelude::IteratorRandom;
use std::time::{Duration, Instant};
use std::{fs, io};

fn game(counter: u16) -> io::Result<(GameState, Duration)> {
    let content = fs::read_to_string("words.txt")?;
    let words: Vec<&str> = content.split_whitespace().collect();

    let mut rng = rand::rng();
    let typeword = *words.iter().choose(&mut rng).ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidData, "words.txt is empty")
    })?;

    let mut start: Option<Instant> = None;
    let mut game = GameState::new(counter, typeword);

    execute!(io::stdout(), EnterAlternateScreen, Hide, Clear(ClearType::All))?;
    render_game(&game)?;

    loop {
        match read()? {
            Event::Key(key) => {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                match key.code {
                KeyCode::Char(character) => {
                    start.get_or_insert_with(Instant::now);

                    game.type_char(character);


                    if game.is_word_complete() {
                        game.finish_word();

                        if game.is_finished() {
                            break;
                        }

                        game.start_word(*words.iter().choose(&mut rng).unwrap());
                    }


                }
                KeyCode::Backspace => {
                    if !game.typed_chars().is_empty() {
                        game.backspace();
                    }
                },
                KeyCode::Enter => break,
                _ => {}
            }
            render_game(&game)?;
        }
            Event::Resize(_, _) => {
                execute!(io::stdout(), Clear(ClearType::All))?;
                render_game(&game)?;
            }

            _ => {}
        }
    }

    let elapsed = start.map(|time| time.elapsed()).unwrap_or_default();
    Ok((game, elapsed))
}

fn run(counter: u16) -> io::Result<()> {
    let (game_result, elapsed) = game(counter)?;

    render_result(&game_result, elapsed)?;

    loop {
        match read()? {
            Event::Key(key)
            if key.kind == KeyEventKind::Press
                && key.code == KeyCode::Enter =>
                {
                    break;
                }
            Event::Resize(_, _) => {
                render_result(&game_result, elapsed)?;
            }
            _ => {}
        }
    }

    Ok(())
}
fn main() -> io::Result<()> {
    enable_raw_mode()?;

    let result = run(20);

    let screen_result = execute!(
        io::stdout(),
        ResetColor,
        Show,
        LeaveAlternateScreen
    );
    let raw_result = disable_raw_mode();

    result?;
    screen_result?;
    raw_result?;

    Ok(())
}