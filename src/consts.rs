//common type aliases
pub type Rotation = Vec<Vec<u8>>;
pub type Score = u32;
pub type BoardXY = (i8, i8);
pub type BlockIndex = usize;

/* Base points (at level 0):
Single: 40 points
Double: 100 points
Triple: 300 points
Tetris (4 lines): 1200 points */

pub const BASE_SCORES: [u32; 5] = [0, 40, 100, 300, 1200];

pub const J_BLOCK: u8 = 1;
pub const Z_BLOCK: u8 = 2;
pub const O_BLOCK: u8 = 3;
pub const S_BLOCK: u8 = 4;
pub const L_BLOCK: u8 = 5;
pub const I_BLOCK: u8 = 6;
pub const T_BLOCK: u8 = 7;

pub const STATE_PLAYING: u8 = 0;
pub const STATE_PAUSED: u8 = 1;
pub const STATE_GAME_OVER: u8 = 2;
pub const STATE_START_SCREEN: u8 = 3;
pub type GameState = u8;

pub const REROLL: u8 = 0;
pub const MINO_TYPES: u8 = 8;
pub const NO_BLOCK: u8 = 8;

pub const TOP_SCORE_FILENAME: &str = "top_score";

//x and y for the actual play area
pub const GAME_BOARD_WIDTH: usize = 10;
pub const GAME_BOARD_HEIGHT: usize = 20;

//directional offsets for the games movement methods
pub const LEFT_OFFSET: (i8, i8) = (-2, 0);
pub const RIGHT_OFFSET: (i8, i8) = (2, 0);
pub const DOWN_OFFSET: (i8, i8) = (0, 1);
pub const NO_OFFSET: (i8, i8) = (0, 0);

//match clarity
pub const ROT_LEFT: u8 = 0;
pub const ROT_RIGHT: u8 = 1;

//signal identifiers for the game timer thread
pub const SIGNAL_INCREASE: u8 = 1;
pub const SIGNAL_PAUSE: u8 = 2;
pub const SIGNAL_UNPAUSE: u8 = 3;
pub const SIGNAL_KILL: u8 = 4;
pub const SIGNAL_DROP: () = ();

//frame counts for level difficulty
pub const GRAVITY_TABLE: [u8; 15] = [48, 43, 38, 33, 28, 23, 18, 13, 8, 6, 5, 4, 3, 2, 1];

//user interface styling, text, borders, and rect identifiers - etc...
pub const BIG_TEXT_TETRIS: &str = r#"  ██████  ████  ██████  ████    ██    ████    ██  
    ██    ██      ██    ██  ██  ██  ██        ██  
    ██    ████    ██    ████    ██    ████    ██  
    ██    ██      ██    ████    ██        ██      
    ██    ████    ██    ██  ██  ██    ████    ██  "#;

pub const BIG_TEXT_PAUSED: &str = r#" ██████  ██████  ██  ██    ████    ██████  ████   
 ██  ██  ██  ██  ██  ██  ██        ██      ██  ██ 
 ██████  ██████  ██  ██    ████    ████    ██  ██ 
 ██      ██  ██  ██  ██        ██  ██      ██  ██ 
 ██      ██  ██  ██████    ████    ██████  ████   
"#;

pub const GAME_OVER_TEXT: &str = 
r#"   ████      ██      ██  ██    ██████             
 ██        ██  ██  ██  ██  ██  ██                 
 ██  ████  ██████  ██      ██  ████               
 ██    ██  ██  ██  ██      ██  ██                 
   ████    ██  ██  ██      ██  ██████             
                                                  
               ██    ██  ██  ██████  ██████    ██ 
             ██  ██  ██  ██  ██      ██    ██  ██ 
             ██  ██  ██  ██  ████    ██    ██  ██ 
             ██  ██  ██  ██  ██      ██████       
               ██      ██    ██████  ██    ██  ██ 
"#;
pub const CONTROLS_TEXT: &str = " start: space quit: q rot: pgup/dn move: ←→ slam: ↑ drop: ↓"; 

