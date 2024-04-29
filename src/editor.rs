use std::env;

use crate::Document;
use crate::Row;
use crate::Terminal;
use std::time::Duration;
use std::time::Instant;
use termion::color;
use termion::event::Key;

const STATUS_FG_COLOR: color::Rgb = color::Rgb(63, 63, 63);
const STATUS_BG_COLOR: color::Rgb = color::Rgb(239, 239, 239);
const VERSION: &str = env!("CARGO_PKG_VERSION");
const QUIT_TIMES: u8 = 3;

#[derive(PartialEq, Copy, Clone)]
pub enum SearchDirection {
    Forward,
    Backward,
}

#[derive(Default, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

struct StatusMessage {
    text: String,
    time: Instant,
}

impl StatusMessage {
    fn from(message: String) -> Self {
        Self {
            time: Instant::now(),
            text: message,
        }
    }
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_pos: Position,
    offset: Position,
    document: Document,
    status_msg: StatusMessage,
    quit_times: u8,
}

impl Editor {
    pub fn run(&mut self) {
        loop {
            if let Err(error) = self.refresh_screen() {
                die(&error)
            }
            if self.should_quit {
                break;
            }
            if let Err(error) = self.process_keypress() {
                die(&error);
            }
        }
    }

    pub fn default() -> Self {
        let args: Vec<String> = env::args().collect();
        let mut initial_status =
            String::from("HELP: Ctrl-F = find | Ctrl-S = save | Ctrl-A = quit");
        let document = if args.len() > 1 {
            let file_name = &args[1];
            // Document::open(&file_name).unwrap_or_default()
            let doc = Document::open(file_name);
            if doc.is_ok() {
                doc.unwrap()
            } else {
                initial_status = format!("ERR: Could not open file: {}", file_name);
                Document::default()
            }
        } else {
            Document::default()
        };

        Self {
            should_quit: false,
            terminal: Terminal::default().expect("initional the termional failed"),
            // cursor_pos: Position { x: 0, y: 0 },
            cursor_pos: Position::default(),
            offset: Position::default(),
            // document: Document::open(),
            document,
            status_msg: StatusMessage::from(initial_status),
            quit_times: 0,
        }
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            // Key::Ctrl('c') => panic!("Program end"),
            // Key::Ctrl('a') => self.should_quit = true,
            Key::Ctrl('a') => {
                if self.quit_times > 0 && self.document.is_dirty() {
                    self.status_msg = StatusMessage::from(format!(
                        "WARNING! File has unsaved changes Press CTRL-A {} more times to quit.",
                        self.quit_times
                    ));
                    self.quit_times -= 1;
                    return Ok(());
                }
                self.should_quit = true
            }
            Key::Char(c) => {
                self.document.insert(&self.cursor_pos, c);
                self.move_cursor(Key::Right);
            }
            Key::Backspace => {
                if self.cursor_pos.x > 0 || self.cursor_pos.y > 0 {
                    self.move_cursor(Key::Left);
                    self.document.delete(&self.cursor_pos);
                }
                // self.document.insert(&self.cursor_pos, c);
            }
            Key::Delete => {
                self.document.delete(&self.cursor_pos);
                // if self.cursor_pos.x >= 0 && self.cursor_pos.y > 0 {
                // self.move_cursor(Key::Left);
                // }
            }
            Key::Ctrl('s') => {
                self.save();
            }
            Key::Ctrl('f') => {
                self.search();
            }
            Key::Insert => {}
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
        if self.quit_times < QUIT_TIMES {
            self.quit_times = QUIT_TIMES;
            self.status_msg = StatusMessage::from(String::new());
        }
        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        Terminal::cursor_pos(&Position::default());
        if self.should_quit {
            Terminal::clear_screen();
            println!("Goodbye!!!");
        } else {
            self.draw_rows();
            // Terminal::cursor_pos(&self.cursor_pos);
            self.draw_status_bar();
            self.draw_status_msg();

            Terminal::cursor_pos(&Position {
                x: self.cursor_pos.x.saturating_sub(self.offset.x),
                y: self.cursor_pos.y.saturating_sub(self.offset.y),
            });
        }
        Terminal::cursor_show();
        Terminal::flash()
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        // for row in 0..self.terminal.size().height - 1 {
        for terminal_row in 0..height {
            Terminal::clear_current_line();
            // if terminal_row == height / 3 {
            // if let Some(row) = self
            //     .document
            //     .row(terminal_row as usize + self.offset.y as usize)
            if let Some(row) = self
                .document
                .row(self.offset.y.saturating_add(terminal_row as usize))
            {
                self.draw_row(row);
            } else if self.document.is_empty() && terminal_row == height / 3 {
                self.draw_welcome_msg();
            } else {
                println!("~\r");
            }
        }
    }

