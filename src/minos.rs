use rand::Rng;
use crate::consts::*;

#[derive(Clone)]
pub struct Mino {
    rotations: Vec<Rotation>,
    pub selected_mino: u8,
    pub current_rotation: usize,
    pub start_offset: (i8, i8),
}

impl Mino {

    pub fn get_rotation(&self) -> &Rotation {
        &self.rotations[self.current_rotation]
    }

    pub fn next_rotation(&mut self, direction: u8) -> &Rotation {
        match direction {
            ROT_LEFT => {
                if self.current_rotation <= 0 {
                    return &self.rotations[self.rotations.len() - 1];
                } else {
                    return &self.rotations[self.current_rotation - 1];
                }
            },
            ROT_RIGHT => {
                if self.current_rotation >= self.rotations.len() - 1{
                    return &self.rotations[0];
                } else {
                    return &self.rotations[self.current_rotation + 1];
                }
            },
            _ => { return &self.rotations[self.current_rotation] },
        }
    }

    pub fn rotate(&mut self, direction: u8) {
        match direction {
            ROT_LEFT => {
                if self.current_rotation <= 0 {
                    self.current_rotation = self.rotations.len() - 1;
                } else {
                    self.current_rotation -= 1;
                }
            },
            ROT_RIGHT => {
                if self.current_rotation >= self.rotations.len() - 1 {
                    self.current_rotation = 0;
                } else {
                    self.current_rotation += 1;
                }
            },
            _ => {},
        }
    }

    pub fn new(current_next: &u8) -> Self {

        let mut rng = rand::thread_rng();
        //pick a mino
        let mut selected_mino = rng.gen_range(0..MINO_TYPES);
        //if it's the same as mino that's next then reroll it, or if a reroll is specified
        if selected_mino == *current_next || selected_mino == REROLL {
            loop {
                //roll for a new block and take the first one that isn't a reroll result
                selected_mino = rng.gen_range(0..MINO_TYPES);
                if selected_mino != REROLL {
                    break;
                }
            }
        }
        let mut rotations: Vec<Rotation> = vec![];
        let mut start_offset: (i8, i8) = (0, 0);
        //wish I had access to some of the game state here... pass in reference to current mino then
        //FIX: this should use constant arrays instead of vectors built like this
        match selected_mino {
            T_BLOCK => {
                rotations = vec![
                    vec![
                        vec![0, 0, 0],
                        vec![1, 1, 1],
                        vec![0, 1, 0],
                    ],
                    vec![
                        vec![0, 1, 0],
                        vec![1, 1, 0],
                        vec![0, 1, 0],
                    ],
                    vec![
                        vec![0, 1, 0],
                        vec![1, 1, 1],
                        vec![0, 0, 0],
                    ],
                    vec![
                        vec![0, 1, 0],
                        vec![0, 1, 1],
                        vec![0, 1, 0],
                    ],
                ];
                start_offset = (8, 0);
            },
            J_BLOCK => {
                rotations = vec![
                    vec![
                        vec![0, 0, 0],
                        vec![1, 1, 1],
                        vec![0, 0, 1],
                    ],
                    vec![
                        vec![0, 1, 0],
                        vec![0, 1, 0],
                        vec![1, 1, 0],
                    ],
                    vec![
                        vec![1, 0, 0],
                        vec![1, 1, 1],
                        vec![0, 0, 0],
                    ],
                    vec![
                        vec![0, 1, 1],
                        vec![0, 1, 0],
                        vec![0, 1, 0],
                    ],
                ];
                start_offset = (8, 0);
            },
            Z_BLOCK => {
                rotations = vec![
                    vec![
                        vec![0, 0, 0],
                        vec![0, 1, 1],
                        vec![1, 1, 0],
                    ],
                    vec![
                        vec![0, 1, 0],
                        vec![0, 1, 1],
                        vec![0, 0, 1],
                    ],
                ];
                start_offset = (8, 0);
            },
            O_BLOCK => {
                rotations = vec![
                    vec![
                        vec![1, 1],
                        vec![1, 1],
                    ],
                ];
                start_offset = (8, 1);
            },
            S_BLOCK => {
                rotations = vec![
                    vec![
                        vec![0, 0, 0],
                        vec![1, 1, 0],
                        vec![0, 1, 1],
                    ],
                    vec![
                        vec![0, 0, 1],
                        vec![0, 1, 1],
                        vec![0, 1, 0],
                    ],
                ];
                //was 9
                start_offset = (8, 0);
            },
            L_BLOCK => {
                rotations = vec![
                    vec![
                        vec![0, 0, 0],
                        vec![1, 1, 1],
                        vec![1, 0, 0],
                    ],
                    vec![
                        vec![1, 1, 0],
                        vec![0, 1, 0],
                        vec![0, 1, 0],
                    ],
                    vec![
                        vec![0, 0, 1],
                        vec![1, 1, 1],
                        vec![0, 0, 0],
                    ],
                    vec![
                        vec![0, 1, 0],
                        vec![0, 1, 0],
                        vec![0, 1, 1],
                    ],
                ];
                start_offset = (8, 0);
            },
            I_BLOCK => {
                rotations = vec![
                    vec![
                        vec![0, 0, 0, 0],
                        vec![0, 0, 0, 0],
                        vec![1, 1, 1, 1],
                        vec![0, 0, 0, 0],
                    ],
                    vec![
                        vec![0, 0, 1, 0],
                        vec![0, 0, 1, 0],
                        vec![0, 0, 1, 0],
                        vec![0, 0, 1, 0],
                    ],
                ];
                start_offset = (6, -1);
            },
            _ => {},
        }
        
        Self {
            rotations,
            selected_mino,
            current_rotation: 0,
            start_offset,
        }
    }
}