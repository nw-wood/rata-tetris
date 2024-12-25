#programming #projects #rust #logs #tetris

# rata-tetris

***Things I want to include***

- A terminal rendition on the user interface. This is meant to be played in the terminal so it won't be _exactly_ like the classic Tetris.
- Recreation of the rules, rotation behavior, game speed, score, and anything else.
- Addition of the instant drop mechanic from more modern Tetris clones.
### Making the background

 - Initially I've recognized that there are some limitations with the way things can be displayed, and I want a repeating texture of Tetris pieces as the background similar to the classic Tetris.
	
- First I gathered a piece of vector art from the internet as an example, and I'm going to turn that into a grid of numbers to translate it into color values, and that's going to get translated into Ratatui text line information.

***A Repeating Seamless Tetris Pattern***

![[Pasted image 20241224134946.png]]

***Cells Annotated***

![[Pasted image 20241224135539.png]]

***Annotations to a 2D Vector*** #snippets

```rust
//data translated
let background_pattern: Vec<Vec<usize>> = vec![
	vec![1, 2, 1, 1, 1, 2, 3, 3],
	vec![1, 2, 1, 4, 5, 2, 1, 1],
	vec![2, 2, 2, 4, 5, 5, 4, 2],
	vec![3, 6, 4, 4, 7, 5, 4, 3],
	vec![3, 6, 6, 7, 7, 4, 4, 3],
	vec![1, 6, 1, 7, 6, 7, 7, 1],
	vec![1, 2, 1, 6, 6, 2, 7, 7],
	vec![1, 2, 1, 1, 6, 2, 3, 3],
];
```

- Each digit here needs to be turned into an associated piece of line information
	
- The 2D Vector gets mapped into material that Ratatui can use to draw
	
- 100 lines later we've got our background starting to render.

***A Ratatui Implementation*** #snippets

```rust
use std::io;
use once_cell::sync::Lazy;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
    DefaultTerminal,
};

//global variable that will contain the background widget
static BACKGROUND: Lazy<CachedBackground> = Lazy::new(||CachedBackground::new());

//a struct for the background
#[derive(Debug, Clone)]
struct CachedBackground {
    widget: Paragraph<'static>,
}

//creation method for the background widget
impl CachedBackground {
    pub fn new() -> Self {
		//2 chars so they take up a square instead of a tall rectangle
        let bg_char = "██";
        //values to map to styles
        let background_pattern = vec![
            vec![1, 2, 1, 1, 1, 2, 3, 3],
            vec![1, 2, 1, 4, 5, 2, 1, 1],
            vec![2, 2, 2, 4, 5, 5, 4, 2],
            vec![3, 6, 4, 4, 7, 5, 4, 3],
            vec![3, 6, 6, 7, 7, 4, 4, 3],
            vec![1, 6, 1, 7, 6, 7, 7, 1],
            vec![1, 2, 1, 6, 6, 2, 7, 7],
            vec![1, 2, 1, 1, 6, 2, 3, 3],
        ];
        //iterate over the pattern, map rows, map cells
        //... for each cell match value to a style, and return a styled span
        let lines: Vec<Line> = background_pattern
            .iter()
            .map(|row| {
                Line::from(
                    row.iter()
                        .map(|&cell| {
                            let style = match cell {
                                1 => Style::default().fg(Color::Indexed(241)),
                                2 => Style::default().fg(Color::Indexed(240)),
                                3 => Style::default().fg(Color::Indexed(235)),
                                4 => Style::default().fg(Color::Indexed(236)),
                                5 => Style::default().fg(Color::Indexed(237)),
                                6 => Style::default().fg(Color::Indexed(238)),
                                7 => Style::default().fg(Color::Indexed(239)),
                                _ => Style::default().fg(Color::Indexed(232)),
                            };
                            //styled span returned here
                            Span::styled(bg_char, style)
                        })
                        //spans collected into a vector of them
                        .collect::<Vec<Span>>(),
                )
            })
            //span vectors collected into a vector of lines
            .collect();
		// return struct
        Self {
	        //widget is a paragraph made from the lines
            widget: Paragraph::new(lines),
        }
    }
}

//implementing the widget trait onto the background struct so it can be rendered
impl Widget for CachedBackground {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.widget.render(area, buf);
    }
}

fn main() -> io::Result<()> {
	//typical ratatui init - sets a few important things up
    let mut terminal = ratatui::init();
    //clear it and force a redraw
    terminal.clear()?;
    //run function
    let app_result = run(&mut terminal);
    //restore the terminal
    ratatui::restore();
    //return the run result
    app_result
}

//setup if any before draw loop
fn run(terminal: &mut DefaultTerminal) -> io::Result<()> {
    loop {
	    //draw the user interface to the provided backend
        draw_ui(terminal)?;

		//read for key presses, blocks execution (!!!)
        if let Event::Key(key) = event::read()? {
	        //break if 'q' is pressed
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q'){
                break;
            }
        }
    }
    //q must have been pressed - bye!
    Ok(())
}

fn draw_ui(terminal: &mut DefaultTerminal) -> io::Result<()> {
	//draw using the provided lambda function as a callback
    terminal.draw(|frame| {
	    //since background is a lazy static the first time this gets
	    //... called here it will be calculated for, and then just copies after
		frame.render_widget(BACKGROUND.clone(), frame.area());
	})
	//map the frame result into a unit struct instead
    .map(|_| ())
}

```

