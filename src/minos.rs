use rand::Rng;

pub const T_BLOCK: u8 = 0;
pub const J_BLOCK: u8 = 1;
pub const Z_BLOCK: u8 = 2;
pub const O_BLOCK: u8 = 3;
pub const S_BLOCK: u8 = 4;
pub const L_BLOCK: u8 = 5;
pub const I_BLOCK: u8 = 6;
pub const MINO_TYPES: u8 = 7;

type Rotation = Vec<Vec<u8>>;

const ROT_LEFT: u8 = 0;
const ROT_RIGHT: u8 = 1;

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

    pub fn new() -> Self {

        let mut rng = rand::thread_rng();
        let selected_mino = rng.gen_range(0..MINO_TYPES);
        let mut rotations: Vec<Rotation> = vec![];
        let mut start_offset: (i8, i8) = (0, 0);

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
                start_offset = (10, 0);
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
                start_offset = (10, 0);
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
                start_offset = (10, 0);
            },
            O_BLOCK => {
                rotations = vec![
                    vec![
                        vec![1, 1],
                        vec![1, 1],
                    ],
                ];
                start_offset = (10, 1);
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
                start_offset = (10, 0);
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
                start_offset = (10, 0);
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
                start_offset = (8, -1);
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