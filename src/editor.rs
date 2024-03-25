use termion::event::Key;

use crate::Terminal;

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
}

impl Editor {
    pub fn run(&mut self) {
        loop {
            if let Err(error) = self.refresh_screen() {
                die(&error)
            }
            if self.should_quit == true {
                break;
            }
            if let Err(error) = self.process_keypress() {
                die(&error);
            }
        }
    }

    pub fn default() -> Self {
        Self {
            should_quit: false,
            terminal: Terminal::default().expect("initional the termional failed"),
        }
    }
    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            // Key::Ctrl('c') => panic!("Program end"),
            Key::Ctrl('a') => self.should_quit = true,
            _ => (),
        }
        Ok(())
    }
    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        // print!("\x1b[2J");
        // print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
        Terminal::clear_screen();
        if self.should_quit {
            println!("Goodbye!!!");
        } else {
            self.draw_rows();
            // print!("{}", termion::cursor::Goto(1, 1));
            Terminal::cursor_pos(0, 0);
        }
        // io::stdout().flush()
        Terminal::flash()
    }

    fn draw_rows(&self) {
        // for _ in 0..24 {
        for _ in 0..self.terminal.size().height {
            println!("~\r");
        }
    }
}

fn die(e: &std::io::Error) {
    Terminal::clear_screen();
    panic!("{}", e);
}
