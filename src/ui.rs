use crate::consts::*;

use once_cell::sync::Lazy;

use crate::game::Game;

use std::{
    io, sync::{mpsc::Receiver, Arc, Mutex}, thread, time::Duration
};

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Offset, Rect},
    style::{Color, Style, Stylize},
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
                                1 => Style::default().fg(Color::Indexed(237)),
                                2 => Style::default().fg(Color::Indexed(238)),
                                3 => Style::default().fg(Color::Indexed(236)),
                                4 => Style::default().fg(Color::Indexed(235)),
                                5 => Style::default().fg(Color::Indexed(237)),
                                6 => Style::default().fg(Color::Indexed(235)),
                                7 => Style::default().fg(Color::Indexed(239)),
                                _ => Style::default().fg(Color::Indexed(235)),
                            };
                            Span::styled(BLOCK, style)
                        })
                        .collect::<Vec<Span>>(),
                )
            })
            .collect();
        Self {
            widget: Paragraph::new(lines),
            width: ((background_pattern[0].len() - 1) * 2) as u16,
            height: background_pattern.len() as u16 - 1,
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
        let bg_color = Color::Indexed(BACKGROUND_COLOR);
        let alert_block = Block::bordered().style(Style::default().fg(Color::DarkGray).bg(bg_color));

        //if the terminal is too small draw a message for now
        if area.width < SCREEN_WIDTH || area.height < SCREEN_HEIGHT {
            Paragraph::new(format!("Terminal must be at least {} x {}! \n\n{}", SCREEN_WIDTH, SCREEN_HEIGHT, ZOOM_TIP_TEXT)).block(alert_block).bold().render(area, buf);
            return;
        }
        //helper function to assemble interface rects for reference
        let elements = build_element_rects(&area);

        //define some style rules
        let board_block = Block::bordered().style(Style::default().fg(Color::White).bg(bg_color));
        let bg_color = Color::Indexed(BACKGROUND_COLOR);
        let block = Block::bordered();
        let board_block = Block::bordered().style(Style::default().fg(Color::White).bg(bg_color));
        let block_no_border = Block::new();
        let board_style = Style::default().fg(bg_color).bg(bg_color);
        let screen_style = Style::default().fg(bg_color).bg(bg_color);
        let element_style = Style::default().fg(Color::White).bg(bg_color);
        
        //draw the play area background, and the controls text at the bottom
        draw_element(PRECALC_SCREEN, &elements[RECT_SCREEN], &block, &screen_style, buf);
        draw_element(CONTROLS_TEXT, &elements[RECT_CONTROLS], &block_no_border, &element_style, buf);

        //depending on the game state draw a different version of the screen
        match self.game_state {
            STATE_PLAYING => {
                //draw board background, next piece background, and stats background
                draw_element(TEXT_NEXT, &elements[RECT_NEXT], &block, &element_style, buf);
                draw_element(TEXT_STATS, &elements[RECT_STATS], &block, &element_style, buf);
                draw_element("", &elements[RECT_BOARD], &board_block, &board_style, buf);

                //fill out line count, current and top scores, and the current level elements
                draw_element(&format!("{}{:03}     ", TEXT_LINES, self.line_count), &elements[RECT_LINES], &block, &element_style, buf);
                draw_element(&format!("\n {}\n     {:06} \n\n {}\n     {:06} \n ", "TOP", self.top_score, "SCORE", self.current_score), &elements[RECT_SCORES], &block, &element_style, buf);
                draw_element(&format!("{}{:02}  ", TEXT_LEVEL, self.current_level), &elements[RECT_LEVEL], &block, &element_style, buf);                

                //iteate through and draw each stat item, and it's corresponding counted value
                let stats_boxes = Layout::new(
                    Direction::Vertical,
                    Constraint::from_maxes([3, 3, 3, 3, 3, 3, 2]),
                )
                .split(elements[RECT_STATS_INSET]);
                stats_boxes.iter().enumerate().for_each(|(index, rect)| {
                    let mino_style = mino_to_styling(index as u8 + 1, self.current_level);
                    let number_display_box = Layout::new(Direction::Horizontal, Constraint::from_percentages([60, 40]))
                        .split(*rect)[1];
                    draw_element(mino_style.0.as_str(), rect, &block_no_border, &mino_style.1, buf);
                    draw_element(&format!(" {:03}", self.statistics[index]), &number_display_box, &block_no_border, &element_style, buf);
                });

                //iterate through and draw the cells on the board
                self.board_state.iter().enumerate().for_each(|(cell_y, row)| {
                    row.iter().enumerate().for_each(|(cell_x, value)| {
                        let board_rect = &elements[RECT_BOARD];

                        let cell_rect = Rect::new(
                            board_rect.x + (cell_x as u16 * 2) + 1,
                            board_rect.y + cell_y as u16 + 1,
                            2,
                            1,
                        );
        
                        let style = mino_to_styling(*value, self.current_level);
                        Paragraph::new(BLOCK).style(style.1).render(cell_rect, buf);
                    });
                });
                
                //draw the next piece to the next inset
                let next_mino_id = self.next_mino.selected_mino;
                let next_mino_style = mino_to_styling(next_mino_id, self.current_level);
                draw_element(next_mino_style.0.as_str(), &elements[RECT_NEXT_INSET], &block_no_border, &next_mino_style.1, buf);                

                //draw the current falling mino onto the screen
                self.current_mino.get_rotation().iter().enumerate().for_each(|(y, row)| {
                    row.iter().enumerate().for_each(|(x, value)| {
                        if *value != 0 {
                            let board_rect = &elements[RECT_BOARD];

                            let cell_screen_position: BoardXY = (
                                board_rect.x as i16 + self.current_mino_position.0,
                                board_rect.y as i16 + self.current_mino_position.1,
                            );

                            let cell_rect = Rect::new(
                                ((x as i16 * 2) + cell_screen_position.0 as i16 + 1).max(0) as u16,
                                (y as i16 + cell_screen_position.1 as i16).max(0) as u16,
                                2,
                                1
                            );

                            
                            let slamming_by = self.slam_offset.1 as u16;
                            let ghost_rect = cell_rect.offset(Offset { x: 0, y: slamming_by as i32 });
                            let style = mino_to_styling(self.current_mino.selected_mino, self.current_level);
                            draw_element(BLOCK, &ghost_rect, &block, &style.1, buf); //this accidentally made a really nice colored ghost piece out of borders amusingly
                            if cell_rect.y <= board_rect.y { return; } //don't draw pieces if off screen
                            draw_element(BLOCK, &cell_rect, &block_no_border, &style.1, buf);
                            
                        }
                    });
                });
            }
            //draw corresponding game screens for the other states
            STATE_PAUSED => draw_element(BIG_TEXT_PAUSED, &elements[RECT_BIG_TEXT], &block, &element_style, buf),
            STATE_START_SCREEN => {
                draw_element(BIG_TEXT_TETRIS, &elements[RECT_BIG_TEXT], &block, &element_style, buf);
                draw_element(ZOOM_TIP_TEXT, &elements[RECT_ZOOM_TIP], &block, &Style::new().bg(bg_color).fg(Color::DarkGray), buf);
            },
            STATE_GAME_OVER => {
                
                let score_message = if self.current_score > self.top_score {
                    let message = format!("🎆 New top score set at {}! 🎇", self.current_score);
                    format!("{message:^SCORE_PADDING$}") // ?
                } else {
                    let message = format!("You scored {} points.", self.current_score);
                    format!("{message:^SCORE_PADDING$}")
                };

                let stats_uncentered = format!("Reach level {}, and cleared {} lines.", self.current_level, self.line_count);
                let stats_message = format!("{stats_uncentered:^SCORE_PADDING$}");

                draw_element(GAME_OVER_TEXT, &elements[RECT_GAME_OVER_TEXT], &block, &element_style, buf);
                draw_element(score_message.as_str(), &elements[RECT_NEW_TOP_SCORE], &block, &element_style, buf);
                draw_element(stats_message.as_str(), &elements[RECT_GAME_OVER_STATS], &block, &element_style, buf);

            }
            _ => {}
        }
    }
}