***First Tile of the Background Rendering to Terminal  : )***

![[Pasted image 20241224205741.png]]

***Things to Improve On***

- The colors are contrasting too much.
	
- The pattern when tiled, since it is so small, will end up repeating too obviously.
	
- As it stands, the logic for tiling the background to fill the entire terminal doesn't exist.
	
- Can directly reference the BACKGROUND static .


***Improvement: Background Tiling Implementation*** #snippets

```rust

//figure which is smaller - a tiles edge, or the areas edge
fn calc_tile_size(tile_start: u16, total_size: u16, max_tile_size: u16) -> u16 {
	let remaining_size = total_size.saturating_sub(tile_start);
	u16::min(max_tile_size, remaining_size)
}

//implement for references to a CacheBackground instead
impl Widget for &CachedBackground {
    fn render(self, area: Rect, buf: &mut Buffer) {

		//figure how many tiles will fit in the space and overlap by 1 
        let x_tiles = area.width / self.width + 1;
        let y_tiles = area.height / self.height + 1;

		//for each y and x tile in x_tiles and y_tiles
        for y_tile in 0..y_tiles {
			//store y_start only this iter instead of each iter * x iters
            let y_start = y_tile * self.height;
            let tile_h = calc_tile_size(y_start, area.height, self.height);

            for x_tile in 0..x_tiles {
				//store x_start, and the width of the tile that doesn't overlap
                let x_start = x_tile * self.width;
                let tile_w = calc_tile_size(x_start, area.width, self.width);

				//creat the react that will fill the space in the buffer
                let tile_rect = Rect::new(
	                x_start, 
	                y_start, 
	                tile_w, 
	                tile_h
	            );

				//clone the paragraph, render it for the current rect in buffer
                self.widget.clone().render(tile_rect, buf);
            }
        }
    }
}

//...

fn draw_ui(terminal: &mut DefaultTerminal) -> io::Result<()> {
    terminal
        .draw(|frame| {
	        //reference underlying value of the static
            frame.render_widget(&*BACKGROUND, frame.area());
        })
        .map(|_| ())
}
```

***Background Tiles Filing the Whole Terminal***

![[Pasted image 20241225121129.png]]

***Next Items that can be worked on...***
- Use multiple threads to handle input and draw the background at the same time.
- Setup input for all the keys that are going to be used to play.
- Draw the basics of the user interface.
- Implement the game's logic.

***Starting With Multi-Threading***

```rust
//contain a reference to the terminal backend in an arc mutex
fn run(terminal: Arc<Mutex<DefaultTerminal>>) -> io::Result<()> {
	//clone the backend arc mutex, and create a channel
    let arc_terminal = terminal.clone();
    let (stop_sender, stop_receiver) = std::sync::mpsc::channel();
	//spawn a thread and pass draw_up the arc mutex, and the receiver
    let draw_thread_handle = thread::spawn(|| -> io::Result<()> {
        draw_ui(arc_terminal, stop_receiver)?;
        Ok(())
    });
	//still read for key inputs here
    loop {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q'){
                break;
            }
        }
    }
	//'q' pressed so input loop broke - so send an empty value
    stop_sender.send(()).unwrap();
	//join the thread and propagate the error result if any
    draw_thread_handle.join().unwrap()?;
    Ok(())
}

//new draw function takes the terminal and a receiver
fn draw_ui(terminal: Arc<Mutex<DefaultTerminal>>, stop_receiver: Receiver<()>) -> io::Result<()> {
    loop {
        thread::sleep(Duration::from_millis(100));
        //match against the try_recv result on the reciever
        match stop_receiver.try_recv() {
            Err(std::sync::mpsc::TryRecvError::Empty) => {}, 
            _ => break, //if anything other than empty, break!
        }
		//otherwise, draw the background
        terminal.lock().unwrap().draw(|frame| {
                frame.render_widget(&*BACKGROUND, frame.area());
        })
        .map(|_| ())?;
    }
    Ok(())
}
```

***Changing Run to Handle More Inputs***

