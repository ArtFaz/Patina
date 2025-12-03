use std::io::{self, stdout, Stdout};
use crossterm::{execute, terminal::*};
use ratatui::prelude::*;

// Aqui criamos um "apelido" para o tipo do nosso Terminal,
// para não ter que digitar "Terminal<CrosstermBackend<Stdout>>" toda hora.
pub type Tui = Terminal<CrosstermBackend<Stdout>>;

/// Inicializa o terminal para o nosso editor.
pub fn init() -> io::Result<Tui> {
    // 1. Entra na "Alternate Screen" (uma tela secundária, para não sujar o histórico do terminal do usuário)
    execute!(stdout(), EnterAlternateScreen)?;

    // 2. Ativa o Raw Mode (captura teclas cruas, sem esperar Enter)
    enable_raw_mode()?;

    // 3. Cria a interface do Ratatui usando o Backend do Crossterm
    Terminal::new(CrosstermBackend::new(stdout()))
}

/// Restaura o terminal para o estado original.
pub fn restore() -> io::Result<()> {
    // 1. Sai da Alternate Screen
    execute!(stdout(), LeaveAlternateScreen)?;

    // 2. Desativa o Raw Mode
    disable_raw_mode()?;

    Ok(())
}