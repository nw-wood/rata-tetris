mod ui;
mod game;

use std::{
    io, 
    sync::{Arc, Mutex}, 
    thread
};

use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    DefaultTerminal,
};

use ui::draw_ui;

use game::Game;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app_result = run(Arc::new(Mutex::new(terminal)));
    ratatui::restore();
    app_result
}

fn run(terminal: Arc<Mutex<DefaultTerminal>>) -> io::Result<()> {

    let arc_terminal = terminal.clone();
    let (stop_sender, stop_receiver) = std::sync::mpsc::channel();

    let mut game = Game::new();
    let arc_game_state = Arc::new(Mutex::new(game.clone()));

    let draw_thread_handle = thread::spawn(|| -> io::Result<()> {
        draw_ui(arc_terminal, arc_game_state, stop_receiver)?;
        Ok(())
    });

    loop {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Up =>          game.slam(),
                    KeyCode::Down =>        game.drop_speed_faster(),
                    KeyCode::Left =>        game.move_left(),
                    KeyCode::Right =>       game.move_right(),
                    KeyCode::PageUp =>      game.rotate_counter_clockwise(),
                    KeyCode::PageDown =>    game.rotate_clockwise(),
                    KeyCode::Char(' ') =>   game.toggle_paused(),
                    KeyCode::Char('q') => {
                        stop_sender.send(()).unwrap();
                        draw_thread_handle.join().unwrap()?;
                        break;
                    },
                    _ => {}
                }
            }
            else if key.kind == KeyEventKind::Release {
                match key.code {
                    KeyCode::Down => game.drop_speed_normal(),
                    _ => {}
                }
            }
        }
    }
    Ok(())
}