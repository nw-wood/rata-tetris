use std::fs::File;
use std::io::{self, Read, Write};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};
use dirs::home_dir;

use crate::minos::Mino;
use crate::consts::*;

pub struct Game {
    pub line_count: u16,
    pub statistics: Vec<u16>,
    pub top_score: u32,
    pub current_score: u32,
    pub current_level: u8,
    pub board_state: Vec<Vec<u8>>,
    pub game_state: GameState,
    pub current_mino: Mino,
    pub current_mino_position: BoardXY,
    pub next_mino: Mino,
    timer_tx: Sender<u8>,
    pub timer_rx: Receiver<()>,
    pub timer_handle: JoinHandle<()>,
    pub slam_offset: BoardXY,
    pub cleared: bool,
}

impl Game {
    pub fn new() -> Arc<Mutex<Self>> {

        let (timer_tx, timer_receiver) = mpsc::channel();
        let (timer_sender, timer_rx) = mpsc::channel();
        let timer_handle = thread::spawn(move || { game_timer(timer_receiver, timer_sender);});
        timer_tx.send(SIGNAL_PAUSE).unwrap();

        let current_mino = Mino::new(&NO_BLOCK);

        let game = Self {
            line_count: 0,
            current_level: 0,
            current_score: 0,
            top_score: {
                match load_top_score() {
                    Some(score) => score,
                    None => 0,
                }
            },
            statistics: {
                let mut statistics: Vec<u16> = vec![0; MINO_TYPES as usize - 1];
                statistics[current_mino.selected_mino as usize - 1] += 1;
                statistics
            },
            board_state: vec![vec![u8::from(0); GAME_BOARD_WIDTH]; GAME_BOARD_HEIGHT],
            game_state: STATE_START_SCREEN,
            next_mino: Mino::new(&current_mino.selected_mino),
            current_mino_position: current_mino.start_offset,
            current_mino,
            timer_tx,
            timer_rx,
            timer_handle,
            slam_offset: NO_OFFSET,
            cleared: false,
        };

        let game_state = Arc::new(Mutex::new(game));
        game_state
    }

    pub fn start_game(&mut self) {
        self.game_state = STATE_PLAYING;
        self.timer_tx.send(SIGNAL_UNPAUSE).unwrap();
    }
    pub fn new_game(&mut self) {
        self.current_level = 0;
        self.line_count = 0;
        self.statistics = vec![0; 7];
        if self.current_score > self.top_score {
            self.top_score = self.current_score;
        }
        self.current_score = 0;
        
        self.start_game();
    }

    pub fn update(&mut self) {

        self.check_rows();

        let mut drop_count = 0;

        while !&self.timer_rx.try_recv().is_err() {
            drop_count += 1;
        }

        (0..drop_count).for_each(|_| self.move_down());

        //find the next collision to render ghost pieces and enable piece slamming - if needed this can use an optimized collision function
        let mut has_collided = false;
        let mut collision_offset = NO_OFFSET;
        while !has_collided {
            collision_offset.1 += 1;
            has_collided = self.collision(collision_offset, self.current_mino.get_rotation());
        }
        self.slam_offset = (0, collision_offset.1 - 1);
    }

    pub fn collision(&self, direction: BoardXY, rotation: &Rotation) -> bool {

        let new_position: BoardXY = (
            self.current_mino_position.0 + direction.0,
            self.current_mino_position.1 + direction.1
        );

        //check against walls, the floor, filled cells in the current board
        rotation.iter().enumerate().any(|(cell_y, row)| {
            row.iter().enumerate().any(|(cell_x, value)| {
                if *value != 0 {
                    let (board_x_pos, board_y_pos): BoardXY = ((cell_x as i16 * 2) + new_position.0, cell_y as i16 + new_position.1);

                    if board_x_pos < 0 || board_x_pos >= 20 { //left and right walls limits
                        return true;
                    } else if board_y_pos > 20 {//floor limit
                        return true;
                    } else {
                        
                        let mut current_pos = new_position.clone();
                        current_pos.0 += cell_x as i16 * 2;
                        current_pos.1 += cell_y as i16 - 1;
                        current_pos.0 /= 2;

                        if self.board_state[current_pos.1.max(0) as usize][current_pos.0 as usize] != 0 {
                            return true;    
                        }
                        else {
                            return false;
                        }
                    }
                }
                return false;
            })
        })
    }

