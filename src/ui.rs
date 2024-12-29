use crate::consts::*;

use once_cell::sync::Lazy;

use crate::game::Game;

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
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal
};

pub static BACKGROUND: Lazy<CachedBackground> = Lazy::new(|| CachedBackground::new());

pub struct CachedBackground {
    pub widget: Paragraph<'static>,
    width: u16,
    height: u16,
}

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
        if area.width < SCREEN_WIDTH || area.height < SCREEN_HEIGHT {
            Paragraph::new(Line::from(format!("Terminal must be at least {} x {}!", SCREEN_WIDTH, SCREEN_HEIGHT))).render(area, buf);
            return;
        }

        let elements = build_element_rects(&area);
        let bg_color = Color::Indexed(BACKGROUND_COLOR);

        let block = Block::bordered();
        let board_block = Block::bordered().style(Style::default().fg(Color::White).bg(bg_color));

        let board_style = Style::default().fg(bg_color).bg(bg_color);
        let screen_style = Style::default().fg(bg_color).bg(bg_color);
        let element_style = Style::default().fg(Color::White).bg(bg_color);
        let cell_style = Style::default().fg(Color::White);
        
        draw_element(PRECALC_SCREEN, &elements[RECT_SCREEN], &block, &screen_style, buf);
        
        match self.playing {
            true => {
                match self.paused {
                    false => {
                        draw_element("", &elements[RECT_BOARD], &board_block, &board_style, buf);
                        draw_element(TEXT_STATS, &elements[RECT_STATS], &block, &element_style, buf);
                        draw_element(TEXT_LINES, &elements[RECT_LINES], &block, &element_style, buf);
                        draw_element(TEXT_SCORES, &elements[RECT_SCORES], &block, &element_style, buf);
                        draw_element(TEXT_NEXT, &elements[RECT_NEXT], &block, &element_style, buf);
                        draw_element(TEXT_LEVEL, &elements[RECT_LEVEL], &block, &element_style, buf);

                        let mino = &self.current_mino;
                        let mino_cells = mino.get_rotation();

                        mino_cells.iter().enumerate().for_each(|(y, row)| {
                            row.iter().enumerate().for_each(|(x, cell_value)| {
                                if *cell_value != 0 {
                                    let board_rect = &elements[RECT_BOARD];
                                    let cell_screen_position = (board_rect.x as i8 + self.current_mino_position.0, board_rect.y as i8 + self.current_mino_position.1);
                                    let cell_x_pos = (x as i8* 2) + cell_screen_position.0 + 1;
                                    let cell_y_pos = y as i8 + cell_screen_position.1;
                                    if cell_y_pos <= board_rect.y as i8 { return; }
                                    let cell_rect = Rect::new(cell_x_pos as u16, cell_y_pos as u16, 2, 1);
                                    Paragraph::new(BLOCK).style(cell_style).render(cell_rect, buf);
                                }
                            });
                        });

                    }
                    true => {
                        //draw the pause screen
                        draw_element(BIG_TEXT_PAUSED, &elements[RECT_BIG_TEXT], &block, &element_style, buf);
                    },
                }
            },
            false => {
                match self.paused {
                    false => {
                        //draw the start screen
                        draw_element(BIG_TEXT_TETRIS, &elements[RECT_BIG_TEXT], &block, &element_style, buf);
                    },
                    true => {
                        //draw the high score screen
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

fn build_element_rects(area: &Rect) -> Vec<Rect> {
    let area_center = (area.width / 2, area.height / 2);
    let mut rects: Vec<Rect> = vec![];

    let screen = Rect::new(
        area_center.0 - SCREEN_WIDTH / 2,
        area_center.1 - SCREEN_HEIGHT / 2,
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
    );

    let create_rect = |xy: (u16, u16), width: u16, height: u16| Rect::new(
        screen.x + xy.0 + ELEMENTS_XY.0,
        screen.y + xy.1 + ELEMENTS_XY.1,
        BORDER_WIDTH_PAD + width,
        BORDER_HEIGHT_PAD + height,
    );

    rects.push(create_rect(STATS_XY, STATS_WIDTH, STATS_HEIGHT));
    rects.push(create_rect(LINES_XY, LINES_WIDTH, LINES_HEIGHT));
    rects.push(create_rect(SCORES_XY, SCORES_WIDTH, SCORES_HEIGHT));
    rects.push(create_rect(NEXT_XY, NEXT_WIDTH, NEXT_HEIGHT));
    rects.push(create_rect(BOARD_XY, BOARD_WIDTH, BOARD_HEIGHT));
    rects.push(create_rect(LEVEL_XY, LEVEL_WIDTH, LEVEL_HEIGHT));
    rects.push(create_rect(BIG_TEXT_XY, BIG_TEXT_WIDTH, BIG_TEXT_HEIGHT));
    rects.push(screen);

    rects
}

fn draw_element(text: &str, rect: &Rect, block: &Block, style: &Style, buf: &mut Buffer) {
    Paragraph::new(text)
        .block(block.clone())
        .style(*style)
        .render(*rect, buf);
}