```rust
///fn run()...
	loop {
		if let Event::Key(key) = event::read()? {
			if key.kind == KeyEventKind::Press {
				match key.code {
					//the controls for the game to consider
					KeyCode::Char('q') => break,
					KeyCode::Up => {},          //slam
					KeyCode::Down => {},        //drop faster
					KeyCode::Left => {},        //move left 
					KeyCode::Right => {},       //move right
					KeyCode::PageUp => {},      //rotate piece counter-clockwise
					KeyCode::PageDown => {},    //rotate piece clockwise
					KeyCode::Char(' ') => {},   //pause the game
					_ => {}
				}
			}
		}
	}
//...
```


***Fleshing Out Some Game Logic***

```rust
use std::fs::File;
use std::io::{self, Write};
//added the dirs crate for easy access to home on windows, mac, and linux
use dirs::home_dir;

const TOP_SCORE_FILENAME: &str = "top_score";

//these will be expanded into a 2D vector of u8's that represent the board
const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 20;

//member vars for the game
pub struct Game {
    line_count: u16,
    statistics: Vec<u16>,
    top_score: u32,
    current_score: u32,
    current_level: u16,
    board_state: Vec<Vec<u8>>,
}

//methods
impl Game {
    pub fn new() -> Self { //initialize a new game :)
        Self {
            line_count: 0,
            statistics: vec![0; 7],
            top_score: {
	            //match against function that returns our score on new Game
                match load_top_score() {
                    Some(score) => score,
                    None => 0,
                }
            },
            current_level: 0,
            current_score: 0,
            //construct a 2D vector for the board
            board_state: vec![vec![u8::from(0); BOARD_WIDTH]; BOARD_HEIGHT],
        }
    }

    fn board_insert(&mut self, to_insert: Vec<Vec<u8>>, color: u8) {
        /*
	        up next 🤠
        */
    }

    fn increase_lines(&mut self) { //increment line count
        self.line_count += 1;
    }

    fn increase_stat(&mut self, index: BlockIndex) { //increment a stat item
        self.statistics[index] += 1;
    }

    fn set_top_score(&mut self) { //set the top score
	    //first try to load it from the file system
        if let Some(top_score) = load_top_score() {
            self.top_score = top_score;
        } else {
	        //or set it to 0 if it can't be loaded
            self.top_score = 0;
        }
    }

	//change the game state back to the original values
    fn game_over(&mut self) {
        self.current_level = 0;
        self.line_count = 0;
        self.statistics = vec![0; 7];
		//if a new top score
        if self.top_score < self.current_score {
	        //try to save it to the file system
            if let Err(e) = save_top_score(self.current_score) {
                println!("couldn't save top score file: {e}");
            }
            //update the top score
            self.top_score = self.current_score;
        }
        self.current_score = 0;
        //however, leave the top score alone
        //self.top_score = 0;
    }
}
//try to load the top score from the file system
fn load_top_score() -> Option<Score> {
    if let Some(home) = home_dir() {
        let file_path = home.join(TOP_SCORE_FILENAME);
        //if the file path doesn't exist then try saving and None
        if !file_path.exists() {
            if let Err(e) = save_top_score(0) {
                println!("couldn't save top score file: {e}");
            }
            return None;
        } else {
	        //otherwise if it does try loading it and parsing it to a u32
            if let Ok(mut file) = File::open(file_path) {
                let mut contents = String::new();
                if let Ok(byte_size) = file.read_to_string(&mut contents) {
                    println!("loaded {byte_size} bytes from file");
                    if let Ok(score) = contents.trim().parse::<u32>() {
	                    //success, return it
                        return Some(score);
                    }
                }
            }
        }
    }
    //in the case the file path existed but we couldn't read it
    //... additionally if it couldn't parse the value to a u32
    None
}

//try saving the top score to the file system
fn save_top_score(score: Score) -> io::Result<()> {
    if let Some(home) = home_dir() {
        let file_path = home.join(TOP_SCORE_FILENAME);
        //if it doesn't work just prop back an error for printing
        let mut file = File::create(file_path)?;
        //score, as a string, and then as a byte array, written to file
        file.write(score.to_string().as_bytes())?;
    }
    Ok(())
}
```

***Some File Restructuring***

![[Pasted image 20241225162623.png]]

- The file `game.rs` contains some unused game logic now.
	
- The file `ui.rs` contains the background and new `draw_ui()` implementation
	
- A library of widgets to call from seems to me to be a good idea...
	
- Additionally, abstracting out the interface is a good idea as it's going to grow.

***Instantiating the Game State with Multi-threaded Access**

```rust
//in main.rs...
use game::Game;

fn run() {
	//...
	let arc_game = Arc::new(Mutex::new(Game::new()));
		//... 
		//a smart pointer to the game state is passed into the drawing thread
		draw_ui(arc_terminal, arc_game, stop_receiver)?;
}
```