    fn move_mino(&mut self, change_offset: BoardXY) {
        if !self.collision(change_offset, self.current_mino.get_rotation()) {
            //println!("move: {}, {}", change_offset.0, change_offset.1);
            self.current_mino_position.0 += change_offset.0;
            self.current_mino_position.1 += change_offset.1;
        } else if change_offset == DOWN_OFFSET {
            //now the mino needs placed
            self.place();
            //self.check_rows(); can this be moved???
            self.new_mino();
            if self.collision(DOWN_OFFSET, self.current_mino.get_rotation()) {
                self.game_over();
            }
        }
    }

    fn check_rows(&mut self) {

        let state = self.board_state.clone();
        let mut count = 0;
        state.iter().enumerate().rev().for_each(|(index, row)| {
            if row.iter().all(|cell| *cell != 0) {
                self.board_state.remove(index);
                count += 1;
                self.increase_lines();
            }
        });

        if count > 0 { self.cleared = true };
        (0..count).for_each(|_| { 
            self.board_state.insert(0, vec![u8::from(0); GAME_BOARD_WIDTH]);
        });
        //the base points are multiplied by (level + 1) - if count was 0 no score is added
        let base_score_earned = BASE_SCORES[count];
        let score_earned = (self.current_level as u32 + 1) * base_score_earned;
        self.current_score += score_earned;

        //increase the level
        if self.line_count / 10 != self.current_level as u16 {
            self.increase_level();
            self.timer_tx.send(SIGNAL_INCREASE).unwrap();
        }

    }

    fn new_mino(&mut self) {
        self.increase_stat(self.next_mino.selected_mino as usize);
        self.current_mino = self.next_mino.clone();
        self.next_mino = Mino::new(&self.next_mino.selected_mino);
        self.current_mino_position = self.current_mino.start_offset;
        //okay! bug testing time
    }

    fn place(&mut self) {
        let mino_state = self.current_mino.get_rotation();
        mino_state.iter().enumerate().for_each(|(cell_y, row)| {
            row.iter().enumerate().for_each(|(cell_x, val)| {
                if *val != 0 {
                    let mut current_pos: BoardXY = self.current_mino_position;
                    current_pos.0 += cell_x as i16 * 2;
                    current_pos.1 += cell_y as i16 - 1;
                    current_pos.0 /= 2;
                    self.board_state[current_pos.1.max(0) as usize][current_pos.0 as usize] = self.current_mino.selected_mino;
                }
            });
        });
    }
    
    fn rotate_mino(&mut self, direction: u8) {
        let next_rotation = self.current_mino.next_rotation(direction).clone();
        if !self.collision(NO_OFFSET, &next_rotation) {
            self.current_mino.rotate(direction);
        }
    }
    pub fn move_down(&mut self) {
        if self.game_state != STATE_PLAYING { return; }
        self.move_mino(DOWN_OFFSET);
    }
    pub fn move_left(&mut self) {
        if self.game_state != STATE_PLAYING { return; }
        self.move_mino(LEFT_OFFSET);
    }
    pub fn move_right(&mut self) {
        if self.game_state != STATE_PLAYING { return; }
        self.move_mino(RIGHT_OFFSET);
    }
    pub fn rotate_right(&mut self) {
        if self.game_state != STATE_PLAYING { return; }
        self.rotate_mino(ROT_RIGHT);
    }
    pub fn rotate_left(&mut self) {
        if self.game_state != STATE_PLAYING { return; }
        self.rotate_mino(ROT_LEFT);
    }

    fn increase_lines(&mut self) {
        self.line_count += 1;
    }

    fn increase_stat(&mut self, index: BlockIndex) {
        //this produced a crash, subtracting one since valid block indexes are 1..=7, and statistics is 0..=6; 0 represents empty cells and rerolls in the mino generator
        self.statistics[index - 1] += 1;
    }

    fn increase_level(&mut self) -> u8 {
        self.current_level += 1;
        self.current_level
    }

