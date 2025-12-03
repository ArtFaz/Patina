mod tui;
mod ui;
mod app;

use anyhow::Result;
use clap::Parser;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crate::app::App;

/// Estrutura para os argumentos da linha de comando
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Nome do arquivo para abrir (opcional)
    #[arg(required = false)]
    filename: Option<String>,
}

fn main() -> Result<()> {
    // 1. Parsear os argumentos
    let args = Args::parse();

    // 2. Inicializa infra
    let mut terminal = tui::init()?;

    // 3. Inicializa dados e carrega arquivo
    let mut app = App::new();

    if let Some(file) = args.filename {
        app.load_file(&file);
    }

    // 4. Loop Principal
    loop {
        // --- ATUALIZAÇÃO DO VIEWPORT (Fase 4 - Scroll) ---
        // Antes de desenhar ou mover, atualizamos a altura disponível.
        // O layout usa 1 linha para o rodapé, então a altura do editor é Total - 1.
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

                match key.code {
                    KeyCode::Char('q') => break,

                    // Movimento Híbrido (HJKL + Setas)
                    KeyCode::Char('h') | KeyCode::Left => app.move_cursor_left(),
                    KeyCode::Char('j') | KeyCode::Down => app.move_cursor_down(),
                    KeyCode::Char('k') | KeyCode::Up => app.move_cursor_up(),
                    KeyCode::Char('l') | KeyCode::Right => app.move_cursor_right(),

                    _ => {}
                }
            }
        }
    }

    tui::restore()?;
    Ok(())
}