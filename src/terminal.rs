use std::io::{self, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

use crate::editor::Position;
pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    size: Size,
    _stdout: RawTerminal<std::io::Stdout>,
}

impl Terminal {
    pub fn default() -> Result<Self, std::io::Error> {
        let size = termion::terminal_size()?;
        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1,
            },
            _stdout: stdout().into_raw_mode()?,
        })
    }
    pub fn size(&self) -> &Size {
        &self.size
    }
    pub fn cursor_postion(postion: &Position) {
        let Position { x, y } = postion;
        print!(
            "{}",
            termion::cursor::Goto(x.saturating_add(1) as u16, y.saturating_add(1) as u16)
        );
    }
    pub fn clear_screen() {
        print!("{}", termion::clear::All);
    }
    pub fn flush() -> Result<(), std::io::Error> {
        std::io::stdout().flush()
    }
    pub fn read_key() -> Result<Key, std::io::Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }
    pub fn cursor_hide() {
        print!("{}", termion::cursor::Hide);
    }
    pub fn cursor_show() {
        print!("{}", termion::cursor::Show);
    }
    pub fn clear_current_line() {
        print!("{}", termion::clear::CurrentLine);
    }
}