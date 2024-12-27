const BIG_TEXT_TETRIS: &str = r#"  ██████  ████  ██████  ████    ██    ████  
    ██    ██      ██    ██  ██  ██  ██      
    ██    ████    ██    ████    ██    ████  
    ██    ██      ██    ████    ██        ██
    ██    ████    ██    ██  ██  ██    ████  "#;

const BIG_TEXT_PAUSED: &str = r#"  ██████  ██████  ██  ██    ████    ██████  ████
  ██  ██  ██  ██  ██  ██  ██        ██      ██  ██
  ██████  ██████  ██  ██    ████    ████    ██  ██
  ██      ██  ██  ██  ██        ██  ██      ██  ██
  ██      ██  ██  ██████    ████    ██████  ████
"#;

use crossterm::style::Stylize;
use once_cell::sync::Lazy;

use crate::game::{self, Game};

use std::{
    io, 
    sync::{mpsc::Receiver, Arc, Mutex}, 
    thread, 
    time::Duration
};

use ratatui::{
    buffer::Buffer, layout::{Constraint, Direction, Layout, Rect}, style::{Color, Style, Styled}, symbols::line, text::{Line, Span, Text}, widgets::{Block, Paragraph, Widget}, DefaultTerminal
};

pub static BACKGROUND: Lazy<CachedBackground> = Lazy::new(|| CachedBackground::new());

pub struct CachedBackground {
    pub widget: Paragraph<'static>,
    width: u16,
    height: u16,
}

const BLOCK: &str = "██";
const SCREEN_WIDTH: u16 = 32 * 2; // x 2 since each cell is 2 chars per block
const SCREEN_HEIGHT: u16 = 28;

impl CachedBackground {
    pub fn new() -> Self {
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
                            Span::styled(BLOCK, style)
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
        //if the terminal is too small draw a notice to the screen
        if area.width < SCREEN_WIDTH || area.height < SCREEN_HEIGHT {
            Paragraph::new(Line::from(format!("Terminal must be at least {} x {}!", SCREEN_WIDTH, SCREEN_HEIGHT))).render(area, buf);
            return;
        }

        //build a rect for the screen space that's in the center of the terminal
        let area_center: (u16, u16) = (area.width / 2,  area.height / 2);
        let screen = Rect::new(
            area_center.0 - SCREEN_WIDTH / 2,
            area_center.1 - SCREEN_HEIGHT / 2,
            SCREEN_WIDTH,
            SCREEN_HEIGHT
        );

        //build a widget out of a collection of lines built from spans made of block strings
        let line_blocks = (0..SCREEN_WIDTH / 2).map(|_| { BLOCK }).collect::<String>();
        let screen_space_widget = Paragraph::new(
            (0..SCREEN_HEIGHT).map(|_| {
                Span::styled(&line_blocks, Style::default().bg(Color::Black).fg(Color::Black)
            )}).map(|span| {
                Line::from(span)
            }).collect::<Vec<Line>>()
        );

        //render the new widget to the screen rect
        screen_space_widget.render(screen, buf);

        match self.playing {
            true => {
                match self.paused {
                    false => {
                        //the user interface should be drawn
                    }
                    true => {
                        //draw pause screen
                        let big_text_area = Rect::new(screen.x + 9, screen.y + 3, 46, 7);
                        let big_text_widget = Paragraph::new(Text::from(BIG_TEXT_PAUSED))
                            .block(Block::bordered())
                            .style(Style::default().fg(Color::White).bg(Color::Black));
                        big_text_widget.render(big_text_area, buf);
                    },
                }
            },
            false => {
                match self.paused {
                    false => {
                        //draw start screen
                        let big_text_area = Rect::new(screen.x + 9, screen.y + 3, 46, 7);
                        let big_text_widget = Paragraph::new(Text::from(BIG_TEXT_TETRIS))
                            .block(Block::bordered())
                            .style(Style::default().fg(Color::White).bg(Color::Black));
                        big_text_widget.render(big_text_area, buf);
                    },
                    true => {
                    },
                }
            }
        }
    }
}

pub fn draw_ui(
	mut terminal: DefaultTerminal, 
	game_state: Arc<Mutex<Game>>, 
	stop_receiver: Receiver<()>
) -> io::Result<()> {
    let mut frame_count: usize = 0;
    loop {
        //60 frames per second hard limit
        thread::sleep(Duration::from_millis(16));
        frame_count += 1;
        //println!("frame_count: {frame_count}");
        match stop_receiver.try_recv() {
            Err(std::sync::mpsc::TryRecvError::Empty) => {}, 
            _ => break,
        }

        terminal.draw(|frame| {
            frame.render_widget(&*BACKGROUND, frame.area());
            let mut game = game_state.lock().unwrap();
            game.update();
            frame.render_widget(&*game, frame.area());
        })
        .map(|_| ())?;
    }
    Ok(())
}