pub const PRECALC_SCREEN: &str = r#"████████████████████████████████████████████████████████████████
████████████████████████████████████████████████████████████████
████████████████████████████████████████████████████████████████
████████████████████████████████████████████████████████████████
████████████████████████████████████████████████████████████████
████████████████████████████████████████████████████████████████
████████████████████████████████████████████████████████████████
████████████████████████████████████████████████████████████████
████████████████████████████████████████████████████████████████
████████████████████████████████████████████████████████████████
████████████████████████████████████████████████████████████████
████████████████████████████████████████████████████████████████
████████████████████████████████████████████████████████████████
████████████████████████████████████████████████████████████████
████████████████████████████████████████████████████████████████
████████████████████████████████████████████████████████████████
████████████████████████████████████████████████████████████████
████████████████████████████████████████████████████████████████
████████████████████████████████████████████████████████████████
████████████████████████████████████████████████████████████████
████████████████████████████████████████████████████████████████
████████████████████████████████████████████████████████████████
████████████████████████████████████████████████████████████████
████████████████████████████████████████████████████████████████
████████████████████████████████████████████████████████████████
████████████████████████████████████████████████████████████████
████████████████████████████████████████████████████████████████"#;



pub const BORDER_WIDTH_PAD: u16 = 2;
pub const BORDER_HEIGHT_PAD: u16 = 1;

//remember* when adding rects width, height, xy, and rect below - remember to push the rect into elements in ui!
pub const CONTROLS_WIDTH: u16 = SCREEN_WIDTH - 7;
pub const CONTROLS_HEIGHT: u16 = 0;
pub const CONTROL_XY: (u16, u16) = (0, SCREEN_HEIGHT - 2);

pub const STATS_WIDTH: u16 = 18;
pub const STATS_HEIGHT: u16 = 18;
pub const STATS_XY: (u16, u16) = (2, 5);

pub const LINES_WIDTH: u16 = 20;
pub const LINES_HEIGHT: u16 = 2;
pub const LINES_XY: (u16, u16) = (22, 0);

pub const SCORES_WIDTH: u16 = 12;
pub const SCORES_HEIGHT: u16 = 8;
pub const SCORES_XY: (u16, u16) = (44, 0);

pub const NEXT_WIDTH: u16 = 8;
pub const NEXT_HEIGHT: u16 = 5;
pub const NEXT_XY: (u16, u16) = (44, 11);

pub const BOARD_WIDTH: u16 = 20;
pub const BOARD_HEIGHT: u16 = 21;
pub const BOARD_XY: (u16, u16) = (22, 3);

pub const LEVEL_WIDTH: u16 = 12;
pub const LEVEL_HEIGHT: u16 = 2;
pub const LEVEL_XY: (u16, u16) = (44, 17);

pub const BIG_TEXT_WIDTH: u16 = 50;
pub const BIG_TEXT_HEIGHT: u16 = 6;
pub const BIG_TEXT_XY: (u16, u16) = (5, 3);

pub const GAME_OVER_TEXT_WIDTH: u16 = 50;
pub const GAME_OVER_TEXT_HEIGHT: u16 = 12;
pub const GAME_OVER_TEXT_XY: (u16, u16) = (5, 3);

pub const RECT_STATS: usize = 0;
pub const RECT_LINES: usize = 1;
pub const RECT_BOARD: usize = 4;
pub const RECT_NEXT: usize = 3;
pub const RECT_SCORES: usize = 2;
pub const RECT_LEVEL: usize = 5;
pub const RECT_BIG_TEXT: usize = 6;
pub const RECT_SCREEN: usize = 7;
pub const RECT_GAME_OVER_TEXT: usize = 8;
pub const RECT_CONTROLS: usize = 9;

pub const ELEMENTS_XY: (u16, u16) = (2, 1);

pub const TEXT_STATS: &str = "    STATISTICS    ";
pub const TEXT_LINES: &str = "LINES";
pub const TEXT_SCORES: &str = "\nTOP\n 0 0 0 0 0 0\n\nSCORE\n 0 0 0 0 0 0";
pub const TEXT_NEXT: &str = "  NEXT  ";
pub const TEXT_LEVEL: &str = "  LEVEL ??";

pub const BLOCK: &str = "██";
pub const EMPTY: &str = "  ";
pub const SCREEN_WIDTH: u16 = 32 * 2; // x 2 since each cell is 2 chars per block
pub const SCREEN_HEIGHT: u16 = 28;
pub const BACKGROUND_COLOR: u8 = 234;

