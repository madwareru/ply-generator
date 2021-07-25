use crate::types::{Tile, TilePiece, Direction};
use ply_generator_lib::V3;
use crate::generators::SlideHasSide;

pub const PTS_FILE: &[u8] = include_bytes!("control_points.ron");
pub const NORMALIZED_ONE: f32 = std::f64::consts::FRAC_1_SQRT_2 as f32;
pub const BASES_FORWARD: ((f32, f32), (f32, f32), (f32, f32)) =
    (
        (1.0, 0.0),
        (0.0, 1.0),
        (NORMALIZED_ONE, NORMALIZED_ONE)
    );
pub const BASES_LEFT: ((f32, f32), (f32, f32), (f32, f32)) =
    (
        (0.0, 1.0),
        (-1.0, 0.0),
        (-NORMALIZED_ONE, NORMALIZED_ONE)
    );
pub const BASES_BACKWARD: ((f32, f32), (f32, f32), (f32, f32)) =
    (
        (-1.0, 0.0),
        (0.0, -1.0),
        (-NORMALIZED_ONE, -NORMALIZED_ONE)
    );
pub const BASES_RIGHT: ((f32, f32), (f32, f32), (f32, f32)) =
    (
        (0.0, -1.0),
        (1.0, 0.0),
        (NORMALIZED_ONE, -NORMALIZED_ONE)
    );

