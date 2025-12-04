<div align="center">
<pre>
 ______   ______     ______   __     __   __     ______    
/\  == \ /\  __ \   /\__  _\ /\ \   /\ "-.\ \   /\  __ \   
\ \  _-/ \ \  __ \  \/_/\ \/ \ \ \  \ \ \-.  \  \ \  __ \  
 \ \_\    \ \_\ \_\    \ \_\  \ \_\  \ \_\\"\_\  \ \_\ \_\ 
  \/_/     \/_/\/_/     \/_/   \/_/   \/_/ \/_/   \/_/\/_/
</pre>
</div>

<div align="center">
  
[![Status](https://img.shields.io/badge/status-development-orange.svg?style=for-the-badge&logo=github&logoColor=white)](https://github.com/ArtFaz/Patina)
[![Latest Release](https://img.shields.io/github/v/release/ArtFaz/Patina?style=for-the-badge&logo=github&logoColor=white)](https://github.com/ArtFaz/patina/releases/latest)
[![Language](https://img.shields.io/badge/language-Rust-1f1f1f.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.python.org/) 
[![License](https://img.shields.io/badge/license-MIT-gold.svg?style=for-the-badge)](LICENSE)

</div> 



**Patina** is a lightweight, modal TUI (Terminal User Interface) text editor built in Rust. It aims to bridge the gap between the efficiency of Vim and the approachable aesthetics of modern editors like LazyVim.

Patina is built using the robust [Ratatui](https://ratatui.rs/) ecosystem and handles modern terminal requirements like UTF-8/Unicode characters seamlessly.


## Features

* **Modal Editing:** Vim-inspired workflow with distinct **Normal** and **Insert** modes.
* **Hybrid Navigation:** Supports both classic `HJKL` bindings and standard Arrow keys.
* **Modern Dashboard:** A clean startup screen with quick actions when no file is loaded.
* **UTF-8 Safe:** Robust handling of multi-byte characters, emojis, and accents (no crashes on `ç` or `🦀`).
* **Visual Feedback:** Dynamic cursor styles (Block vs. Bar) and status bar colors based on the active mode.
* **Smart Scrolling:** Automatic viewport adjustment and relative line numbering.
* **Persistence:** Global hotkey to save your work to disk instantly.

## Installation & Usage

Ensure you have [Rust and Cargo](https://rustup.rs/) installed.

### 1. Clone the repository
```bash
git clone [https://github.com/your-username/patina.git](https://github.com/your-username/patina.git)
cd patina
```

### 2. Run the editor
To open the dashboard:
```bash
cargo run
# or if built: ./patina
```

To open a specific file directly:
```bash
cargo run -- my_file.txt
```

## ⌨️ Keybindings

Patina uses a modal input system.

### Global
| Key | Action |
| :--- | :--- |
| `Ctrl + s` | **Save File** (Write buffer to disk) |

### Normal Mode (Blue Status)
| Key | Action |
| :--- | :--- |
| `h` / `←` | Move Cursor Left |
| `j` / `↓` | Move Cursor Down |
| `k` / `↑` | Move Cursor Up |
| `l` / `→` | Move Cursor Right |
| `Shift + i` (or `I`) | Enter **Insert Mode** |
| `e` | Edit (from Dashboard) |
| `?` | Toggle **Help Popup** |
| `q` | Quit Patina |

### Insert Mode (Green Status)
| Key | Action |
| :--- | :--- |
| `Typing` | Insert text |
| `Enter` | New line |
| `Backspace` | Delete character / Merge lines |
| `Esc` | Return to **Normal Mode** |
| `Arrows` | Move Cursor (Convenience) |

## Architecture

The project is structured into modular components:

* **`main.rs`**: Entry point, argument parsing (`clap`), and the main event loop.
* **`app.rs`**: Application state, buffer management, UTF-8 logic, and movement algorithms.
* **`ui.rs`**: Rendering logic using `ratatui`. Handles the dashboard, layout splitting, and cursor styling.
* **`tui.rs`**: Boilerplate for setting up and restoring the terminal raw mode.

## Roadmap

The following features are planned for future releases:

* [ ] **Command Mode (`:`):** Support for commands like `:w`, `:q`, and `:w <filename>`.
* [ ] **Search (`/`):** Find text within the buffer.
* [ ] **Syntax Highlighting:** Basic coloring for common file types.
* [ ] **Clipboard Support:** Copy/Paste integration with the system clipboard.

## License

This project is open-source and available under the MIT License.
