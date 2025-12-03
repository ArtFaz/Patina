use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::{App, Mode};

pub fn render(frame: &mut Frame, app: &App) {
    // 1. DIVISÃO DA TELA (LAYOUT)
    let layout_vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(frame.area());

    let layout_editor = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(4),
            Constraint::Min(0),
        ])
        .split(layout_vertical[0]);

    let area_gutter = layout_editor[0];
    let area_buffer = layout_editor[1];
    let area_status = layout_vertical[1];

    // --- LÓGICA DO SCROLL (VIEWPORT) ---
    // Calculamos quantas linhas cabem na área de texto
    let height = area_buffer.height as usize;

    // Calculamos o intervalo vertical visível
    // De: scroll_y
    // Até: scroll_y + altura (sem estourar o tamanho do buffer)
    let start_line = app.scroll_y;
    let end_line = std::cmp::min(app.scroll_y + height, app.buffer.len());

    // 2. RENDERIZAÇÃO DO GUTTER (Números)
    // Agora iteramos apenas sobre as linhas visíveis
    let line_numbers: String = (start_line..end_line)
        .map(|n| format!("{:>3} ", n + 1)) // n + 1 porque o usuário vê base 1
        .collect::<Vec<String>>()
        .join("\n");

    let gutter = Paragraph::new(line_numbers)
        .style(Style::default().fg(Color::DarkGray));

    frame.render_widget(gutter, area_gutter);

    // 3. RENDERIZAÇÃO DO CONTEÚDO (Texto)
    // Pegamos apenas a fatia visível do buffer para desenhar
    let visible_content = app.buffer.iter()
        .skip(start_line)
        .take(height)
        .cloned()
        .collect::<Vec<String>>()
        .join("\n");

    let editor_content = Paragraph::new(visible_content)
        .block(Block::default().borders(Borders::NONE));

    frame.render_widget(editor_content, area_buffer);

    // 4. STATUS BAR
    let (mode_text, mode_color) = match app.mode {
        Mode::Normal => (" NORMAL ", Color::Blue),
        Mode::Insert => (" INSERT ", Color::Green),
        Mode::Help   => (" HELP ", Color::Yellow),
    };

    let status_bar = Line::from(vec![
        Span::styled(mode_text, Style::default().bg(mode_color).fg(Color::Black).bold()),
        Span::raw(" | "),
        // Mostra o nome do arquivo ou "[Sem Nome]"
        Span::raw(app.filename.as_deref().unwrap_or("[Sem Nome]")),
        Span::raw(" | "),
        Span::raw(format!("Ln {}, Col {}", app.cursor_y + 1, app.cursor_x + 1)),
    ]);

    let status_widget = Paragraph::new(status_bar)
        .style(Style::default().bg(Color::Rgb(20, 20, 20)).fg(Color::White));

    frame.render_widget(status_widget, area_status);

    // 5. CURSOR (Ajustado pelo Scroll)
    // A posição visual é a posição real MENOS o quanto já rolamos.
    // Usamos saturating_sub para evitar crash caso a lógica de scroll atrase um frame.
    let visual_cursor_y = app.cursor_y.saturating_sub(app.scroll_y);

    frame.set_cursor_position(
        (
            area_buffer.x + app.cursor_x as u16,
            area_buffer.y + visual_cursor_y as u16,
        )
    );
}