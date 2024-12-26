use rand::Rng;

const T_BLOCK: u8 = 0;
const J_BLOCK: u8 = 1;
const Z_BLOCK: u8 = 2;
const O_BLOCK: u8 = 3;
const S_BLOCK: u8 = 4;
const L_BLOCK: u8 = 5;
const I_BLOCK: u8 = 6;
const MINO_TYPES: u8 = 7;

type Rotation = Vec<Vec<u8>>;
pub struct Mino {
    rotations: Vec<Rotation>,
    selected_mino: u8,
    current_rotation: usize,
}

impl Mino {
    pub fn new() -> Self {

        let mut rng = rand::thread_rng();
        let selected_mino = rng.gen_range(0..MINO_TYPES);
        let mut rotations: Vec<Rotation> = vec![];

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
            },
            O_BLOCK => {
                rotations = vec![
                    vec![
                        vec![1, 1],
                        vec![1, 1],
                    ],
                ];
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
            },
            I_BLOCK => {
                rotations = vec![
                    vec![
                        vec![1, 1, 1, 1],
                    ],
                    vec![
                        vec![1],
                        vec![1],
                        vec![1],
                        vec![1],
                    ],
                ];
            },
            _ => {},
        }
        
        Self {
            rotations,
            selected_mino,
            current_rotation: 0,
        }
    }
}