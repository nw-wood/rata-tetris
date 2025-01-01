//common type aliases
pub type Rotation = Vec<Vec<u8>>;
pub type Score = u32;
pub type BoardXY = (i16, i16);
pub type BlockIndex = usize;

/* pub const SOUND_STARTUP: &str = "startup";
pub const SOUND_PAUSE: &str = "pause";
pub const SOUND_RESUME: &str = "resume";
pub const SOUND_PLAY: &str = "play";
pub const SOUND_GAME_OVER: &str = "game_over";
pub const SOUND_MOVE_LEFT: &str = "left_movement";
pub const SOUND_MOVE_RIGHT: &str = "right_movement";
pub const SOUND_ROTATE_LEFT: &str = "left_rotation";
pub const SOUND_ROTATE_RIGHT: &str = "right_rotation";
pub const SOUND_SLAM: &str = "slam";
pub const SOUND_PLACE: &str = "place";
pub const SOUND_LINE_CLEAR: &str = "clear_line";
pub const SOUND_TETRIS: &str = "tetris_sound";
pub const SOUND_NEW_TOP_SCORE: &str = "top_score"; */

pub const BASE_SCORES: [u32; 5] = [0, 40, 100, 300, 1200];

pub const J_BLOCK: u8 = 1;
pub const Z_BLOCK: u8 = 2;
pub const O_BLOCK: u8 = 3;
pub const S_BLOCK: u8 = 4;
pub const T_BLOCK: u8 = 5;
pub const L_BLOCK: u8 = 6;
pub const I_BLOCK: u8 = 7;

pub const STATE_PLAYING: u8 = 0;
pub const STATE_PAUSED: u8 = 1;
pub const STATE_GAME_OVER: u8 = 2;
pub const STATE_START_SCREEN: u8 = 3;
pub type GameState = u8;

pub const MINO_TYPES: u8 = 7;

pub const TOP_SCORE_FILENAME: &str = "top_score";

//x and y for the actual play area
pub const GAME_BOARD_WIDTH: usize = 10;
pub const GAME_BOARD_HEIGHT: usize = 20;

//directional offsets for the games movement methods
pub const LEFT_OFFSET:  BoardXY = (-2, 0);
pub const RIGHT_OFFSET: BoardXY = (2, 0);
pub const DOWN_OFFSET:  BoardXY = (0, 1);
pub const NO_OFFSET:    BoardXY = (0, 0);

//match clarity
pub const ROT_LEFT: u8 = 0;
pub const ROT_RIGHT: u8 = 1;

//signal identifiers for the game timer thread
pub const SIGNAL_INCREASE: u8 = 1;
pub const SIGNAL_PAUSE: u8 = 2;
pub const SIGNAL_UNPAUSE: u8 = 3;
pub const SIGNAL_KILL: u8 = 4;
pub const SIGNAL_RESET: u8 = 5;
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

pub const ZOOM_TIP_TEXT: &str = "Tip: On many systems you can adjust the zoom. You can try\nthis by holding down the Ctrl key, and pressing +, -, or \nscrolling the mouse wheel. 💬";

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

pub const SCORE_PADDING: usize = SCREEN_WIDTH as usize - 6;

pub const BORDER_WIDTH_PAD: u16 = 2;
pub const BORDER_HEIGHT_PAD: u16 = 1;

//remember* when adding rects width, height, xy, and rect below - remember to push the rect into elements in ui!
pub const CONTROLS_WIDTH: u16 = SCREEN_WIDTH - 7;
pub const CONTROLS_HEIGHT: u16 = 0;
pub const CONTROL_XY: (u16, u16) = (0, SCREEN_HEIGHT - 2);

pub const GAME_OVER_STATS_WIDTH: u16 = SCREEN_WIDTH - 7;
pub const GAME_OVER_STATS_HEIGHT: u16 = 2;
pub const GAME_OVER_STATS_XY: (u16, u16) = (0, SCREEN_HEIGHT - 10);

pub const ZOOM_TIP_WIDTH: u16 = SCREEN_WIDTH - 7;
pub const ZOOM_TIP_HEIGHT: u16 = 4;
pub const ZOOM_TIP_XY: (u16, u16) = (0, SCREEN_HEIGHT - 10);

pub const NEW_TOP_SCORE_WIDTH: u16 = SCREEN_WIDTH - 7;
pub const NEW_TOP_SCORE_HEIGHT: u16 = 2;
pub const NEW_TOP_SCORE_XY: (u16, u16) = (0, SCREEN_HEIGHT - 6);

pub const STATS_INSET_WIDTH: u16 = 14;
pub const STATS_INSET_HEIGHT: u16 = 22;
pub const STATS_INSET_XY: (u16, u16) = (5, 3); 

pub const STATS_WIDTH: u16 = 18;
pub const STATS_HEIGHT: u16 = 23;
pub const STATS_XY: (u16, u16) = (2, 0);

pub const LINES_WIDTH: u16 = 20;
pub const LINES_HEIGHT: u16 = 2;
pub const LINES_XY: (u16, u16) = (22, 0);

