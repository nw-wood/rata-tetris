mod ui;
mod game;
mod minos;
mod consts;

use std::{
    io, 
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
    let app_result = run(terminal);
    ratatui::restore();
    app_result
}

fn run(terminal: DefaultTerminal) -> io::Result<()> {

    let (stop_sender, stop_receiver) = std::sync::mpsc::channel();

    let game_state = Game::new();
    let game_state_clone = game_state.clone();

    let draw_thread_handle = thread::spawn(|| -> io::Result<()> {
        println!("hello from draw thread...");
        draw_ui(terminal, game_state_clone, stop_receiver)?;
        Ok(())
    });

    loop {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                let mut game_state = game_state.lock().unwrap();
                match key.code {
                    KeyCode::Up =>          game_state.slam(),
                    KeyCode::Down =>        game_state.drop_speed_faster(),
                    KeyCode::Left =>        game_state.move_left(),
                    KeyCode::Right =>       game_state.move_right(),
                    KeyCode::PageUp =>      game_state.rotate_left(),
                    KeyCode::PageDown =>    game_state.rotate_right(),
                    KeyCode::Char(' ') => {
                        //key has multiple uses
                        match game_state.playing {
                            true => game_state.toggle_paused(),
                            false => game_state.start_game(),
                        }
                    }
                    KeyCode::Char('q') => {
                        break;
                    },
                    _ => {}
                }
            }
            else if key.kind == KeyEventKind::Release {
                match key.code {
                    KeyCode::Down => game_state.lock().unwrap().drop_speed_normal(),
                    _ => {}
                }
            }
        }
    }

    stop_sender.send(()).unwrap();
    draw_thread_handle.join().unwrap()?;
    Ok(())
}