pub fn draw_ui(
	mut terminal: DefaultTerminal, 
	game_state: Arc<Mutex<Game>>, 
	stop_receiver: Receiver<()>
) -> io::Result<()> {
    loop {
        thread::sleep(Duration::from_millis(16));
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
    rects.push(create_rect(GAME_OVER_TEXT_XY, GAME_OVER_TEXT_WIDTH, GAME_OVER_TEXT_HEIGHT));
    rects.push(create_rect(CONTROL_XY, CONTROLS_WIDTH, CONTROLS_HEIGHT));
    rects.push(create_rect(STATS_INSET_XY, STATS_INSET_WIDTH, STATS_INSET_HEIGHT));
    rects.push(create_rect(NEXT_INSET_XY, NEXT_INSET_WIDTH, NEXT_INSET_HEIGHT));
    rects.push(create_rect(GAME_OVER_STATS_XY, GAME_OVER_STATS_WIDTH, GAME_OVER_STATS_HEIGHT));
    rects.push(create_rect(NEW_TOP_SCORE_XY, NEW_TOP_SCORE_WIDTH, NEW_TOP_SCORE_HEIGHT));
    rects.push(create_rect(ZOOM_TIP_XY, ZOOM_TIP_WIDTH, ZOOM_TIP_HEIGHT));

    rects
}

fn draw_element(text: &str, rect: &Rect, block: &Block, style: &Style, buf: &mut Buffer) {
    //background fill fix... should no longer need to append spaces 🧐 ...works good!
    let text_padded = text.lines().map(|line| {
        let line_width = line.len();
        let pad_needed = (rect.width as isize - line_width as isize).max(0) as usize;
        let padding = " ".repeat(pad_needed);
        format!("{}{}", line, padding)
    }).collect::<Vec<String>>().join("\n");

    Paragraph::new(text_padded)
        .block(block.clone())
        .style(*style)
        .render(*rect, buf);
}

fn mino_to_styling(id: u8, current_level: u8) -> (String, Style) {

    let mino_text = match id {
        I_BLOCK => TEXT_MINO_I,
        J_BLOCK => TEXT_MINO_J,
        L_BLOCK => TEXT_MINO_L,
        O_BLOCK => TEXT_MINO_O,
        Z_BLOCK => TEXT_MINO_Z,
        S_BLOCK => TEXT_MINO_S,
        T_BLOCK => TEXT_MINO_T,
        _ => "",
    };

    let palette = [
            PALETTE_BLURPLE,
            PALETTE_LIME,
            PALETTE_PINK,
            PALETTE_SWAMP,
            PALETTE_MELON,
            PALETTE_LAKE,
            PALETTE_FACTORY,
            PALETTE_MUAVE,
            PALETTE_NARU,
            PALETTE_CREAM
        ][(current_level % 10) as usize];

    let bg_color = Color::Indexed(BACKGROUND_COLOR);
    let fg_color = if id == 0 { bg_color } else { Color::Indexed(palette[id as usize - 1]) };

    (mino_text.to_string(), Style::default().fg(fg_color).bg(bg_color))
}