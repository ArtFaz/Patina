use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};
use crossterm::{cursor::SetCursorStyle, execute};
use std::io::stdout;

use crate::app::{App, Mode};

// --- CONSTANTES VISUAIS ---
// Novo logo atualizado
const LOGO: &str = r#"
:::::::::     ::: ::::::::::: ::::::::::: ::::    :::     :::
:+:    :+:  :+: :+:   :+:         :+:     :+:+:   :+:   :+: :+:
+:+    +:+ +:+   +:+  +:+         +:+     :+:+:+  +:+  +:+   +:+
+#++:++#+ +#++:++#++: +#+         +#+     +#+ +:+ +#+ +#++:++#++:
+#+       +#+     +#+ +#+         +#+     +#+  +#+#+# +#+     +#+
#+#       #+#     #+# #+#         #+#     #+#   #+#+# #+#     #+#
###       ###     ### ###     ########### ###    #### ###     ###
"#;

pub fn render(frame: &mut Frame, app: &App) {
    // 1. DECISÃO DO QUE MOSTRAR
    let is_virgin = app.buffer.len() == 1 && app.buffer[0].is_empty();

    if app.filename.is_none() && is_virgin && app.mode == Mode::Normal {
        render_dashboard(frame);
    } else {
        render_editor(frame, app);
    }

    // 2. OVERLAYS (POPUPS)
    if app.mode == Mode::Help {
        render_help_popup(frame);
    }
}

fn render_editor(frame: &mut Frame, app: &App) {
    let layout_vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(frame.area());

    let layout_editor = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(4), Constraint::Min(0)])
        .split(layout_vertical[0]);

    let area_gutter = layout_editor[0];
    let area_buffer = layout_editor[1];
    let area_status = layout_vertical[1];

    // Scroll Logic
    let height = area_buffer.height as usize;
    let start_line = app.scroll_y;
    let end_line = std::cmp::min(app.scroll_y + height, app.buffer.len());

    // Gutter
    let line_numbers: String = (start_line..end_line)
        .map(|n| format!("{:>3} ", n + 1))
        .collect::<Vec<String>>()
        .join("\n");

    frame.render_widget(
        Paragraph::new(line_numbers).style(Style::default().fg(Color::DarkGray)),
        area_gutter
    );

    // Buffer Content
    let visible_content = app.buffer.iter()
        .skip(start_line)
        .take(height)
        .cloned()
        .collect::<Vec<String>>()
        .join("\n");

    frame.render_widget(
        Paragraph::new(visible_content).block(Block::default().borders(Borders::NONE)),
        area_buffer
    );

    // Status Bar
    let (mode_text, mode_color) = match app.mode {
        Mode::Normal => (" NORMAL ", Color::Blue),
        Mode::Insert => (" INSERT ", Color::Green),
        Mode::Help   => (" HELP ", Color::Yellow),
    };

    let status_bar = Line::from(vec![
        Span::styled(mode_text, Style::default().bg(mode_color).fg(Color::Black).bold()),
        Span::raw(" | "),
        Span::raw(app.filename.as_deref().unwrap_or("[Sem Nome]")),
        Span::raw(" | "),
        Span::raw(format!("Ln {}, Col {}", app.cursor_y + 1, app.cursor_x + 1)),
    ]);

    frame.render_widget(
        Paragraph::new(status_bar).style(Style::default().bg(Color::Rgb(20, 20, 20)).fg(Color::White)),
        area_status
    );

    // Cursor
    let cursor_style = match app.mode {
        Mode::Normal => SetCursorStyle::BlinkingBlock,
        Mode::Insert => SetCursorStyle::BlinkingBar,
        Mode::Help   => SetCursorStyle::BlinkingBlock,
    };
    let _ = execute!(stdout(), cursor_style);

    let visual_cursor_y = app.cursor_y.saturating_sub(app.scroll_y);
    frame.set_cursor_position((
        area_buffer.x + app.cursor_x as u16,
        area_buffer.y + visual_cursor_y as u16,
    ));
}

// --- RENDERIZAÇÃO DO DASHBOARD ---

fn render_dashboard(frame: &mut Frame) {
    let area = centered_rect(60, 60, frame.area());

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(10),
            Constraint::Min(0),     // Menu
            Constraint::Length(2),  // Footer
        ])
        .split(area);

    // --- CORREÇÃO DA LARGURA ---
    // O novo logo tem 68 caracteres de largura.
    // Ajustamos a Constraint para garantir que nada seja cortado.

    let logo_width = 68; // <--- AJUSTADO DE 46 PARA 68
    let logo_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(0),           // Espaço vazio esquerda
            Constraint::Length(logo_width), // O Logo exato
            Constraint::Min(0),           // Espaço vazio direita
        ])
        .split(layout[0]);

    let logo = Paragraph::new(LOGO.trim())
        .alignment(Alignment::Left)
        .style(Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD));

    frame.render_widget(logo, logo_layout[1]);

    // 2. Menu
    let menu_options = vec![
        "",
        " [e]    Novo Arquivo / Editar ",
        " [?]    Ajuda / Keymaps       ",
        " [q]    Sair                  ",
    ];

    let menu_text: Vec<Line> = menu_options.iter().map(|&opt| {
        Line::from(Span::styled(opt, Style::default().fg(Color::White)))
    }).collect();

    let menu = Paragraph::new(menu_text)
        .alignment(Alignment::Center)
        .style(Style::default());

    frame.render_widget(menu, layout[1]);

    // 3. Footer
    let footer = Paragraph::new("Patina v0.1.0 • Made in Rust")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::DarkGray));

    frame.render_widget(footer, layout[2]);
}

fn render_help_popup(frame: &mut Frame) {
    let area = centered_rect(50, 50, frame.area());
    frame.render_widget(Clear, area);

    let help_text = vec![
        Line::from(vec![Span::styled("Movimento:", Style::default().add_modifier(Modifier::BOLD))]),
        Line::from("  h/j/k/l ou Setas  -> Mover Cursor"),
        Line::from(""),
        Line::from(vec![Span::styled("Modos:", Style::default().add_modifier(Modifier::BOLD))]),
        Line::from("  Shift + i / e     -> Entrar no Insert"),
        Line::from("  Esc               -> Sair do Insert"),
        Line::from("  ?                 -> Abrir/Fechar Ajuda"),
        Line::from(""),
        Line::from(vec![Span::styled("Geral:", Style::default().add_modifier(Modifier::BOLD))]),
        Line::from("  q                 -> Sair do Patina"),
    ];

    let block = Block::default()
        .title(" Ajuda do Patina ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow))
        .style(Style::default().bg(Color::Rgb(20, 20, 20)));

    let paragraph = Paragraph::new(help_text)
        .block(block)
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Left);

    frame.render_widget(paragraph, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}