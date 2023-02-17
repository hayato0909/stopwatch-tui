use termion::{
    event::Key,
    input::TermRead,
    raw::IntoRawMode,
    screen::AlternateScreen,
    color,
    clear,
    style, cursor,
    async_stdin,
};
use std::{
    io::{Write, stdout, stdin},
    error::Error,
    thread::{sleep, spawn}, time::Duration,
};
use tui::{
    backend::{Backend, TermionBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, BorderType, Borders},
    Terminal,
    Frame,
};

struct App {
    number: u16,
}

impl App {
    fn new() -> App {
        App {
            number: 0,
        }
    }
}

const FIRST: [[i32; 3]; 5] = [[0, 0, 1], [0, 0, 1], [0, 0, 1], [0, 0, 1], [0, 0, 1]];
const SECOND: [[i32; 3]; 5] = [[1, 1, 1], [0, 0, 1], [1, 1, 1], [1, 0, 0], [1, 1, 1]];
const THREE: [[i32; 3]; 5] = [[1, 1, 1], [0, 0, 1], [1, 1, 1], [0, 0, 1], [1, 1, 1]];
const FOUR: [[i32; 3]; 5] = [[1, 0, 1], [1, 0, 1], [1, 1, 1], [0, 0, 1], [0, 0, 1]];
const FIVE: [[i32; 3]; 5] = [[1, 1, 1], [1, 0, 0], [1, 1, 1], [0, 0, 1], [1, 1, 1]];
const SIX: [[i32; 3]; 5] = [[1, 1, 1], [1, 0, 0], [1, 1, 1], [1, 0, 1], [1, 1, 1]];
const SEVEN: [[i32; 3]; 5] = [[1, 1, 1], [1, 0, 1], [0, 0, 1], [0, 0, 1], [0, 0, 1]];
const EIGHT: [[i32; 3]; 5] = [[1, 1, 1], [1, 0, 1], [1, 1, 1], [1, 0, 1], [1, 1, 1]];
const NINE: [[i32; 3]; 5] = [[1, 1, 1], [1, 0, 1], [1, 1, 1], [0, 0, 1], [1, 1, 1]];
const ZERO: [[i32; 3]; 5] = [[1, 1, 1], [1, 0, 1], [1, 0, 1], [1, 0, 1], [1, 1, 1]];

fn main() -> Result<(), Box<dyn Error>> {
    let stdout = stdout().into_raw_mode()?;
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new();
    run_app(&mut terminal, app)?;
    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> Result<(), Box<dyn Error>> {
    let mut stdin = async_stdin().keys();
    loop {
        terminal.draw(|f| ui(f, &mut app))?;
        match stdin.next() {
            Some(Ok(key)) => {
                match key {
                    Key::Char('q') => break,
                    _ => {}
                }
            }
            _ => {}
        }
        sleep(Duration::from_millis(1000));
        app.number += 1;
        if app.number == 10 { app.number = 0; }
    }
    // write!(stdout, "{}{}{}{}", clear::All, style::Reset, cursor::Goto(1, 1), cursor::Show).unwrap();
    Ok(())
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let size = f.size();

    let number = {
        match app.number {
            0 => ZERO,
            1 => FIRST,
            2 => SECOND,
            3 => THREE,
            4 => FOUR,
            5 => FIVE,
            6 => SIX,
            7 => SEVEN,
            8 => EIGHT,
            9 => NINE,
            _ => ZERO,
        }
    };

    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(&mut stdout, "{}{}{}", style::Reset, cursor::Goto(1, 1), cursor::Hide).unwrap();
    for i in 0..5 {
        for j in 0..3 {
            if number[i][j] == 1 {
                write!(stdout, "{}{}  {}", cursor::Goto(size.width as u16 / 2 - 3 + 2 * j as u16, size.height as u16 / 2 - 2 + i as u16), color::Bg(color::White), style::Reset).unwrap();
            } else {
                write!(stdout, "{}  {}", cursor::Goto(size.width as u16 / 2 - 3 + 2 * j as u16, size.height as u16 / 2 - 2 + i as u16), style::Reset).unwrap();
            }
        }
    }

    let block = Block::default()
        .borders(Borders::ALL)
        .title("Timer")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);
    f.render_widget(block, size);
}