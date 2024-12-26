use std::fs::File;
use std::io::{self, Read, Write};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};
use dirs::home_dir;

use crate::minos::Mino;

const TOP_SCORE_FILENAME: &str = "top_score";
const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 20;

const LEFT_OFFSET: (i8, i8) = (-1, 0);
const RIGHT_OFFSET: (i8, i8) = (1, 0);
const DOWN_OFFSET: (i8, i8) = (0, 1); //higher = further down the board

const SIGNAL_INCREASE: u8 = 1;
const SIGNAL_PAUSE: u8 = 2;
const SIGNAL_UNPAUSE: u8 = 3;
const SIGNAL_KILL: u8 = 4;
const SIGNAL_DROP: () = ();

//'frame counts' for the level difficulties
const GRAVITY_TABLE: [u8; 15] = [48, 43, 38, 33, 28, 23, 18, 13, 8, 6, 5, 4, 3, 2, 1];
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

type BlockIndex = usize;
type Score = u32;
//considering making this it's own type.. signed i8's also makes the logic a bit more simple
type BoardXY = (i8, i8);

//#[derive(Debug, Clone)]
pub struct Game {
    line_count: u16,
    statistics: Vec<u16>,
    top_score: u32,
    current_score: u32,
    current_level: u8,
    board_state: Vec<Vec<u8>>,
    playing: bool,
    paused: bool,
    current_mino: Mino,
    current_mino_position: BoardXY,
    next_mino: Mino,
    timer_tx: Sender<u8>,
    pub timer_rx: Receiver<()>,
    pub timer_handle: JoinHandle<()>,
}

impl Game {
    pub fn new() -> Arc<Mutex<Self>> {

        //setup senders and receivers
        let (game_sender, timer_receiver) = mpsc::channel();
        let (timer_sender, game_receiver) = mpsc::channel();

        //game timer
        let handle = thread::spawn(move || {
            let mut time = Instant::now();
            let mut current_level = 0;
            let mut duration = get_drop_time_duration(current_level);
            'timer: loop {
                let elapsed = time.elapsed().as_micros();
                if elapsed >= duration {
                    if let Ok(signal) =  timer_receiver.try_recv() {
                        match signal {
                            SIGNAL_INCREASE => {
                                current_level += 1;
                                duration = get_drop_time_duration(current_level);
                            },
                            SIGNAL_PAUSE => {
                                loop {
                                    thread::sleep(Duration::from_millis(250)); //recheck every quarter second
                                    if let Ok(signal) = timer_receiver.try_recv() {
                                        match signal {
                                            SIGNAL_UNPAUSE => break,
                                            SIGNAL_KILL => break 'timer,
                                            _ => {}
                                        }
                                    }
                                }
                            }
                            SIGNAL_KILL => break 'timer,
                            _ => {},
                        }
                    }
                    time = Instant::now();
                    timer_sender.send(SIGNAL_DROP).unwrap();
                }
            }    
        });

        let game = Self {
            line_count: 0,
            statistics: vec![0; 7],
            top_score: {
                match load_top_score() {
                    Some(score) => score,
                    None => 0,
                }
            },
            current_level: 0,
            current_score: 0,
            board_state: vec![vec![u8::from(0); BOARD_WIDTH]; BOARD_HEIGHT],
            playing: false,
            paused: false,
            current_mino: Mino::new(),
            next_mino: Mino::new(),
            current_mino_position: (0, 0),
            timer_tx: game_sender,
            timer_rx: game_receiver,
            timer_handle: handle,    
        };

        let game_state = Arc::new(Mutex::new(game));
        game_state
    }

    fn move_mino(&mut self, change_offset: BoardXY) {
        self.current_mino_position.0 += change_offset.0;
        self.current_mino_position.1 += change_offset.1;
    }

    fn place_mino(&mut self) {
        let type_fill = self.current_mino.selected_mino;
        self.current_mino = self.next_mino.clone();
        self.next_mino = Mino::new();
    }

    fn check_collision(& mut self) {
        let rotation = self.current_mino.get_rotation();
        /*working out collision handling and game time passing logic still*/
        if false == true {
            self.place_mino();
        } else {
            self.move_down();
        }
    }

    fn increase_lines(&mut self) {
        self.line_count += 1;
    }

    fn increase_stat(&mut self, index: BlockIndex) {
        self.statistics[index] += 1;
    }

    fn increase_level(&mut self) -> u8 {
        self.current_level += 1;
        self.current_level
    }

    fn set_top_score(&mut self) {
        if let Some(top_score) = load_top_score() {
            self.top_score = top_score;
        } else {
            self.top_score = 0;
        }
    }

    fn game_over(&mut self) {
        self.current_level = 0;
        self.line_count = 0;
        self.statistics = vec![0; 7];

        if self.top_score < self.current_score {
            if let Err(e) = save_top_score(self.current_score) {
                println!("couldn't save top score file: {e}");
            }
            self.top_score = self.current_score;
        }

        self.current_score = 0;
        //doesn't change self.top_score = 0; 
    }

    pub fn move_down(&mut self) {
        self.move_mino(DOWN_OFFSET);
    }

    pub fn update(&mut self) {
        let mut drop_count = 0;
        for _ in &self.timer_rx {
            drop_count += 1;
        }
        for _ in 0..drop_count {
            self.move_down();
        }
    }

    //input functions
    pub fn slam(&mut self) {
        if !self.playing || self.paused { return; }
    }
    pub fn drop_speed_faster(&mut self) {
        if !self.playing || self.paused { return; }
    }
    pub fn drop_speed_normal(&mut self) {
        if !self.playing || self.paused { return; }
    }
    pub fn move_left(&mut self) {
        if !self.playing || self.paused { return; }
        self.move_mino(LEFT_OFFSET);
    }
    pub fn move_right(&mut self) {
        if !self.playing || self.paused { return; }
        self.move_mino(RIGHT_OFFSET);
    }
    pub fn rotate_clockwise(&mut self) {
        if !self.playing || self.paused { return; }
    }
    pub fn rotate_counter_clockwise(&mut self) {
        if !self.playing || self.paused { return; }
    }
    pub fn toggle_paused(&mut self) {
        if !self.playing { return; }
        self.paused = !self.paused;
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