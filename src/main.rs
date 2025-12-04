mod tui;
mod ui;
mod app;

use anyhow::Result;
use clap::Parser;
// Adicionamos KeyModifiers para detectar o Ctrl
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
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

                // --- ATALHOS GLOBAIS (Fase 7) ---
                // Verifica se Ctrl + s foi pressionado
                if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('s') {
                    app.save_file();
                    continue; // Pula o resto da lógica (não insere 's' no texto)
                }

                match app.mode {
                    Mode::Normal => match key.code {
                        KeyCode::Char('q') => break,

                        KeyCode::Char('I') => app.switch_mode(Mode::Insert),
                        KeyCode::Char('?') => app.switch_mode(Mode::Help),
                        KeyCode::Char('e') => app.switch_mode(Mode::Insert),

                        KeyCode::Char('h') | KeyCode::Left => app.move_cursor_left(),
                        KeyCode::Char('j') | KeyCode::Down => app.move_cursor_down(),
                        KeyCode::Char('k') | KeyCode::Up => app.move_cursor_up(),
                        KeyCode::Char('l') | KeyCode::Right => app.move_cursor_right(),

                        _ => {}
                    },

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

                    Mode::Help => match key.code {
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