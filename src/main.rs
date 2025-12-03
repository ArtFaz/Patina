mod tui;
mod ui;
mod app;

use anyhow::Result;
use clap::Parser;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crate::app::{App, Mode};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(required = false)]
    filename: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut terminal = tui::init()?;
    let mut app = App::new();

    if let Some(file) = args.filename {
        app.load_file(&file);
    }

    loop {
        if let Ok(size) = terminal.size() {
            app.viewport_height = (size.height.saturating_sub(1)) as usize;
        }

        terminal.draw(|frame| {
            ui::render(frame, &app);
        })?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                match app.mode {
                    // --- MODO NORMAL ---
                    Mode::Normal => match key.code {
                        KeyCode::Char('q') => break,

                        // Atalhos de Mudança de Modo
                        KeyCode::Char('I') => app.switch_mode(Mode::Insert), // Shift+i
                        KeyCode::Char('?') => app.switch_mode(Mode::Help),   // Abre Ajuda
                        KeyCode::Char('e') => app.switch_mode(Mode::Insert), // Atalho do Dashboard

                        // Movimento
                        KeyCode::Char('h') | KeyCode::Left => app.move_cursor_left(),
                        KeyCode::Char('j') | KeyCode::Down => app.move_cursor_down(),
                        KeyCode::Char('k') | KeyCode::Up => app.move_cursor_up(),
                        KeyCode::Char('l') | KeyCode::Right => app.move_cursor_right(),

                        _ => {}
                    },

                    // --- MODO INSERT ---
                    Mode::Insert => match key.code {
                        KeyCode::Esc => app.switch_mode(Mode::Normal),
                        KeyCode::Enter => app.enter_key(),
                        KeyCode::Backspace => app.delete_char(),
                        KeyCode::Char(c) => app.insert_char(c),
                        KeyCode::Left => app.move_cursor_left(),
                        KeyCode::Down => app.move_cursor_down(),
                        KeyCode::Up => app.move_cursor_up(),
                        KeyCode::Right => app.move_cursor_right(),
                        _ => {}
                    },

                    // --- MODO HELP ---
                    Mode::Help => match key.code {
                        // Qualquer uma dessas teclas fecha a ajuda
                        KeyCode::Esc | KeyCode::Char('?') | KeyCode::Char('q') => {
                            app.switch_mode(Mode::Normal);
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    tui::restore()?;
    Ok(())
}