pub const SCORES_WIDTH: u16 = 12;
pub const SCORES_HEIGHT: u16 = 8;
pub const SCORES_XY: (u16, u16) = (44, 0);

pub const NEXT_WIDTH: u16 = 10;
pub const NEXT_HEIGHT: u16 = 6;
pub const NEXT_XY: (u16, u16) = (44, 11);

pub const NEXT_INSET_WIDTH: u16 = 6;
pub const NEXT_INSET_HEIGHT: u16 = 2;
pub const NEXT_INSET_XY: (u16, u16) = (46, 14);

pub const BOARD_WIDTH: u16 = 20;
pub const BOARD_HEIGHT: u16 = 21;
pub const BOARD_XY: (u16, u16) = (22, 3);

pub const LEVEL_WIDTH: u16 = 12;
pub const LEVEL_HEIGHT: u16 = 2;
pub const LEVEL_XY: (u16, u16) = (44, 18);

pub const BIG_TEXT_WIDTH: u16 = 50;
pub const BIG_TEXT_HEIGHT: u16 = 6;
pub const BIG_TEXT_XY: (u16, u16) = (5, 3);

pub const GAME_OVER_TEXT_WIDTH: u16 = 50;
pub const GAME_OVER_TEXT_HEIGHT: u16 = 12;
pub const GAME_OVER_TEXT_XY: (u16, u16) = (5, 3);

//rects for the playing screen
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
pub const RECT_STATS_INSET: usize = 10;
pub const RECT_NEXT_INSET: usize = 11;
pub const RECT_GAME_OVER_STATS: usize = 12;
pub const RECT_NEW_TOP_SCORE: usize = 13;
pub const RECT_ZOOM_TIP: usize = 14;

pub const ELEMENTS_XY: (u16, u16) = (2, 1);

pub const TEXT_STATS: &str = "    STATISTICS    \n \n \n \n \n \n \n \n \n \n \n \n \n \n \n \n \n \n \n \n \n \n "; //doing this causes draw_render paragraphs to fill with empty space properly
pub const TEXT_LINES: &str = "     LINES - ";
pub const TEXT_NEXT: &str = "   NEXT\n \n \n \n ";
pub const TEXT_LEVEL: &str = "  LEVEL ";

pub const BLOCK: &str = "██";
pub const CLEAR: &str = "░░";
/* pub const EMPTY: &str = "  "; */
pub const SCREEN_WIDTH: u16 = 32 * 2; // x 2 since each cell is 2 chars per block
pub const SCREEN_HEIGHT: u16 = 28;
pub const BACKGROUND_COLOR: u8 = 234;


pub const LEVEL_PALLETE_LENGH: usize = 8;
pub const PALETTE_BLURPLE: [u8; LEVEL_PALLETE_LENGH] =[069, 063, 057, 069, 075, 033, 039, 081]; 
pub const PALETTE_LIME: [u8; LEVEL_PALLETE_LENGH] =   [040, 046, 047, 034, 028, 022, 082, 082 + 2]; //most + 2 are fine - some adjusted for custom
pub const PALETTE_PINK: [u8; LEVEL_PALLETE_LENGH] =   [219, 213, 207, 201, 200, 199, 206, 205];
pub const PALETTE_SWAMP: [u8; LEVEL_PALLETE_LENGH] =  [033, 063, 027, 039, 041, 047, 046, 046 + 2];
pub const PALETTE_MELON: [u8; LEVEL_PALLETE_LENGH] =  [085, 120, 048, 199, 200, 201, 207, 207 + 2];
pub const PALETTE_LAKE: [u8; LEVEL_PALLETE_LENGH] =   [069, 063, 057, 085, 079, 120, 115, 115 + 2];
pub const PALETTE_FACTORY: [u8; LEVEL_PALLETE_LENGH] =[242, 244, 249, 196, 160, 124, 202, 202 + 2];
pub const PALETTE_MUAVE: [u8; LEVEL_PALLETE_LENGH] =  [052, 088, 089, 091, 093, 141, 129, 129 + 2];
pub const PALETTE_NARU: [u8; LEVEL_PALLETE_LENGH] =   [196, 160, 124, 020, 027, 111, 075, 214];
pub const PALETTE_CREAM: [u8; LEVEL_PALLETE_LENGH] =  [222, 216, 221, 202, 196, 160, 228, 228 + 2];


/* pub const J_BLOCK: u8 = 1;
pub const Z_BLOCK: u8 = 2;
pub const O_BLOCK: u8 = 3;
pub const S_BLOCK: u8 = 4;
pub const L_BLOCK: u8 = 5;
pub const I_BLOCK: u8 = 6;
pub const T_BLOCK: u8 = 7; */

pub const TEXT_MINO_O: &str = r#"████
████"#;

pub const TEXT_MINO_L: &str = r#"██████
██


"#;

pub const TEXT_MINO_J: &str = r#"██
██████"#;

pub const TEXT_MINO_T: &str = r#"  ██      
██████

"#;

pub const TEXT_MINO_S: &str = r#"████  
  ████"#;

pub const TEXT_MINO_Z: &str = r#"  ████
████"#;

pub const TEXT_MINO_I: &str = r#"████████         



"#;