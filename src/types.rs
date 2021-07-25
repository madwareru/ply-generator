use crate::wang_info::{BASES_FORWARD, BASES_LEFT, BASES_BACKWARD, BASES_RIGHT};
use serde::Deserialize;
use ply_generator_lib::{V3, PlyData, VPos, merge_meshes};
use crate::generators::{SlideHasSide, gen_outer_corner, gen_inner_corner, gen_side, gen_quad, gen_slide};

#[derive(Debug, Deserialize)]
pub struct Point {
    pub x: f32,
    pub y: f32
}

#[derive(Debug, Deserialize)]
pub struct ControlPoints {
    pub pt0: Point,
    pub pt1: Point,
    pub pt2: Point
}

pub enum Direction {
    Forward,
    Left,
    Backward,
    Right
}
impl Direction {
    pub(crate) fn get_bases(&self) -> ((f32, f32), (f32, f32), (f32, f32)) {
        match self {
            Direction::Forward => BASES_FORWARD,
            Direction::Left => BASES_LEFT,
            Direction::Backward => BASES_BACKWARD,
            Direction::Right => BASES_RIGHT
        }
    }
}

pub enum TilePiece {
    OuterCorner{ direction: Direction, offset: V3 },
    InnerCorner{ direction: Direction, offset: V3 },
    Side{ direction: Direction, offset: V3 },
    Slide{ direction: Direction, offset: V3, has_side: SlideHasSide},
    Quad{ height: f32, offset: V3}
}
impl TilePiece {
    pub fn gen_mesh(&self, points: &ControlPoints, origin: V3, central_k: f32) -> PlyData<VPos> {
        match self {
            TilePiece::OuterCorner { direction, offset } => {
                let ((basis_1_x, basis_1_z),
                    (basis_2_x, basis_2_z),
                    (central_basis_x, central_basis_z)
                ) = direction.get_bases();
                let true_origin = V3 {
                    x: origin.x + offset.x,
                    y: origin.y + offset.y,
                    z: origin.z + offset.z
                };
                gen_outer_corner(
                    points,
                    true_origin,
                    basis_1_x, basis_1_z,
                    basis_2_x, basis_2_z,
                    central_basis_x, central_basis_z,
                    central_k
                )
            }
            TilePiece::InnerCorner { direction, offset } => {
                let ((basis_1_x, basis_1_z),
                    (basis_2_x, basis_2_z),
                    (central_basis_x, central_basis_z)
                ) = direction.get_bases();
                let true_origin = V3 {
                    x: origin.x + offset.x,
                    y: origin.y + offset.y,
                    z: origin.z + offset.z
                };
                gen_inner_corner(
                    points,
                    true_origin,
                    basis_1_x, basis_1_z,
                    basis_2_x, basis_2_z,
                    central_basis_x, central_basis_z,
                    central_k
                )
            }
            TilePiece::Side { direction, offset } => {
                let ((basis_1_x, basis_1_z),
                    (basis_2_x, basis_2_z),
                    _
                ) = direction.get_bases();
                let true_origin = V3 {
                    x: origin.x + offset.x,
                    y: origin.y + offset.y,
                    z: origin.z + offset.z
                };
                gen_side(
                    points,
                    true_origin,
                    basis_1_x, basis_1_z,
                    basis_2_x, basis_2_z,
                    central_k
                )
            }
            TilePiece::Quad { height, offset } => {
                let true_origin = V3 {
                    x: origin.x + offset.x,
                    y: origin.y + offset.y,
                    z: origin.z + offset.z
                };
                gen_quad(true_origin, *height)
            }
            TilePiece::Slide { direction, offset, has_side } => {
                let ((basis_1_x, basis_1_z),
                    (basis_2_x, basis_2_z),
                    _
                ) = direction.get_bases();
                let true_origin = V3 {
                    x: origin.x + offset.x,
                    y: origin.y + offset.y,
                    z: origin.z + offset.z
                };
                gen_slide(
                    points,
                    true_origin,
                    basis_1_x, basis_1_z,
                    basis_2_x, basis_2_z,
                    *has_side
                )
            }
        }
    }
}

pub struct Tile(pub TilePiece, pub TilePiece, pub TilePiece, pub TilePiece);
impl Tile {
    pub fn gen_mesh(&self, points: &ControlPoints, origin: V3, central_k: f32) -> PlyData<VPos> {
        merge_meshes(&[
            self.0.gen_mesh(points, origin, central_k),
            self.1.gen_mesh(points, origin, central_k),
            self.2.gen_mesh(points, origin, central_k),
            self.3.gen_mesh(points, origin, central_k)
        ])
    }
}