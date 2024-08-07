use crate::engine::tile::tile::TileId;
use crate::engine::utility::random::random;
use crate::engine::utility::types::Coordinate;

/**
 * Tetrinomino constants and associated functions
 */

type RawPieceData = &'static [&'static [&'static str]];

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

pub type Shape = Vec<Coordinate>;
pub type ShapeData = Vec<Shape>;

const RAW_DATA_TILE: char = '#';
pub const DEFAULT_ROTATION: usize = 0; // The first index is the default rotation

/// Convert the raw piece data into shape data
fn get_shape_rotation_coordinates(data: RawPieceData) -> ShapeData {
  let mut shape_rotations: ShapeData = Vec::new();
  for rotation in data {
    let mut shape: Vec<Coordinate> = Vec::new();
    for (y, row) in rotation.iter().enumerate() {
      for (x, cell) in row.chars().enumerate() {
        if cell == RAW_DATA_TILE {
          let coordinate = Coordinate::new(x as i32, y as i32);
          shape.push(coordinate);
        }
      };
    };
    shape_rotations.push(shape);
  };
  shape_rotations
}

/// Primitive data for a Tetrimino
pub struct PieceData {
  pub tile_id: TileId,
  pub shape: ShapeData,
  /// align shapes better in the preview
  pub preview_offset: Coordinate,
  /// The shape should spawn against the roof
  /// So offset by the empty space above the shape in the first rotation.
  pub offset_y: u32,
}

impl PieceData {
  pub fn new(shape: ShapeData, tile_id: TileId, offset_y: u32, preview_offset: Coordinate) -> Self {
    Self { shape, tile_id, offset_y, preview_offset }
  }
}

/// The different types of Tetriminos
#[derive(Debug)]
pub enum ShapeType { I, J, L, O, S, T, Z }

impl ShapeType {
  /// Get the piece data for the shape
  pub fn data(&self) -> PieceData {
    match self {
      ShapeType::I => PieceData::new(get_shape_rotation_coordinates(I_PIECE), 1, 2, Coordinate::new(0, -1)),
      ShapeType::J => PieceData::new(get_shape_rotation_coordinates(J_PIECE), 2, 1, Coordinate::new(1, 0)),
      ShapeType::L => PieceData::new(get_shape_rotation_coordinates(L_PIECE), 3, 1, Coordinate::new(1, 0)),
      ShapeType::O => PieceData::new(get_shape_rotation_coordinates(O_PIECE), 4, 0, Coordinate::new(2, 1)),
      ShapeType::S => PieceData::new(get_shape_rotation_coordinates(S_PIECE), 5, 1, Coordinate::new(1, 0)),
      ShapeType::T => PieceData::new(get_shape_rotation_coordinates(T_PIECE), 6, 1, Coordinate::new(1, 0)),
      ShapeType::Z => PieceData::new(get_shape_rotation_coordinates(Z_PIECE), 7, 1, Coordinate::new(1, 0)),
    }
  }

  /// return a random shape
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
