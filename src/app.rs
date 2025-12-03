use std::fs;

/// Define os modos de operação do editor (Estilo Vim)
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum Mode {
    #[default]
    Normal, // Navegação e comandos
    Insert, // Digitação de texto
    Help,   // Menu de ajuda
}

/// A estrutura principal que segura o estado da aplicação
pub struct App {
    /// O modo atual (Normal, Insert, Help)
    pub mode: Mode,

    /// Flag para saber se devemos fechar o programa
    pub should_quit: bool,

    /// O conteúdo do texto. Cada String é uma linha.
    pub buffer: Vec<String>,

    /// Posição X do cursor (Coluna)
    pub cursor_x: usize,

    /// Posição Y do cursor (Linha)
    pub cursor_y: usize,

    /// Nome do arquivo sendo editado
    pub filename: Option<String>,

    /// Offset de rolagem vertical (Qual linha é a primeira visível na tela)
    pub scroll_y: usize,

    /// Altura da área de visualização (Viewport)
    /// Precisamos disso para saber quando rolar a tela para baixo
    pub viewport_height: usize,
}

impl App {
    /// Cria uma nova instância do App com valores padrão
    pub fn new() -> Self {
        Self {
            mode: Mode::default(),
            should_quit: false,
            buffer: vec![String::new()],
            cursor_x: 0,
            cursor_y: 0,
            filename: None,
            scroll_y: 0,
            viewport_height: 0, // Será atualizado pelo main loop
        }
    }

    /// Carrega um arquivo do disco para o buffer
    pub fn load_file(&mut self, path: &str) {
        if let Ok(contents) = fs::read_to_string(path) {
            self.buffer = contents.lines().map(|line| line.to_string()).collect();
            if self.buffer.is_empty() {
                self.buffer.push(String::new());
            }
        }
        self.filename = Some(path.to_string());
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn switch_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }

    // --- MÉTODOS DE MOVIMENTO COM SCROLL ---

    pub fn move_cursor_left(&mut self) {
        if self.cursor_x > 0 {
            self.cursor_x -= 1;
        }
    }

    pub fn move_cursor_right(&mut self) {
        let current_line_len = self.buffer[self.cursor_y].len();
        if self.cursor_x < current_line_len {
            self.cursor_x += 1;
        }
    }

    pub fn move_cursor_up(&mut self) {
        if self.cursor_y > 0 {
            self.cursor_y -= 1;
            self.clamp_cursor_x();
        }

        // Lógica de Scroll UP
        // Se o cursor subiu além do topo visível (scroll_y), puxamos o scroll junto.
        if self.cursor_y < self.scroll_y {
            self.scroll_y = self.cursor_y;
        }
    }

    pub fn move_cursor_down(&mut self) {
        if self.cursor_y < self.buffer.len() - 1 {
            self.cursor_y += 1;
            self.clamp_cursor_x();
        }

        // Lógica de Scroll DOWN
        // Precisamos saber se o cursor saiu da parte de baixo da tela.
        // A lógica é: Se cursor >= (topo + altura), então precisamos descer o topo.
        if self.viewport_height > 0 {
            if self.cursor_y >= self.scroll_y + self.viewport_height {
                self.scroll_y = self.cursor_y - self.viewport_height + 1;
            }
        }
    }

    fn clamp_cursor_x(&mut self) {
        let current_line_len = self.buffer[self.cursor_y].len();
        if self.cursor_x > current_line_len {
            self.cursor_x = current_line_len;
        }
    }
}