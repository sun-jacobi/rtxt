use crate::terminal::Terminal;
use crate::Doc;
use crate::Row;
use std::env;
//use colored::Colorize;
use termion::event::Key;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn die(err: std::io::Error) {
    Terminal::clear_screen();
    panic!("{}", err);
}

#[derive(Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor: Position,
    offset: Position,
    document: Doc,
}

impl Editor {
    pub fn default() -> Self {
        let args: Vec<String> = env::args().collect();
        let doc = if args.len() > 1 {
            let filename = &args[1];
            Doc::open(&filename).unwrap_or_default()
        } else {
            Doc::default()
        };
        Self {
            should_quit: false,
            terminal: Terminal::default().expect("failed to initialize Terminal"),
            cursor: Position::default(),
            offset: Position::default(),
            document: doc,
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
            Key::Char(c) => match c {
                '\n' => {
                    self.document.insert(&self.cursor, c);
                    self.move_cursor(Key::Down);
                }
                '\t' => {
                    self.document.insert(&self.cursor, c);
                    for _ in 0..4 {
                        self.move_cursor(Key::Right);
                    }
                }
                _ => {
                    self.document.insert(&self.cursor, c);
                    self.move_cursor(Key::Right);
                }
            },
            // For mac
            Key::Backspace => {
                self.move_cursor(Key::Left);
                self.document.delete(&self.cursor);
            }
            Key::Delete => self.document.delete(&self.cursor),
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
        self.scroll();
        Ok(())
    }

    fn scroll(&mut self) {
        let Position { x, y } = self.cursor;
        let height = self.terminal.size().height as usize;
        let width = self.terminal.size().width as usize;
        if y < self.offset.y {
            self.offset.y = y;
        } else if y >= self.offset.y.saturating_add(height) {
            self.offset.y = self.offset.y.saturating_add(height);
        }
        if x < self.offset.x {
            self.offset.x = x;
        } else if x >= self.offset.x.saturating_add(width) {
            self.offset.x = self.offset.x.saturating_add(width)
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        Terminal::cursor_postion(&Position { x: 0, y: 0 });
        if self.should_quit {
            Terminal::clear_screen();
            println!("Successfully Quit.\r")
        } else {
            self.draw_rows();
            Terminal::cursor_postion(&Position {
                x: self.cursor.x.saturating_sub(self.offset.x),
                y: self.cursor.y.saturating_sub(self.offset.y),
            });
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

    fn draw_row(&self, row: &Row) {
        let start = self.offset.x;
        let end = self.offset.x + self.terminal.size().width as usize;
        let row = row.render(start, end);
        println!("{}\r", row);
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for row_index in 0..height - 1 {
            Terminal::clear_current_line();
            if let Some(row) = self.document.row(row_index as usize + self.offset.y) {
                self.draw_row(row);
            } else if self.document.is_empty() && row_index == height / 3 {
                self.draw_welcome();
            } else {
                println!("~\r");
            }
        }
    }
    fn move_cursor(&mut self, key: Key) {
        let Position { mut x, mut y } = self.cursor;
        let height = self.document.len();
        let mut width = if let Some(row) = self.document.row(y) {
            row.len()
        } else {
            0
        };
        match key {
            Key::Up => y = y.saturating_sub(1),
            Key::Down => {
                if y < height {
                    y = y.saturating_add(1)
                }
            }
            Key::Left => {
                if x == 0 {
                    x = match y {
                        0 => 0,
                        _ => {
                            y = y.saturating_sub(1);
                            match self.document.row(y) {
                                Some(row) => row.len(),
                                None => 0,
                            }
                        }
                    };
                } else {
                    x = x.saturating_sub(1);
                }
            }
            Key::Right => {
                if x == width {
                    y = y.saturating_add(1);
                    x = 0;
                } else {
                    x = x.saturating_add(1);
                }
            }
            // not support for Mac
            Key::PageUp => y = 0,
            Key::PageDown => y = height,
            Key::Home => x = 0,
            Key::End => x = width,
            _ => (),
        }
        width = if let Some(row) = self.document.row(y) {
            row.len()
        } else {
            0
        };
        if x > width {
            x = width;
        }
        self.cursor = Position { x, y };
    }
}