```rust
//in ui.rs...

//implement the Widget trait for Game references so they can be drawn
impl Widget for &Game {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized
    {
        /*code that renders the game will be here! 🫥*/
    }
}

pub fn draw_ui(
	terminal: Arc<Mutex<DefaultTerminal>>, 
	game_state: Arc<Mutex<Game>>, //added the game_state smart pointer
	stop_receiver: Receiver<()>
) -> io::Result<()> {
			//in the draw call to the terminal...
			//pass reference to game state to the method
            frame.render_widget(&*game_state.lock().unwrap(), frame.area());
	
```

Now this means we'll be able to draw the game state, and also be able to interact with in the key pressing logic.

***Keys Trigger Game Functions***

```rust
//...
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => break,
                    //a bunch of method calls to interact with the game state
                    KeyCode::Up =>          game.slam(),
                    KeyCode::Down =>        game.drop_speed_faster(),
                    KeyCode::Left =>        game.move_left(),
                    KeyCode::Right =>       game.move_right(),
                    KeyCode::PageUp =>      game.rotate_counter_clockwise(),
                    KeyCode::PageDown =>    game.rotate_clockwise(),
                    KeyCode::Char(' ') =>   game.toggle_paused(),
                    _ => {}
                }
            }
            else if key.kind == KeyEventKind::Release {
                match key.code {
                    KeyCode::Down => game.drop_speed_normal(),
                    _ => {}
                }
            }
        }
//...
```

`probably a good plan to push this to github at this point...`













































































....

***Interesting stuff along the way

https://github.com/charmbracelet/vhs ... terminal gif recording - neat!
https://crates.io/crates/font8x8 ... a bitmap font of unicode characters
https://ratatui.rs/examples/style/colors/ ... an example of messing with lots of color stuff?

```bash
# will clone all the ratatui examples for exploration
git clone https://github.com/ratatui/ratatui.git --branch latest
cd ratatui
cargo run --example=colors --features=crossterm
```

_This is particularly interesting... how was this done?_

![[Pasted image 20241224141932.png]]

```rust
fn render_indexed_grayscale(frame: &mut Frame, area: Rect) {
	//create a vertical container holding 2 items of 1 length
	//so two horizontal rows in a vertical layout...
	let layout = Layout::vertical([
		Constraint::Length(1), // 232 - 243
		Constraint::Length(1), // 244 - 255
	])
	//splits area into the two rows
	.split(area)
	//iterations over the two rows
	.iter()
	//maps iterations to a bunch of vectors and flattens them
	.flat_map(|area| {
		//area in this closure refers to the area of each row...
		//layout specified as 12 columns of 6 in length
		Layout::horizontal([Constraint::Length(6); 12])
		//the closure's area is split up by this layout
		.split(*area)
		//each Rect in Rc<[Rect]> is pushed into a vector
		.to_vec()
	})
	//layout is now 2 rows, 12 columns of 6, in a vector of Rects
	.collect_vec();


// for the numbers 232 through and including 255
	for i in 232..=255 {
		// retrieve a Color from an 8bit 256 color value
		let color = Color::Indexed(i);
		// create a format string from i in the for {}
		// ... 0> right aligned and padded with 0's
		// ... 3 is the width of the result (always at least 3)
		let color_index = format!("{i:0>3}");
		// make the dark colors easier to read
		let bg = if i < 244 { Color::Gray } else { Color::Black };
		// so a small paragraph widget gets created for each
		// ... it's filled out by a vector of three items
		let paragraph = Paragraph::new(Line::from(vec![
			//first, the string with a foreground made from the color indexed by i
			//and with a background of the bg result...
			// ... a styled color item from a string
			color_index.fg(color).bg(bg),
			//the same deal here, but a block text representation of the color
			"██".bg(color).fg(color),
			//a bug fix for vhs, a library that makes gifs from the terminal
			"███████".reversed(),
		]));
		frame.render_widget(paragraph, layout[i as usize - 232]);
	}
}
```

A screen shot of all the rotations...

![[Pasted image 20241224152310.png]]

***What Tetris Looks Like***

![[Pasted image 20241225144659.png]]

***Items and Things For Consideration***
- Each widget will need a kind of border outline.
- I'll ignore the A-TYPE label, as that has to do with game music and colors.
- The terminal obviously isn't to match the interface exactly.

- Items
	- Board, 10 width, 20 height
		- Border, 12x1 top and bottom, 20x1 side walls 
	- L I N E S, 5 characters, ' - ', 3 more, '000', and 3 more for the count 2 for pre and suf
		- 13 characters, and border is needed around that
			- 14 top row, 14 bottom row, and a 1 on each side of the lines text

Okay, so I need to create rectangles that fill these spaces thoughtfully.