    fn game_over(&mut self) {
        self.game_state = STATE_GAME_OVER;
        self.timer_tx.send(SIGNAL_RESET).unwrap();
        self.timer_tx.send(SIGNAL_PAUSE).unwrap();
        self.board_state = vec![vec![u8::from(0); GAME_BOARD_WIDTH]; GAME_BOARD_HEIGHT];

        if self.top_score < self.current_score {
            if let Err(e) = save_top_score(self.current_score) {
                println!("couldn't save top score file: {e}");
            }
        }
    }
    //input functions
    pub fn slam(&mut self) {
        if self.game_state != STATE_PLAYING { return; }
        self.move_mino((0, self.slam_offset.1));
        self.move_down();
    }
    pub fn toggle_paused(&mut self) {
        match self.game_state {
            STATE_PAUSED => {
                self.game_state = STATE_PLAYING;
                self.timer_tx.send(SIGNAL_UNPAUSE).unwrap()
            },
            STATE_PLAYING => {
                self.game_state = STATE_PAUSED;
                self.timer_tx.send(SIGNAL_PAUSE).unwrap();
            },
            _ => {}
        }
    }
}

fn load_top_score() -> Option<Score> {
    if let Some(home) = home_dir() {
        let file_path = home.join(TOP_SCORE_FILENAME);
        if !file_path.exists() {
            if let Err(e) = save_top_score(0) {
                println!("couldn't save top score file: {e}");
            }
            return None;
        } else {
            if let Ok(mut file) = File::open(file_path) {
                let mut contents = String::new();
                if let Ok(byte_size) = file.read_to_string(&mut contents) {
                    println!("loaded {byte_size} bytes from file");
                    if let Ok(score) = contents.trim().parse::<u32>() {
                        return Some(score);
                    }
                }
            }
        }
    }
    None
}

fn save_top_score(score: Score) -> io::Result<()> {
    if let Some(home) = home_dir() {
        let file_path = home.join(TOP_SCORE_FILENAME);
        let mut file = File::create(file_path)?;
        file.write(score.to_string().as_bytes())?;
    }
    Ok(())
}

fn get_drop_time_duration(level: u8) -> u128 {
    let frames = match level {
        0 => GRAVITY_TABLE[0],
        1 => GRAVITY_TABLE[1],
        2 => GRAVITY_TABLE[2],
        3 => GRAVITY_TABLE[3],
        4 => GRAVITY_TABLE[4],
        5 => GRAVITY_TABLE[5],
        6 => GRAVITY_TABLE[6],
        7 => GRAVITY_TABLE[7],
        8 => GRAVITY_TABLE[8],
        9 => GRAVITY_TABLE[9],
        10..=12 => GRAVITY_TABLE[10],
        13..=15 => GRAVITY_TABLE[11],
        16..=18 => GRAVITY_TABLE[12],
        19..=28 => GRAVITY_TABLE[13],
        _ => GRAVITY_TABLE[14],
    };
    17 * frames as u128 //off by .33 per millisecond 🤷
}

struct Timer {
    level: u8,
    duration: u128,
}

impl Timer {
    fn new(start: u8) -> Self {
        Self {
            level: start,
            duration: get_drop_time_duration(start),
        }
    }

    fn increase(&mut self) {
        self.level += 1;
        self.duration = get_drop_time_duration(self.level);
    }
}

//when a game is created this timer is set to run in it's own thread seperate from the game input and draw calls
fn game_timer(timer_receiver: Receiver<u8>, timer_sender: Sender<()>) {

    let mut time = Instant::now();
    let mut timer = Timer::new(0);

    'timer: loop {
        thread::sleep(Duration::from_millis(16));
        let elapsed = time.elapsed().as_millis();
        if elapsed >= timer.duration {
            if let Ok(signal) =  timer_receiver.try_recv() {
                match signal {
                    SIGNAL_INCREASE => timer.increase(),
                    SIGNAL_PAUSE => {
                        loop {
                            thread::sleep(Duration::from_millis(250)); //recheck every quarter second
                            if let Ok(signal) = timer_receiver.try_recv() {
                                match signal {
                                    SIGNAL_UNPAUSE => break,
                                    SIGNAL_KILL => break 'timer,
                                    SIGNAL_RESET => {timer = Timer::new(0)},
                                    _ => {}
                                }
                            }
                        }
                    }
                    SIGNAL_KILL => break 'timer,
                    SIGNAL_RESET => {timer = Timer::new(0)},
                    _ => {},
                }
            }
            time = Instant::now();
            timer_sender.send(SIGNAL_DROP).unwrap();
        }
    }    
}