pub const TILE_DEFINITIONS: &[Tile] = &[
    Tile(
          TilePiece::Quad {
              height: 0.0,
              offset: V3 {
                  x: 0.0,
                  y: 0.0,
                  z: 0.0,
              },
          },
          TilePiece::Quad {
              height: 0.0,
              offset: V3 {
                  x: 0.0,
                  y: 0.0,
                  z: 1.0,
              },
          },
          TilePiece::Quad {
              height: 0.0,
              offset: V3 {
                  x: 1.0,
                  y: 0.0,
                  z: 0.0,
              },
          },
          TilePiece::Quad {
              height: 0.0,
              offset: V3 {
                  x: 1.0,
                  y: 0.0,
                  z: 1.0,
              },
          },
    ),
    Tile(
          TilePiece::Quad {
              height: 0.0,
              offset: V3 {
                  x: 0.0,
                  y: 0.0,
                  z: 0.0,
              },
          },
          TilePiece::OuterCorner {
              direction: Direction::Right,
              offset: V3 {
                  x: 0.0,
                  y: 0.0,
                  z: 2.0,
              },
          },
          TilePiece::Quad {
              height: 0.0,
              offset: V3 {
                  x: 1.0,
                  y: 0.0,
                  z: 0.0,
              },
          },
          TilePiece::Quad {
              height: 0.0,
              offset: V3 {
                  x: 1.0,
                  y: 0.0,
                  z: 1.0,
              },
          },
    ),
    Tile(
          TilePiece::Quad {
              height: 0.0,
              offset: V3 {
                  x: 0.0,
                  y: 0.0,
                  z: 0.0,
              },
          },
          TilePiece::Quad {
              height: 0.0,
              offset: V3 {
                  x: 0.0,
                  y: 0.0,
                  z: 1.0,
              },
          },
          TilePiece::Quad {
              height: 0.0,
              offset: V3 {
                  x: 1.0,
                  y: 0.0,
                  z: 0.0,
              },
          },
          TilePiece::OuterCorner {
              direction: Direction::Backward,
              offset: V3 {
                  x: 2.0,
                  y: 0.0,
                  z: 2.0,
              },
          },
    ),
    Tile(
          TilePiece::Quad {
              height: 0.0,
              offset: V3 {
                  x: 0.0,
                  y: 0.0,
                  z: 0.0,
              },
          },
          TilePiece::Side {
              direction: Direction::Backward,
              offset: V3 {
                  x: 1.0,
                  y: 0.0,
                  z: 2.0,
              },
          },
          TilePiece::Quad {
              height: 0.0,
              offset: V3 {
                  x: 1.0,
                  y: 0.0,
                  z: 0.0,
              },
          },
          TilePiece::Side {
              direction: Direction::Backward,
              offset: V3 {
                  x: 2.0,
                  y: 0.0,
                  z: 2.0,
              },
          },
    ),
    Tile(
          TilePiece::OuterCorner {
              direction: Direction::Forward,
              offset: V3 {
                  x: 0.0,
                  y: 0.0,
                  z: 0.0,
              },
          },
          TilePiece::Quad {
              height: 0.0,
              offset: V3 {
                  x: 0.0,
                  y: 0.0,
                  z: 1.0,
              },
          },
          TilePiece::Quad {
              height: 0.0,
              offset: V3 {
                  x: 1.0,
                  y: 0.0,
                  z: 0.0,
              },
          },
          TilePiece::Quad {
              height: 0.0,
              offset: V3 {
                  x: 1.0,
                  y: 0.0,
                  z: 1.0,
              },
          },
    ),
    Tile(
          TilePiece::Side {
              direction: Direction::Right,
              offset: V3 {
                  x: 0.0,
                  y: 0.0,
                  z: 1.0,
              },
          },
          TilePiece::Side {
              direction: Direction::Right,
              offset: V3 {
                  x: 0.0,
                  y: 0.0,
                  z: 2.0,
              },
          },
          TilePiece::Quad {
              height: 0.0,
              offset: V3 {
                  x: 1.0,
                  y: 0.0,
                  z: 0.0,
              },
          },
          TilePiece::Quad {
              height: 0.0,
              offset: V3 {
                  x: 1.0,
                  y: 0.0,
                  z: 1.0,
              },
          },
    ),
    Tile(
          TilePiece::OuterCorner {
              direction: Direction::Forward,
              offset: V3 {
                  x: 0.0,
                  y: 0.0,
                  z: 0.0,
              },
          },
          TilePiece::Quad {
              height: 0.0,
              offset: V3 {
                  x: 0.0,
                  y: 0.0,
                  z: 1.0,
              },
          },
          TilePiece::Quad {
              height: 0.0,
              offset: V3 {
                  x: 1.0,
                  y: 0.0,
                  z: 0.0,
              },
          },
          TilePiece::OuterCorner {
              direction: Direction::Backward,
              offset: V3 {
                  x: 2.0,
                  y: 0.0,
                  z: 2.0,
              },
          },
    ),
    Tile(
          TilePiece::InnerCorner {
              direction: Direction::Right,
              offset: V3 {
                  x: 0.0,
                  y: 0.0,
                  z: 1.0,
              },
          },
          TilePiece::Quad {
              height: 1.0,
              offset: V3 {
                  x: 0.0,
                  y: 0.0,
                  z: 1.0,
              },
          },
          TilePiece::OuterCorner {
              direction: Direction::Right,
              offset: V3 {
                  x: 1.0,
                  y: 0.0,
                  z: 1.0,
              },
          },
          TilePiece::InnerCorner {
              direction: Direction::Right,
              offset: V3 {
                  x: 1.0,
                  y: 0.0,
                  z: 2.0,
              },
          },
    ),
    Tile(
          TilePiece::Quad {
              height: 0.0,
              offset: V3 {
                  x: 0.0,
                  y: 0.0,
                  z: 0.0,
              },
          },
          TilePiece::Quad {
              height: 0.0,
              offset: V3 {
                  x: 0.0,
                  y: 0.0,
                  z: 1.0,
              },
          },
          TilePiece::OuterCorner {
              direction: Direction::Left,
              offset: V3 {
                  x: 2.0,
                  y: 0.0,
                  z: 0.0,
              },
          },
          TilePiece::Quad {
              height: 0.0,
              offset: V3 {
                  x: 1.0,
                  y: 0.0,
                  z: 1.0,
              },
          },
    ),
    Tile(
          TilePiece::Quad {
              height: 0.0,
              offset: V3 {
                  x: 0.0,
                  y: 0.0,
                  z: 0.0,
              },
          },
          TilePiece::OuterCorner {
              direction: Direction::Right,
              offset: V3 {
                  x: 0.0,
                  y: 0.0,
                  z: 2.0,
              },
          },
          TilePiece::OuterCorner {
              direction: Direction::Left,
              offset: V3 {
                  x: 2.0,
                  y: 0.0,
                  z: 0.0,
              },
          },
          TilePiece::Quad {
              height: 0.0,
              offset: V3 {
                  x: 1.0,
                  y: 0.0,
                  z: 1.0,
              },
          },
    ),
    Tile(
          TilePiece::Quad {
              height: 0.0,
              offset: V3 {
                  x: 0.0,
                  y: 0.0,
                  z: 0.0,
              },
          },
          TilePiece::Quad {
              height: 0.0,
              offset: V3 {
                  x: 0.0,
                  y: 0.0,
                  z: 1.0,
              },
          },
          TilePiece::Side {
              direction: Direction::Left,
              offset: V3 {
                  x: 2.0,
                  y: 0.0,
                  z: 0.0,
              },
          },
          TilePiece::Side {
              direction: Direction::Left,
              offset: V3 {
                  x: 2.0,
                  y: 0.0,
                  z: 1.0,
              },
          },
    ),
    Tile(
          TilePiece::OuterCorner {
              direction: Direction::Backward,
              offset: V3 {
                  x: 1.0,
                  y: 0.0,
                  z: 1.0,
              },
          },
          TilePiece::InnerCorner {
              direction: Direction::Backward,
              offset: V3 {
                  x: 1.0,
                  y: 0.0,
                  z: 2.0,
              },
          },
          TilePiece::InnerCorner {
              direction: Direction::Backward,
              offset: V3 {
                  x: 2.0,
                  y: 0.0,
                  z: 1.0,
              },
          },
          TilePiece::Quad {
              height: 1.0,
              offset: V3 {
                  x: 1.0,
                  y: 0.0,
                  z: 1.0,
              },
          },
    ),
    Tile(
        TilePiece::Side {
            direction: Direction::Forward,
            offset: V3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        },
        TilePiece::Quad {
            height: 0.0,
            offset: V3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        },
        TilePiece::Side {
            direction: Direction::Forward,
            offset: V3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
        },
        TilePiece::Quad {
            height: 0.0,
            offset: V3 {
                x: 1.0,
                y: 0.0,
                z: 1.0,
            },
        },
    ),
    Tile(
        TilePiece::Quad {
            height: 1.0,
            offset: V3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        },
        TilePiece::InnerCorner {
            direction: Direction::Forward,
            offset: V3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        },
        TilePiece::InnerCorner {
            direction: Direction::Forward,
            offset: V3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
        },
        TilePiece::OuterCorner {
            direction: Direction::Forward,
            offset: V3 {
                x: 1.0,
                y: 0.0,
                z: 1.0,
            },
        },
    ),
    Tile(
        TilePiece::InnerCorner {
            direction: Direction::Left,
            offset: V3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
        },
        TilePiece::OuterCorner {
            direction: Direction::Left,
            offset: V3 {
                x: 1.0,
                y: 0.0,
                z: 1.0,
            },
        },
        TilePiece::Quad {
            height: 1.0,
            offset: V3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
        },
        TilePiece::InnerCorner {
            direction: Direction::Left,
            offset: V3 {
                x: 2.0,
                y: 0.0,
                z: 1.0,
            },
        },
    ),
    Tile(
        TilePiece::Quad {
            height: 1.0,
            offset: V3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        },
        TilePiece::Quad {
            height: 1.0,
            offset: V3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        },
        TilePiece::Quad {
            height: 1.0,
            offset: V3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
        },
        TilePiece::Quad {
            height: 1.0,
            offset: V3 {
                x: 1.0,
                y: 0.0,
                z: 1.0,
            },
        },
    ),
    Tile(
        TilePiece::Quad {
            height: 0.0,
            offset: V3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        },
        TilePiece::Slide {
            direction: Direction::Backward,
            offset: V3 {
                x: 1.0,
                y: 0.0,
                z: 2.0,
            },
            has_side: SlideHasSide::Nowhere,
        },
        TilePiece::Quad {
            height: 0.0,
            offset: V3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
        },
        TilePiece::Slide {
            direction: Direction::Backward,
            offset: V3 {
                x: 2.0,
                y: 0.0,
                z: 2.0,
            },
            has_side: SlideHasSide::Nowhere,
        },
    ),
    Tile(
        TilePiece::Quad {
            height: 0.0,
            offset: V3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        },
        TilePiece::Slide {
            direction: Direction::Backward,
            offset: V3 {
                x: 1.0,
                y: 0.0,
                z: 2.0,
            },
            has_side: SlideHasSide::AtRight,
        },
        TilePiece::Quad {
            height: 0.0,
            offset: V3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
        },
        TilePiece::Slide {
            direction: Direction::Backward,
            offset: V3 {
                x: 2.0,
                y: 0.0,
                z: 2.0,
            },
            has_side: SlideHasSide::Nowhere,
        },
    ),
    Tile(
        TilePiece::Quad {
            height: 0.0,
            offset: V3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        },
        TilePiece::Slide {
            direction: Direction::Backward,
            offset: V3 {
                x: 1.0,
                y: 0.0,
                z: 2.0,
            },
            has_side: SlideHasSide::Nowhere,
        },
        TilePiece::Quad {
            height: 0.0,
            offset: V3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
        },
        TilePiece::Slide {
            direction: Direction::Backward,
            offset: V3 {
                x: 2.0,
                y: 0.0,
                z: 2.0,
            },
            has_side: SlideHasSide::AtLeft,
        },
    ),
    Tile(
        TilePiece::Quad {
            height: 0.0,
            offset: V3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        },
        TilePiece::Slide {
            direction: Direction::Backward,
            offset: V3 {
                x: 1.0,
                y: 0.0,
                z: 2.0,
            },
            has_side: SlideHasSide::AtRight,
        },
        TilePiece::Quad {
            height: 0.0,
            offset: V3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
        },
        TilePiece::Slide {
            direction: Direction::Backward,
            offset: V3 {
                x: 2.0,
                y: 0.0,
                z: 2.0,
            },
            has_side: SlideHasSide::AtLeft,
        },
    ),
    Tile(
        TilePiece::Slide {
            direction: Direction::Right,
            offset: V3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            has_side: SlideHasSide::Nowhere,
        },
        TilePiece::Slide {
            direction: Direction::Right,
            offset: V3 {
                x: 0.0,
                y: 0.0,
                z: 2.0,
            },
            has_side: SlideHasSide::Nowhere,
        },
        TilePiece::Quad {
            height: 0.0,
            offset: V3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
        },
        TilePiece::Quad {
            height: 0.0,
            offset: V3 {
                x: 1.0,
                y: 0.0,
                z: 1.0,
            },
        },
    ),
    Tile( // 0_1_0_1
          TilePiece::Slide {
              direction: Direction::Right,
              offset: V3 {
                  x: 0.0,
                  y: 0.0,
                  z: 1.0,
              },
              has_side: SlideHasSide::AtRight,
          },
          TilePiece::Slide {
              direction: Direction::Right,
              offset: V3 {
                  x: 0.0,
                  y: 0.0,
                  z: 2.0,
              },
              has_side: SlideHasSide::Nowhere,
          },
          TilePiece::Quad {
              height: 0.0,
              offset: V3 {
                  x: 1.0,
                  y: 0.0,
                  z: 0.0,
              },
          },
          TilePiece::Quad {
              height: 0.0,
              offset: V3 {
                  x: 1.0,
                  y: 0.0,
                  z: 1.0,
              },
          },
    ),
    Tile(
        TilePiece::Slide {
            direction: Direction::Right,
            offset: V3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            has_side: SlideHasSide::Nowhere,
        },
        TilePiece::Slide {
            direction: Direction::Right,
            offset: V3 {
                x: 0.0,
                y: 0.0,
                z: 2.0,
            },
            has_side: SlideHasSide::AtLeft,
        },
        TilePiece::Quad {
            height: 0.0,
            offset: V3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
        },
        TilePiece::Quad {
            height: 0.0,
            offset: V3 {
                x: 1.0,
                y: 0.0,
                z: 1.0,
            },
        },
    ),
    Tile(
        TilePiece::Slide {
            direction: Direction::Right,
            offset: V3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            has_side: SlideHasSide::AtRight,
        },
        TilePiece::Slide {
            direction: Direction::Right,
            offset: V3 {
                x: 0.0,
                y: 0.0,
                z: 2.0,
            },
            has_side: SlideHasSide::AtLeft,
        },
        TilePiece::Quad {
            height: 0.0,
            offset: V3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
        },
        TilePiece::Quad {
            height: 0.0,
            offset: V3 {
                x: 1.0,
                y: 0.0,
                z: 1.0,
            },
        },
    ),
    Tile(
        TilePiece::Quad {
            height: 0.0,
            offset: V3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        },
        TilePiece::Quad {
            height: 0.0,
            offset: V3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        },
        TilePiece::Slide {
            direction: Direction::Left,
            offset: V3 {
                x: 2.0,
                y: 0.0,
                z: 0.0,
            },
            has_side: SlideHasSide::Nowhere,
        },
        TilePiece::Slide {
            direction: Direction::Left,
            offset: V3 {
                x: 2.0,
                y: 0.0,
                z: 1.0,
            },
            has_side: SlideHasSide::Nowhere,
        },
    ),
    Tile(
        TilePiece::Quad {
            height: 0.0,
            offset: V3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        },
        TilePiece::Quad {
            height: 0.0,
            offset: V3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        },
        TilePiece::Slide {
            direction: Direction::Left,
            offset: V3 {
                x: 2.0,
                y: 0.0,
                z: 0.0,
            },
            has_side: SlideHasSide::Nowhere,
        },
        TilePiece::Slide {
            direction: Direction::Left,
            offset: V3 {
                x: 2.0,
                y: 0.0,
                z: 1.0,
            },
            has_side: SlideHasSide::AtRight,
        },
    ),
    Tile(
        TilePiece::Quad {
            height: 0.0,
            offset: V3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        },
        TilePiece::Quad {
            height: 0.0,
            offset: V3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        },
        TilePiece::Slide {
            direction: Direction::Left,
            offset: V3 {
                x: 2.0,
                y: 0.0,
                z: 0.0,
            },
            has_side: SlideHasSide::AtLeft,
        },
        TilePiece::Slide {
            direction: Direction::Left,
            offset: V3 {
                x: 2.0,
                y: 0.0,
                z: 1.0,
            },
            has_side: SlideHasSide::Nowhere,
        },
    ),
    Tile(
        TilePiece::Quad {
            height: 0.0,
            offset: V3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        },
        TilePiece::Quad {
            height: 0.0,
            offset: V3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        },
        TilePiece::Slide {
            direction: Direction::Left,
            offset: V3 {
                x: 2.0,
                y: 0.0,
                z: 0.0,
            },
            has_side: SlideHasSide::AtLeft,
        },
        TilePiece::Slide {
            direction: Direction::Left,
            offset: V3 {
                x: 2.0,
                y: 0.0,
                z: 1.0,
            },
            has_side: SlideHasSide::AtRight,
        },
    ),
    Tile(
        TilePiece::Slide {
            direction: Direction::Forward,
            offset: V3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            has_side: SlideHasSide::Nowhere,
        },
        TilePiece::Quad {
            height: 0.0,
            offset: V3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        },
        TilePiece::Slide {
            direction: Direction::Forward,
            offset: V3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            has_side: SlideHasSide::Nowhere,
        },
        TilePiece::Quad {
            height: 0.0,
            offset: V3 {
                x: 1.0,
                y: 0.0,
                z: 1.0,
            },
        },
    ),
    Tile(
        TilePiece::Slide {
            direction: Direction::Forward,
            offset: V3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            has_side: SlideHasSide::Nowhere,
        },
        TilePiece::Quad {
            height: 0.0,
            offset: V3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        },
        TilePiece::Slide {
            direction: Direction::Forward,
            offset: V3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            has_side: SlideHasSide::AtRight,
        },
        TilePiece::Quad {
            height: 0.0,
            offset: V3 {
                x: 1.0,
                y: 0.0,
                z: 1.0,
            },
        },
    ),
    Tile(
        TilePiece::Slide {
            direction: Direction::Forward,
            offset: V3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            has_side: SlideHasSide::AtLeft,
        },
        TilePiece::Quad {
            height: 0.0,
            offset: V3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        },
        TilePiece::Slide {
            direction: Direction::Forward,
            offset: V3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            has_side: SlideHasSide::Nowhere,
        },
        TilePiece::Quad {
            height: 0.0,
            offset: V3 {
                x: 1.0,
                y: 0.0,
                z: 1.0,
            },
        },
    ),
    Tile(
        TilePiece::Slide {
            direction: Direction::Forward,
            offset: V3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            has_side: SlideHasSide::AtLeft,
        },
        TilePiece::Quad {
            height: 0.0,
            offset: V3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        },
        TilePiece::Slide {
            direction: Direction::Forward,
            offset: V3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            has_side: SlideHasSide::AtRight,
        },
        TilePiece::Quad {
            height: 0.0,
            offset: V3 {
                x: 1.0,
                y: 0.0,
                z: 1.0,
            },
        },
    ),
];

