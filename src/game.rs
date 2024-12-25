use std::fs::File;
use std::io::{self, Read, Write};
use dirs::home_dir;

const TOP_SCORE_FILENAME: &str = "top_score";
const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 20;
/* const T_BLOCK_INDEX: usize = 0;
const J_BLOCK_INDEX: usize = 1;
const Z_BLOCK_INDEX: usize = 2;
const O_BLOCK_INDEX: usize = 3;
const S_BLOCK_INDEX: usize = 4;
const L_BLOCK_INDEX: usize = 5;
const I_BLOCK_INDEX: usize = 6; */

type BlockIndex = usize;
type Score = u32;

#[derive(Debug, Clone)]
pub struct Game {
    line_count: u16,
    statistics: Vec<u16>,
    top_score: u32,
    current_score: u32,
    current_level: u16,
    board_state: Vec<Vec<u8>>,
    playing: bool,
    paused: bool,
}

impl Game {
    pub fn new() -> Self {
        Self {
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
        }
    }

    fn board_insert(&mut self, to_insert: Vec<Vec<u8>>, color: u8) {

    }

    fn increase_lines(&mut self) {
        self.line_count += 1;
    }

    fn increase_stat(&mut self, index: BlockIndex) {
        self.statistics[index] += 1;
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
    }
    pub fn move_right(&mut self) {
        if !self.playing || self.paused { return; }
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