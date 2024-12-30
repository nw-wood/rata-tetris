mod ui;
mod game;
mod minos;
mod consts;

use std::{
    io::{self}, 
    thread
};

use consts::{STATE_GAME_OVER, STATE_PAUSED, STATE_PLAYING, STATE_START_SCREEN};
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

    let game = Game::new();
    let game_clone = game.clone();

    let draw_thread_handle = thread::spawn(|| -> io::Result<()> {
        println!("hello from draw thread...");
        draw_ui(terminal, game_clone, stop_receiver)?;
        Ok(())
    });

    loop {
        if let Event::Key(key) = event::read()? {
            match key.kind {
                KeyEventKind::Press => {
                    let mut game = game.lock().unwrap();
                    match key.code {
                        KeyCode::Up =>          game.slam(),
                        KeyCode::Down =>        game.move_down(),
                        KeyCode::Left =>        game.move_left(),
                        KeyCode::Right =>       game.move_right(),
                        KeyCode::PageUp =>      game.rotate_left(),
                        KeyCode::PageDown =>    game.rotate_right(),
                        KeyCode::Char(' ') => {
                            //key has multiple uses
                            match game.game_state {
                                STATE_START_SCREEN => game.start_game(),
                                STATE_PAUSED => game.toggle_paused(),
                                STATE_PLAYING => game.toggle_paused(),
                                STATE_GAME_OVER => game.new_game(),
                                _ => {}
                            }
                        }
                        KeyCode::Char('q') => {
                            break;
                        },
                        _ => {}
                    }
                }
                _ => {},
            }
        }
    }

    stop_sender.send(()).unwrap();
    draw_thread_handle.join().unwrap()?;
    Ok(())
}