pub const NW: u16 = 0b0_0000_00_00_00_11;
pub const NE: u16 = 0b0_0000_00_00_11_00;
pub const SW: u16 = 0b0_0000_00_11_00_00;
pub const SE: u16 = 0b0_0000_11_00_00_00;

pub const SLIDE_SIDE_N: u16 = 0b0_0001_00_00_00_00;
pub const SLIDE_SIDE_S: u16 = 0b0_0010_00_00_00_00;
pub const SLIDE_SIDE_W: u16 = 0b0_0100_00_00_00_00;
pub const SLIDE_SIDE_E: u16 = 0b0_1000_00_00_00_00;

pub const IS_SLIDE: u16 = 0b1_0000_00_00_00_00;

pub const TILE_MASKS: [u16; 32] = [
    0b0_0000_00_00_00_00, 0b0_0000_00_00_00_11, 0b0_0000_00_00_11_00, 0b0_0000_00_00_11_11,
    0b0_0000_00_11_00_00, 0b0_0000_00_11_00_11, 0b0_0000_00_11_11_00, 0b0_0000_00_11_11_11,
    0b0_0000_11_00_00_00, 0b0_0000_11_00_00_11, 0b0_0000_11_00_11_00, 0b0_0000_11_00_11_11,
    0b0_0000_11_11_00_00, 0b0_0000_11_11_00_11, 0b0_0000_11_11_11_00, 0b0_0000_11_11_11_11,

    0b1_1100_00_00_11_11, 0b1_1000_00_00_11_11, 0b1_0100_00_00_11_11, 0b1_0000_00_00_11_11,
    0b1_0011_00_11_00_11, 0b1_0001_00_11_00_11, 0b1_0010_00_11_00_11, 0b1_0000_00_11_00_11,
    0b1_0011_11_00_11_00, 0b1_0010_11_00_11_00, 0b1_0001_11_00_11_00, 0b1_0000_11_00_11_00,
    0b1_1100_11_11_00_00, 0b1_0100_11_11_00_00, 0b1_1000_11_11_00_00, 0b1_0000_11_11_00_00,
];