    fn move_cursor(&mut self, key: Key) {
        let Position { mut x, mut y } = self.cursor_pos;
        // let size = self.terminal.size();
        // let width = size.width.saturating_sub(1) as usize;
        let mut width = if let Some(row) = self.document.row(y) {
            row.len()
        } else {
            0
        };
        // let height = size.height.saturating_sub(1) as usize;
        let height = self.document.len() as usize;

        // let terminal_width = self.terminal.size().width as usize;
        let terminal_height = self.terminal.size().height as usize;

        match key {
            Key::Up => y = y.saturating_sub(1),
            Key::Down => {
                if y < height {
                    y = y.saturating_add(1)
                }
            }
            Key::Left => {
                if x > 0 {
                    x = x.saturating_sub(1)
                } else if y > 0 {
                    y -= 1;
                    x = if let Some(row) = self.document.row(y) {
                        row.len()
                    } else {
                        0
                    };
                }
            }
            Key::Right => {
                if x < width {
                    x = x.saturating_add(1)
                } else if y < height {
                    // y += 1;
                    y = y.saturating_add(1);
                    x = 0;
                }
            }
            Key::PageUp => {
                y = if y > terminal_height {
                    // y - terminal_height
                    y.saturating_sub(terminal_height)
                } else {
                    0
                }
            }
            Key::PageDown => {
                y = if y.saturating_add(terminal_height) < height {
                    // y + terminal_height
                    y.saturating_add(terminal_height)
                } else {
                    height
                }
            }
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

        self.cursor_pos = Position { x, y }
    }

    fn draw_welcome_msg(&self) {
        let mut welcome_msg = format!("Hecto editor -- version {}\r", VERSION);
        let width = self.terminal.size().width as usize;
        let len = welcome_msg.len();
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding);
        welcome_msg = format!("~{}{}", spaces, welcome_msg);
        welcome_msg.truncate(width);
        println!("{}\r", welcome_msg);
    }

    pub fn draw_row(&self, row: &Row) {
        let width = self.terminal.size().width as usize;
        let start = self.offset.x as usize;
        // let end = self.offset.x + self.terminal.size().width as usize;
        let end = self.offset.x.saturating_add(width);

        let row = row.render(start, end);
        println!("{}\r", row);
    }

    fn scroll(&mut self) {
        let Position { x, y } = self.cursor_pos;
        let width = self.terminal.size().width as usize;
        let height = self.terminal.size().height as usize;

        let offset = &mut self.offset;

        if y < offset.y {
            offset.y = y;
        } else if y >= offset.y.saturating_add(height) {
            offset.y = y.saturating_sub(height).saturating_add(1);
        }

        if x < offset.x {
            offset.x = x;
        } else if x >= offset.x.saturating_add(width) {
            offset.x = x.saturating_sub(width).saturating_add(1);
        }
    }

    fn draw_status_bar(&self) {
        // let spaces = " ".repeat(self.terminal.size().width as usize);
        let mut status;
        let width = self.terminal.size().width as usize;

        let modified_indicator = if self.document.is_dirty() {
            " (modeified)"
        } else {
            ""
        };

        let mut file_name = "[No Name]".to_string();
        if let Some(name) = &self.document.filename {
            file_name = name.clone();
            file_name.truncate(20);
        }
        // status = format!("{} - {} lines", file_name, self.document.len());
        status = format!(
            "{} - {} lines{}",
            file_name,
            self.document.len(),
            modified_indicator
        );
        let line_indicator = format!(
            // "{}/{}
            "{}/{}",
            // self.cursor_pos.x.saturating_add(1),
            // self.document.row(self.cursor_pos.y).unwrap().len(),
            self.cursor_pos.y.saturating_add(1),
            self.document.len()
        );
        let len = status.len() + line_indicator.len();
        // if width > len {
        //     status.push_str(&" ".repeat(width - len));
        // }
        status.push_str(&" ".repeat(width.saturating_sub(len)));

        status = format!("{}{}", status, line_indicator);
        status.truncate(width);

        Terminal::set_bg_color(STATUS_BG_COLOR);
        Terminal::set_fg_color(STATUS_FG_COLOR);
        println!("{}\r", status);
        Terminal::unset_bg_color();
        Terminal::unset_fg_color();
    }

