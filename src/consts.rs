//common type aliases
pub type Rotation = Vec<Vec<u8>>;
pub type Score = u32;
pub type BoardXY = (i8, i8);

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

pub const STATS_WIDTH: u16 = 18;
pub const LINES_WIDTH: u16 = 24;
pub const SCORES_WIDTH: u16 = 12;
pub const NEXT_WIDTH: u16 = 8;
pub const BOARD_WIDTH: u16 = 24;
pub const LEVEL_WIDTH: u16 = 12;
pub const BIG_TEXT_WIDTH: u16 = 50;

pub const STATS_HEIGHT: u16 = 18;
pub const LINES_HEIGHT: u16 = 2;
pub const SCORES_HEIGHT: u16 = 8;
pub const NEXT_HEIGHT: u16 = 5;
pub const BOARD_HEIGHT: u16 = 20;
pub const LEVEL_HEIGHT: u16 = 2;
pub const BIG_TEXT_HEIGHT: u16 = 6;

pub const ELEMENTS_XY: (u16, u16) = (2, 2);
pub const STATS_XY: (u16, u16) = (0, 5);
pub const LINES_XY: (u16, u16) = (20, 0);
pub const SCORES_XY: (u16, u16) = (46, 0);
pub const NEXT_XY: (u16, u16) = (46, 11);
pub const BOARD_XY: (u16, u16) = (20, 3);
pub const LEVEL_XY: (u16, u16) = (46, 17);
pub const BIG_TEXT_XY: (u16, u16) = (5, 3);

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

pub const RECT_STATS: usize = 0;
pub const RECT_LINES: usize = 1;
pub const RECT_BOARD: usize = 4;
pub const RECT_NEXT: usize = 3;
pub const RECT_SCORES: usize = 2;
pub const RECT_LEVEL: usize = 5;
pub const RECT_BIG_TEXT: usize = 6;
pub const RECT_SCREEN: usize = 7;