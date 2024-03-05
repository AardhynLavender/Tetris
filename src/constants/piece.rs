use crate::application::tile::tile::TileId;
use crate::application::utility::random::random;
use crate::application::utility::types::Coordinate;

type RawPieceData = &'static [&'static [&'static str]];

// todo:  I'd be nice to use matrix rotation to generate the rotations of the pieces.
//        but the shapes dont rotate predictably.

const I_PIECE: RawPieceData = &[
  &[
    "    ",
    "    ",
    "####",
    "    ",
  ],
  &[
    "  # ",
    "  # ",
    "  # ",
    "  # ",
  ],
];

const J_PIECE: RawPieceData = &[
  &[
    "   ",
    "###",
    "  #",
  ],
  &[
    " ##",
    " # ",
    " # ",
  ],
  &[
    "#  ",
    "###",
    "   ",
  ],
  &[
    " # ",
    " # ",
    "## ",
  ],
];

const L_PIECE: RawPieceData = &[
  &[
    "   ",
    "###",
    "#  ",
  ],
  &[
    " # ",
    " # ",
    " ##",
  ],
  &[
    "  #",
    "###",
    "   ",
  ],
  &[
    "## ",
    " # ",
    " # ",
  ],
];

const O_PIECE: RawPieceData = &[
  &[
    "##",
    "##",
  ],
];

const S_PIECE: RawPieceData = &[
  &[
    "   ",
    " ##",
    "## ",
  ],
  &[
    " # ",
    " ##",
    "  #",
  ],
];

const T_PIECE: RawPieceData = &[
  &[
    "   ",
    "###",
    " # ",
  ],
  &[
    " # ",
    " ##",
    " # ",
  ],
  &[
    " # ",
    "###",
    "   ",
  ],
  &[
    " # ",
    "## ",
    " # ",
  ],
];

const Z_PIECE: RawPieceData = &[
  &[
    "   ",
    "## ",
    " ##",
  ],
  &[
    "  #",
    " ##",
    " # ",
  ],
];

pub type ShapeData = Vec<Vec<Coordinate>>;

const TILE_SYMBOL: char = '#';

// todo:  Missing C++ constexpr right here, I'd love to generate all the shape data at compile time.
//        Might be possible with macros?

fn get_shape_rotation_coordinates(data: RawPieceData) -> ShapeData {
  let mut shape_rotations: ShapeData = Vec::new();
  for rotation in data {
    let mut shape: Vec<Coordinate> = Vec::new();
    for (y, row) in rotation.iter().enumerate() {
      for (x, cell) in row.chars().enumerate() {
        if cell == TILE_SYMBOL {
          let coordinate = Coordinate::new(x as i32, y as i32);
          shape.push(coordinate);
        }
      };
    };
    shape_rotations.push(shape);
  };
  shape_rotations
}

pub const DEFAULT_ROTATION: usize = 0; // The first index is the default rotation

pub struct PieceData {
  pub tile_id: TileId,
  pub shape: ShapeData,
  pub offset_y: u32, // the shape should spawn against the roof, so offset by the empty space above the shape in the first rotation.
}

impl PieceData {
  pub fn new(shape: ShapeData, tile_id: TileId, offset_y: u32) -> Self {
    Self { shape, tile_id, offset_y }
  }
}

#[derive(Debug)]
pub enum ShapeType { I, J, L, O, S, T, Z }

impl ShapeType {
  pub fn data(&self) -> PieceData {
    match self {
      ShapeType::I => PieceData::new(get_shape_rotation_coordinates(I_PIECE), 1, 1),
      ShapeType::J => PieceData::new(get_shape_rotation_coordinates(J_PIECE), 2, 1),
      ShapeType::L => PieceData::new(get_shape_rotation_coordinates(L_PIECE), 3, 1),
      ShapeType::O => PieceData::new(get_shape_rotation_coordinates(O_PIECE), 4, 0),
      ShapeType::S => PieceData::new(get_shape_rotation_coordinates(S_PIECE), 5, 1),
      ShapeType::T => PieceData::new(get_shape_rotation_coordinates(T_PIECE), 6, 1),
      ShapeType::Z => PieceData::new(get_shape_rotation_coordinates(Z_PIECE), 7, 1),
    }
  }

  pub fn random() -> Self {
    let index = random(0, 7);
    match index {
      0 => ShapeType::I,
      1 => ShapeType::J,
      2 => ShapeType::L,
      3 => ShapeType::O,
      4 => ShapeType::S,
      5 => ShapeType::T,
      6 => ShapeType::Z,
      _ => panic!("random shape index out of bounds"),
    }
  }
}