    fn draw_status_msg(&self) {
        Terminal::clear_current_line();
        let message = &self.status_msg;
        if Instant::now() - message.time < Duration::new(5, 0) {
            let mut test = message.text.clone();
            test.truncate(self.terminal.size().width as usize);
            print!("{}", test);
        }
    }

    fn prompt<C>(&mut self, prompt: &str, mut callback: C) -> Result<Option<String>, std::io::Error>
    where
        C: FnMut(&mut Self, Key, &String),
    {
        let mut result = String::new();
        loop {
            self.status_msg = StatusMessage::from(format!("{}{}", prompt, result));
            self.refresh_screen()?;
            let key = Terminal::read_key()?;
            match key {
                Key::Backspace => {
                    if !result.is_empty() {
                        result.truncate(result.len() - 1);
                    }
                }
                Key::Char('\n') => break,
                Key::Char(c) => {
                    if !c.is_control() {
                        result.push(c);
                    }
                }
                Key::Esc => {
                    result.truncate(0);
                    break;
                }
                _ => (),
            }
            callback(self, key, &result);
        }
        self.status_msg = StatusMessage::from(String::new());
        if result.is_empty() {
            return Ok(None);
        }
        Ok(Some(result))
    }

    fn save(&mut self) {
        if self.document.filename.is_none() {
            // let new_name = self.prompt("Save as: ").unwrap_or(None);
            let new_name = self.prompt("Save as: ", |_, _, _| {}).unwrap_or(None);
            if new_name.is_none() {
                self.status_msg = StatusMessage::from("Save aborted.".to_string());
                return;
            }
            self.document.filename = new_name;
        }
        if self.document.save().is_ok() {
            self.status_msg = StatusMessage::from("File saved successfully.".to_string());
        } else {
            self.status_msg = StatusMessage::from("Error writiing file!".to_string());
        }
    }

    fn search(&mut self) {
        let old_position = self.cursor_pos.clone();
        let mut direction = SearchDirection::Forward;

        let query = self
            .prompt(
                "Search(ESC to cancle, Arrows to navigate): ",
                |editor, key, query| {
                    let mut moved = false;

                    match key {
                        Key::Right | Key::Down => {
                            direction = SearchDirection::Forward;
                            editor.move_cursor(Key::Right);
                            moved = true;
                        }
                        Key::Left | Key::Up => {
                            direction = SearchDirection::Backward;
                        }
                        _ => (),
                    }
                    if let Some(position) =
                        editor.document.find(&query, &editor.cursor_pos, direction)
                    {
                        editor.cursor_pos = position;
                        editor.scroll();
                    } else if moved {
                        editor.move_cursor(Key::Left);
                    }
                    editor.document.hightlight(Some(query))
                },
            )
            .unwrap_or(None);
        if query.is_none() {
            self.cursor_pos = old_position;
            self.scroll();
        }

        self.document.hightlight(None);

        // if let Some(query) = self
        //     .prompt(
        //         "Search(ESC to cancle, Arrows to navigate): ",
        //         |editor, key, query| {
        //             let mut moved = false;
        //             match key {
        //                 Key::Right | Key::Down => {
        //                     editor.move_cursor(Key::Right);
        //                     moved = true;
        //                 }
        //                 _ => (),
        //             }
        //             if let Some(position) = editor.document.find(&query, &editor.cursor_pos) {
        //                 editor.cursor_pos = position;
        //                 editor.scroll();
        //             } else if moved {
        //                 editor.move_cursor(Key::Left);
        //             }
        //         },
        //     )
        //     .unwrap_or(None)
        // {
        //     if let Some(position) = self.document.find(&query[..], &old_position) {
        //         self.cursor_pos = position;
        //     } else {
        //         self.status_msg = StatusMessage::from("Not found".to_string());
        //     }
        // }
    }
}

fn die(e: &std::io::Error) {
    Terminal::clear_screen();
    panic!("{}", e);
}
