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
    pub mode: Mode,
    pub should_quit: bool,
    pub buffer: Vec<String>,
    pub cursor_x: usize,
    pub cursor_y: usize,
    pub filename: Option<String>,
    pub scroll_y: usize,
    pub viewport_height: usize,
}

impl App {
    pub fn new() -> Self {
        Self {
            mode: Mode::default(),
            should_quit: false,
            buffer: vec![String::new()],
            cursor_x: 0,
            cursor_y: 0,
            filename: None,
            scroll_y: 0,
            viewport_height: 0,
        }
    }

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

    // --- HELPERS UTF-8 (A CORREÇÃO DO CRASH) ---

    /// Converte a posição visual X (caracteres) para índice de bytes
    fn get_byte_index(&self) -> usize {
        if self.cursor_y >= self.buffer.len() {
            return 0;
        }

        self.buffer[self.cursor_y]
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.cursor_x)
            .unwrap_or_else(|| self.buffer[self.cursor_y].len())
    }

    /// Retorna a quantidade de caracteres (não bytes) na linha atual
    fn current_line_char_count(&self) -> usize {
        if self.cursor_y >= self.buffer.len() {
            return 0;
        }
        self.buffer[self.cursor_y].chars().count()
    }

    // --- MÉTODOS DE EDIÇÃO ---

    pub fn insert_char(&mut self, c: char) {
        if self.cursor_y >= self.buffer.len() {
            return;
        }

        let byte_idx = self.get_byte_index();
        self.buffer[self.cursor_y].insert(byte_idx, c);
        self.cursor_x += 1;
    }

    pub fn enter_key(&mut self) {
        if self.cursor_y >= self.buffer.len() {
            return;
        }

        let byte_idx = self.get_byte_index();
        let current_line = &mut self.buffer[self.cursor_y];

        // Divide a linha usando o índice de bytes correto
        let right_part = if byte_idx < current_line.len() {
            current_line.split_off(byte_idx)
        } else {
            String::new()
        };

        self.buffer.insert(self.cursor_y + 1, right_part);
        self.cursor_y += 1;
        self.cursor_x = 0;
    }

    pub fn delete_char(&mut self) {
        if self.cursor_y >= self.buffer.len() {
            return;
        }

        if self.cursor_x > 0 {
            // Precisamos encontrar o índice do caractere ANTERIOR ao cursor
            self.cursor_x -= 1;
            let byte_idx = self.get_byte_index();
            self.buffer[self.cursor_y].remove(byte_idx);
        }
        else if self.cursor_y > 0 {
            // Juntar linhas
            let current_line = self.buffer.remove(self.cursor_y);
            let prev_line = &mut self.buffer[self.cursor_y - 1];

            // O novo X será o comprimento da linha anterior (em caracteres)
            let new_cursor_x = prev_line.chars().count();

            prev_line.push_str(&current_line);

            self.cursor_y -= 1;
            self.cursor_x = new_cursor_x;
        }
    }

    // --- MÉTODOS DE MOVIMENTO ---

    pub fn move_cursor_left(&mut self) {
        if self.cursor_x > 0 {
            self.cursor_x -= 1;
        }
    }

    pub fn move_cursor_right(&mut self) {
        // Agora usamos a contagem de CARACTERES, não de bytes (len)
        let len_chars = self.current_line_char_count();

        if self.cursor_x < len_chars {
            self.cursor_x += 1;
        }
    }

    pub fn move_cursor_up(&mut self) {
        if self.cursor_y > 0 {
            self.cursor_y -= 1;
            self.clamp_cursor_x();
        }
        if self.cursor_y < self.scroll_y {
            self.scroll_y = self.cursor_y;
        }
    }

    pub fn move_cursor_down(&mut self) {
        if self.cursor_y < self.buffer.len() - 1 {
            self.cursor_y += 1;
            self.clamp_cursor_x();
        }
        if self.viewport_height > 0 {
            if self.cursor_y >= self.scroll_y + self.viewport_height {
                self.scroll_y = self.cursor_y - self.viewport_height + 1;
            }
        }
    }

    fn clamp_cursor_x(&mut self) {
        // Agora limitamos baseado em caracteres visuais
        let len_chars = self.current_line_char_count();
        if self.cursor_x > len_chars {
            self.cursor_x = len_chars;
        }
    }
}