mod game_state;
mod terminal;

use crate::terminal::{render_custom_menu, render_game, render_result, render_start_menu};
use crossterm::cursor::{Hide, Show};
use crossterm::event::{KeyEventKind, poll};
use crossterm::style::ResetColor;
use crossterm::terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{
    event::{Event, KeyCode, read},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use game_state::GameState;
use rand::prelude::IteratorRandom;
use std::io;
use std::time::{Duration, Instant};

fn game(counter: u16, animation_start: &Instant) -> io::Result<(GameState, Duration)> {
    let content = include_str!("../words.txt");
    let words: Vec<&str> = content.split_whitespace().collect();

    let mut rng = rand::rng();
    let typeword = *words
        .iter()
        .choose(&mut rng)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "words.txt is empty"))?;

    let mut start: Option<Instant> = None;
    let mut game = GameState::new(counter, typeword);
    game.enqueue_word(*words.iter().choose(&mut rng).unwrap());

    execute!(io::stdout(), Clear(ClearType::All))?;

    loop {
        render_game(&game, animation_start.elapsed())?;

        if !poll(Duration::from_millis(50))? {
            continue;
        }

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
                    }
                    KeyCode::Enter => break,
                    _ => {}
                }
            }
            Event::Resize(_, _) => {
                execute!(io::stdout(), Clear(ClearType::All))?;
            }

            _ => {}
        }
    }

    let elapsed = start.map(|time| time.elapsed()).unwrap_or_default();
    Ok((game, elapsed))
}

fn run(counter: u16, animation_start: &Instant) -> io::Result<bool> {
    loop {
        let (game_result, elapsed) = game(counter, animation_start)?;
        execute!(io::stdout(), Clear(ClearType::All))?;

        loop {
            render_result(&game_result, elapsed, animation_start.elapsed())?;

            if !poll(Duration::from_millis(50))? {
                continue;
            }
            match read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Enter => return Ok(true),
                    KeyCode::Char('q' | 'Q') => return Ok(false),
                    KeyCode::Char('r' | 'R') => break,
                    _ => {}
                },
                Event::Resize(_, _) => {
                    execute!(io::stdout(), Clear(ClearType::All))?;
                }
                _ => {}
            }
        }
    }
}

fn custom_word_count(animation_start: &Instant) -> io::Result<Option<u16>> {
    let mut input = String::new();
    let mut error = false;

    execute!(io::stdout(), Clear(ClearType::All))?;

    loop {
        render_custom_menu(&input, error, animation_start.elapsed())?;

        if !poll(Duration::from_millis(50))? {
            continue;
        }

        match read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Char(character) if character.is_ascii_digit() => {
                    if input.len() < 4 {
                        input.push(character);
                        error = false;
                    }
                }
                KeyCode::Backspace => {
                    input.pop();
                    error = false;
                }
                KeyCode::Enter => match input.parse::<u16>() {
                    Ok(count) if (1..=1000).contains(&count) => {
                        return Ok(Some(count));
                    }
                    _ => error = true,
                },
                KeyCode::Char('q' | 'Q') => return Ok(None),
                _ => {}
            },
            Event::Resize(_, _) => {
                execute!(io::stdout(), Clear(ClearType::All))?;
            }
            _ => {}
        }
    }
}

fn app() -> io::Result<()> {
    execute!(
        io::stdout(),
        EnterAlternateScreen,
        Hide,
        Clear(ClearType::All)
    )?;

    let mut selected: usize = 0;
    const OPTION_COUNT: usize = 4;

    let animation_start = Instant::now();

    loop {
        render_start_menu(selected, animation_start.elapsed())?;

        if !poll(Duration::from_millis(50))? {
            continue;
        }
        match read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Down => {
                    selected = (selected + 1) % OPTION_COUNT;
                }
                KeyCode::Up => {
                    selected = (selected + OPTION_COUNT - 1) % OPTION_COUNT;
                }
                KeyCode::Enter => {
                    let counter = match selected {
                        0 => Some(10),
                        1 => Some(20),
                        2 => Some(50),
                        3 => custom_word_count(&animation_start)?,
                        _ => unreachable!(),
                    };

                    if let Some(counter) = counter {
                        if !run(counter, &animation_start)? {
                            return Ok(());
                        }
                    }

                    execute!(io::stdout(), Clear(ClearType::All))?;
                }
                KeyCode::Char('q' | 'Q') => return Ok(()),
                _ => {}
            },
            Event::Resize(_, _) => {
                execute!(io::stdout(), Clear(ClearType::All))?;
            }
            _ => {}
        }
    }
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;

    let result = app();

    let screen_result = execute!(io::stdout(), ResetColor, Show, LeaveAlternateScreen);
    let raw_result = disable_raw_mode();

    result?;
    screen_result?;
    raw_result?;

    Ok(())
}
