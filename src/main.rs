use termion::{
    event::Key,
    input::TermRead,
    raw::IntoRawMode,
    screen::AlternateScreen,
    color::{self, Color},
    clear,
    style, cursor,
    async_stdin,
};
use std::{
    io::{Write, stdout, stdin},
    error::Error,
    thread::{sleep, spawn}, time::Duration,
    cmp::max,
};
use tui::{
    backend::{Backend, TermionBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, BorderType, Borders},
    Terminal,
    Frame,
};

struct App {
    min: u16,
    sec: u16,
    is_count: bool,
}

impl App {
    fn new() -> App {
        App {
            min: 0,
            sec: 0,
            is_count: false,
        }
    }

    fn reset(&mut self) {
        self.min = 0;
        self.sec = 0;
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
        if app.is_count {
            sleep(Duration::from_millis(1000));
            if app.sec == 59 && app.min == 59 { continue; }
            else if app.sec == 59 {
                app.sec = 0;
                app.min += 1;
            } else {
                app.sec += 1;
            }
        }

        terminal.draw(|f| ui(f, &mut app))?;
        match stdin.next() {
            Some(Ok(key)) => {
                match key {
                    Key::Char('q') => break,
                    Key::Char('s') => { app.is_count = !app.is_count; }
                    Key::Char('r') => { app.reset(); }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    // write!(stdout, "{}{}{}{}", clear::All, style::Reset, cursor::Goto(1, 1), cursor::Show).unwrap();
    Ok(())
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let size = f.size();

    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(&mut stdout, "{}{}", style::Reset, cursor::Hide).unwrap();
    let center_width = size.width / 2;
    let center_height = size.height / 2 - 2;
    write!(stdout, 
        "{}{} {}", 
        cursor::Goto(center_width, center_height+1),
        color::Bg(color::White),
        style::Reset
    ).unwrap();
    write!(stdout, 
        "{}{} {}", 
        cursor::Goto(center_width, center_height+3),
        color::Bg(color::White),
        style::Reset
    ).unwrap();

    let mut num: [u16; 4] = [0, 0, 0, 0];
    num[0] = app.min / 10;
    num[1] = app.min % 10;
    num[2] = app.sec / 10;
    num[3] = app.sec % 10;

    for k in 0..4 {
        let mut start_pos = center_width as i32 + 7 * (k as i32 - 2);
        if k > 1 { start_pos += 2; }
        let start_pos = max(0, start_pos) as u16;


        let number = {
            match num[k as usize] {
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
        
        for i in 0..5 {
            for j in 0..3 {
                if number[i][j] == 1 {
                    write!(stdout, 
                        "{}{}  {}", 
                        cursor::Goto(start_pos+2*j as u16, center_height+i as u16), 
                            color::Bg(color::White), style::Reset
                        ).unwrap();
                } else {
                    write!(stdout,
                        "{}  {}",
                        cursor::Goto(start_pos+2*j as u16, center_height+i as u16),
                        style::Reset
                        ).unwrap();
                }
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
