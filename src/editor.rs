use crate::terminal::Terminal;
use termion::event::Key;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn die(err: std::io::Error) {
    Terminal::clear_screen();
    panic!("{}", err);
}

pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor: Position,
}

impl Editor {
    pub fn default() -> Self {
        Self {
            should_quit: false,
            terminal: Terminal::default().expect("failed to initialize Terminal"),
            cursor: Position { x: 0, y: 0 },
        }
    }
    pub fn run(&mut self) {
        loop {
            if let Err(err) = self.refresh_screen() {
                die(err);
            }
            if self.should_quit {
                break;
            }
            if let Err(err) = self.process_press() {
                die(err);
            }
        }
    }
    fn process_press(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            Key::Up
            | Key::Down
            | Key::Left
            | Key::Right
            | Key::PageUp
            | Key::PageDown
            | Key::End
            | Key::Home => self.move_cursor(pressed_key),
            _ => (),
        }
        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        Terminal::cursor_postion(&Position { x: 0, y: 0 });
        if self.should_quit {
            Terminal::clear_screen();
            println!("Successfully Quit.\r")
        } else {
            self.draw_rows();
            Terminal::cursor_postion(&self.cursor);
        }
        Terminal::cursor_show();
        Terminal::flush()
    }
    fn draw_welcome(&self) {
        let mut welcome_message = format!("Rtxt editor -- version {}", VERSION);
        let width = self.terminal.size().width as usize;
        let len = welcome_message.len();
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{}{}", spaces, welcome_message);
        welcome_message.truncate(width);
        println!("{}\r", welcome_message);
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for row in 0..height - 1 {
            Terminal::clear_current_line();
            if row == height / 3 {
                self.draw_welcome();
            } else {
                println!("~\r");
            }
        }
    }
    fn move_cursor(&mut self, key: Key) {
        let Position { mut y, mut x } = self.cursor;
        let size = self.terminal.size();
        let height = size.height.saturating_sub(1) as usize;
        let width = size.width.saturating_sub(1) as usize;
        match key {
            Key::Up => y = y.saturating_sub(1),
            Key::Down => {
                if y < height {
                    y = y.saturating_add(1)
                }
            }
            Key::Left => x = x.saturating_sub(1),
            Key::Right => {
                if x < width {
                    x = x.saturating_add(1)
                }
            }
            // not support for Mac
            Key::PageUp => y = 0,
            Key::PageDown => y = height,
            Key::Home => x = 0,
            Key::End => x = width,
            _ => (),
        }
        self.cursor = Position { x, y };
    }
}
