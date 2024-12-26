use once_cell::sync::Lazy;

use crate::game::{self, Game};

use std::{
    io, 
    sync::{mpsc::Receiver, Arc, Mutex}, 
    thread, 
    time::Duration
};

use ratatui::{
    buffer::Buffer, 
    layout::Rect, 
    style::{Color, Style}, 
    text::{Line, Span}, 
    widgets::{Paragraph, Widget}, DefaultTerminal
};

pub static BACKGROUND: Lazy<CachedBackground> = Lazy::new(|| CachedBackground::new());

pub struct CachedBackground {
    pub widget: Paragraph<'static>,
    width: u16,
    height: u16,
}

impl CachedBackground {
    pub fn new() -> Self {
        let bg_char = "██";
        let background_pattern = vec![
            vec![1, 1, 2, 1, 1, 1, 3, 3, 4, 4, 1, 1],
            vec![3, 3, 2, 2, 1, 2, 3, 3, 4, 1, 5, 1],
            vec![3, 3, 5, 2, 1, 2, 2, 1, 1, 1, 5, 1],
            vec![6, 6, 5, 3, 3, 4, 2, 6, 6, 6, 5, 6],
            vec![3, 6, 5, 3, 3, 4, 4, 2, 2, 6, 5, 3],
            vec![3, 4, 5, 1, 1, 4, 2, 2, 3, 3, 2, 3],
            vec![4, 4, 4, 7, 1, 3, 3, 5, 3, 3, 2, 2],
            vec![3, 3, 7, 7, 1, 3, 3, 5, 4, 4, 4, 2],
            vec![3, 3, 7, 1, 6, 6, 6, 5, 6, 4, 7, 7],
            vec![7, 1, 1, 1, 7, 7, 6, 5, 6, 5, 5, 7],
            vec![1, 5, 6, 6, 6, 7, 7, 6, 6, 5, 3, 3],
            vec![1, 5, 5, 5, 6, 1, 1, 1, 4, 5, 3, 3],
        ];
        let lines: Vec<Line> = background_pattern
            .iter()
            .map(|row| {
                Line::from(
                    row.iter()
                        .map(|&cell| {
                            let style = match cell {
                                1 => Style::default().fg(Color::Indexed(241)),
                                2 => Style::default().fg(Color::Indexed(240)),
                                3 => Style::default().fg(Color::Indexed(242)),
                                4 => Style::default().fg(Color::Indexed(243)),
                                5 => Style::default().fg(Color::Indexed(244)),
                                6 => Style::default().fg(Color::Indexed(239)),
                                7 => Style::default().fg(Color::Indexed(240)),
                                _ => Style::default().fg(Color::Indexed(232)),
                            };
                            Span::styled(bg_char, style)
                        })
                        .collect::<Vec<Span>>(),
                )
            })
            .collect();
        Self {
            widget: Paragraph::new(lines),
            width: (background_pattern[0].len() * 2) as u16,
            height: background_pattern.len() as u16,
        }
    }
}

fn calc_tile_size(tile_start: u16, total_size: u16, max_tile_size: u16) -> u16 {
    let remaining_size = total_size.saturating_sub(tile_start);
    u16::min(max_tile_size, remaining_size)
}

impl Widget for &CachedBackground {
    fn render(self, area: Rect, buf: &mut Buffer) {

        let x_tiles = area.width / self.width + 1;
        let y_tiles = area.height / self.height + 1;

        for y_tile in 0..y_tiles {

            let y_start = y_tile * self.height;
            let tile_h = calc_tile_size(y_start, area.height, self.height);

            for x_tile in 0..x_tiles {

                let x_start = x_tile * self.width;
                let tile_w = calc_tile_size(x_start, area.width, self.width);
                
                let tile_rect = Rect::new(
	                x_start,
	                y_start, 
	                tile_w,
	                tile_h
	            );
                self.widget.clone().render(tile_rect, buf);
            }
        }
    }
}

impl Widget for &Game {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized
    {
        //render the game state
    }
}

pub fn draw_ui(
	mut terminal: DefaultTerminal, 
	game_state: Arc<Mutex<Game>>, 
	stop_receiver: Receiver<()>
) -> io::Result<()> {
    loop {
        thread::sleep(Duration::from_millis(100));

        match stop_receiver.try_recv() {
            Err(std::sync::mpsc::TryRecvError::Empty) => {}, 
            _ => break,
        }

        terminal.draw(|frame| {
            frame.render_widget(&*BACKGROUND, frame.area());
            frame.render_widget(&*game_state.lock().unwrap(), frame.area());
        })
        .map(|_| ())?;
    }
    Ok(())
}