fn get_se(x: u16) -> u16 { (x & SE) >> 6 }
fn get_sw(x: u16) -> u16 { (x & SW) >> 4 }
fn get_ne(x: u16) -> u16 { (x & NE) >> 2 }
fn get_nw(x: u16) -> u16 { x & NW }

fn get_side_e(x: u16) -> u16 { (x & SLIDE_SIDE_E) >> 11 }
fn get_side_w(x: u16) -> u16 { (x & SLIDE_SIDE_W) >> 10 }
fn get_side_s(x: u16) -> u16 { (x & SLIDE_SIDE_S) >> 9 }
fn get_side_n(x: u16) -> u16 { (x & SLIDE_SIDE_N) >> 8 }

pub fn is_south_neighbour(me: u16, other: u16) -> bool {
    get_se(me) == get_ne(other) &&
    get_sw(me) == get_nw(other) &&
    (
        (me & IS_SLIDE) == 0 && (other & IS_SLIDE) == 0
        ||
        (get_side_s(me) == get_side_n(other))
    )
}

pub fn is_north_neighbour(me: u16, other: u16) -> bool {
    (get_ne(me) == get_se(other)) &&
    (get_nw(me) == get_sw(other)) &&
    (
        ((me & IS_SLIDE) == 0) && ((other & IS_SLIDE) == 0)
        ||
        (get_side_n(me) == get_side_s(other))
    )
}

pub fn is_east_neighbour(me: u16, other: u16) -> bool {
    get_se(me) == get_sw(other) &&
    get_ne(me) == get_nw(other) &&
    (
        (me & IS_SLIDE) == 0 && (other & IS_SLIDE) == 0
        ||
        get_side_e(me) == get_side_w(other)
    )
}

pub fn is_west_neighbour(me: u16, other: u16) -> bool {
    get_sw(me) == get_se(other) &&
    get_nw(me) == get_ne(other) &&
    (
        (me & IS_SLIDE) == 0 && (other & IS_SLIDE) == 0
        ||
        get_side_w(me) == get_side_e(other)
    )
}