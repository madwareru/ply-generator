use ply_generator_lib::{V3, PlyData, VPos, flip_normals};
use crate::types::{ControlPoints, Point};

pub fn gen_quad(origin: V3, height: f32) -> PlyData<VPos> {
    let vertices = vec![
        VPos { position: V3{
            x: origin.x,
            y: origin.y + height,
            z: origin.z
        }},
        VPos { position: V3{
            x: origin.x,
            y: origin.y + height,
            z: origin.z + 1.0
        }},
        VPos { position: V3{
            x: origin.x + 1.0,
            y: origin.y + height,
            z: origin.z
        }},
        VPos { position: V3{
            x: origin.x + 1.0,
            y: origin.y + height,
            z: origin.z + 1.0
        }}
    ];
    let faces = vec! [
        vec![0, 1, 2],
        vec![2, 1, 3]
    ];
    PlyData {
        vertices,
        faces
    }
}

pub fn gen_outer_corner(
    points: &ControlPoints,
    origin: V3,
    basis_1_x: f32, basis_1_z: f32,
    basis_2_x: f32, basis_2_z: f32,
    central_basis_x: f32, central_basis_z: f32,
    central_k: f32
) -> PlyData<VPos> {
    let pt_high = Point { x: 0.0, y: 1.0 };
    let pt_low = Point { x: 1.0, y: 0.0 };

    let vertices = vec![
        VPos { //0
            position: V3 {
                x: origin.x,
                y: origin.y + pt_high.y,
                z: origin.z
            },
        },
        VPos { //1
            position: V3 {
                x: origin.x,
                y: origin.y + pt_high.y,
                z: origin.z
            },
        },
        VPos { //2
            position: V3 {
                x: origin.x + basis_1_x * points.pt0.x,
                y: origin.y + points.pt0.y,
                z: origin.z + basis_1_z * points.pt0.x
            },
        },
        VPos { //3
            position: V3 {
                x: origin.x + central_basis_x * central_k * points.pt0.x,
                y: origin.y + points.pt0.y,
                z: origin.z + central_basis_z * central_k * points.pt0.x
            },
        },
        VPos { //4
            position: V3 {
                x: origin.x + central_basis_x * central_k * points.pt0.x,
                y: origin.y + points.pt0.y,
                z: origin.z + central_basis_z * central_k * points.pt0.x
            },
        },
        VPos { //5
            position: V3 {
                x: origin.x + basis_2_x * points.pt0.x,
                y: origin.y + points.pt0.y,
                z: origin.z + basis_2_z * points.pt0.x
            },
        },
        VPos { //6
            position: V3 {
                x: origin.x + basis_1_x * points.pt0.x,
                y: origin.y + points.pt0.y,
                z: origin.z + basis_1_z * points.pt0.x
            },
        },
        VPos { //7
            position: V3 {
                x: origin.x + central_basis_x * central_k * points.pt0.x,
                y: origin.y + points.pt0.y,
                z: origin.z + central_basis_z * central_k * points.pt0.x
            },
        },
        VPos { //8
            position: V3 {
                x: origin.x + central_basis_x * central_k * points.pt0.x,
                y: origin.y + points.pt0.y,
                z: origin.z + central_basis_z * central_k * points.pt0.x
            },
        },
        VPos { //9
            position: V3 {
                x: origin.x + basis_2_x * points.pt0.x,
                y: origin.y + points.pt0.y,
                z: origin.z + basis_2_z * points.pt0.x
            },
        },

        VPos { //10
            position: V3 {
                x: origin.x + basis_1_x * points.pt1.x,
                y: origin.y + points.pt1.y,
                z: origin.z + basis_1_z * points.pt1.x
            }
        },
        VPos { //11
            position: V3 {
                x: origin.x + central_basis_x * central_k * points.pt1.x,
                y: origin.y + points.pt1.y,
                z: origin.z + central_basis_z * central_k * points.pt1.x
            },
        },
        VPos { //12
            position: V3 {
                x: origin.x + central_basis_x * central_k * points.pt1.x,
                y: origin.y + points.pt1.y,
                z: origin.z + central_basis_z * central_k * points.pt1.x
            },
        },
        VPos { //13
            position: V3 {
                x: origin.x + basis_2_x * points.pt1.x,
                y: origin.y + points.pt1.y,
                z: origin.z + basis_2_z * points.pt1.x
            },
        },
        VPos { //14
            position: V3 {
                x: origin.x + basis_1_x * points.pt1.x,
                y: origin.y + points.pt1.y,
                z: origin.z + basis_1_z * points.pt1.x
            }
        },
        VPos { //15
            position: V3 {
                x: origin.x + central_basis_x * central_k * points.pt1.x,
                y: origin.y + points.pt1.y,
                z: origin.z + central_basis_z * central_k * points.pt1.x
            },
        },
        VPos { //16
            position: V3 {
                x: origin.x + central_basis_x * central_k * points.pt1.x,
                y: origin.y + points.pt1.y,
                z: origin.z + central_basis_z * central_k * points.pt1.x
            },
        },
        VPos { //17
            position: V3 {
                x: origin.x + basis_2_x * points.pt1.x,
                y: origin.y + points.pt1.y,
                z: origin.z + basis_2_z * points.pt1.x
            },
        },

        VPos { //18
            position: V3 {
                x: origin.x + basis_1_x * points.pt2.x,
                y: origin.y + points.pt2.y,
                z: origin.z + basis_1_z * points.pt2.x
            },
        },
        VPos { //19
            position: V3 {
                x: origin.x + central_basis_x * central_k * points.pt2.x,
                y: origin.y + points.pt2.y,
                z: origin.z + central_basis_z * central_k * points.pt2.x
            },
        },
        VPos { //20
            position: V3 {
                x: origin.x + central_basis_x * central_k * points.pt2.x,
                y: origin.y + points.pt2.y,
                z: origin.z + central_basis_z * central_k * points.pt2.x
            },
        },
        VPos { //21
            position: V3 {
                x: origin.x + basis_2_x * points.pt2.x,
                y: origin.y + points.pt2.y,
                z: origin.z + basis_2_z * points.pt2.x
            },
        },
        VPos { //22
            position: V3 {
                x: origin.x + basis_1_x * points.pt2.x,
                y: origin.y + points.pt2.y,
                z: origin.z + basis_1_z * points.pt2.x
            },
        },
        VPos { //23
            position: V3 {
                x: origin.x + central_basis_x * central_k * points.pt2.x,
                y: origin.y + points.pt2.y,
                z: origin.z + central_basis_z * central_k * points.pt2.x
            },
        },
        VPos { //24
            position: V3 {
                x: origin.x + central_basis_x * central_k * points.pt2.x,
                y: origin.y + points.pt2.y,
                z: origin.z + central_basis_z * central_k * points.pt2.x
            },
        },
        VPos { //25
            position: V3 {
                x: origin.x + basis_2_x * points.pt2.x,
                y: origin.y + points.pt2.y,
                z: origin.z + basis_2_z * points.pt2.x
            },
        },

        VPos { //26
            position: V3 {
                x: origin.x + basis_1_x * pt_low.x,
                y: origin.y + pt_low.y,
                z: origin.z + basis_1_z * pt_low.x
            },
        },
        VPos { //27
            position: V3 {
                x: origin.x + central_basis_x * central_k * pt_low.x,
                y: origin.y + pt_low.y,
                z: origin.z + central_basis_z * central_k * pt_low.x
            },
        },
        VPos { //28
            position: V3 {
                x: origin.x + central_basis_x * central_k * pt_low.x,
                y: origin.y + pt_low.y,
                z: origin.z + central_basis_z * central_k * pt_low.x
            },
        },
        VPos { //29
            position: V3 {
                x: origin.x + basis_2_x * pt_low.x,
                y: origin.y + pt_low.y,
                z: origin.z + basis_2_z * pt_low.x
            },
        },
        VPos { //30
            position: V3 {
                x: origin.x + basis_1_x * pt_low.x,
                y: origin.y + pt_low.y,
                z: origin.z + basis_1_z * pt_low.x
            },
        },
        VPos { //31
            position: V3 {
                x: origin.x + central_basis_x * central_k * pt_low.x,
                y: origin.y + pt_low.y,
                z: origin.z + central_basis_z * central_k * pt_low.x
            },
        },
        VPos { //32
            position: V3 {
                x: origin.x + central_basis_x * central_k * pt_low.x,
                y: origin.y + pt_low.y,
                z: origin.z + central_basis_z * central_k * pt_low.x
            },
        },
        VPos { //33
            position: V3 {
                x: origin.x + basis_2_x * pt_low.x,
                y: origin.y + pt_low.y,
                z: origin.z + basis_2_z * pt_low.x
            },
        },

        VPos { //34
            position: V3 {
                x: origin.x + (basis_1_x + basis_2_x) * pt_low.x,
                y: origin.y + pt_low.y,
                z: origin.z + (basis_1_z + basis_2_z) * pt_low.x
            },
        },
        VPos { //35
            position: V3 {
                x: origin.x + (basis_1_x + basis_2_x) * pt_low.x,
                y: origin.y + pt_low.y,
                z: origin.z + (basis_1_z + basis_2_z) * pt_low.x
            },
        },
    ];

    let faces = vec![
        vec![0, 3, 2],
        vec![1, 5, 4],
        vec![6, 7, 11, 10],
        vec![8, 9, 13, 12],
        vec![14, 15, 19, 18],
        vec![16, 17, 21, 20],
        vec![22, 23, 27, 26],
        vec![24, 25, 29, 28],
        vec![30, 31, 34],
        vec![32, 33, 35],
    ];

    PlyData { vertices, faces }
}

pub fn gen_inner_corner(
    points: &ControlPoints,
    origin: V3,
    basis_1_x: f32, basis_1_z: f32,
    basis_2_x: f32, basis_2_z: f32,
    central_basis_x: f32, central_basis_z: f32,
    central_k: f32
) -> PlyData<VPos> {
    let pt_high = Point { x: 0.0, y: 1.0 };
    let pt_low = Point { x: 1.0, y: 0.0 };

    let origin0 = V3 {
        x: origin.x + basis_2_x,
        y: origin.y,
        z: origin.z + basis_2_z
    };

    let origin1 = V3 {
        x: origin.x + basis_1_x,
        y: origin.y,
        z: origin.z + basis_1_z
    };

    let origin_c = V3 {
        x: origin.x + central_basis_x * central_k / 1.6,
        y: origin.y,
        z: origin.z + central_basis_z * central_k / 1.6
    };

    let vertices = vec![
        VPos { //0
            position: V3 {
                x: origin.x,
                y: origin.y + pt_high.y,
                z: origin.z
            },
        },
        VPos { //1
            position: V3 {
                x: origin.x,
                y: origin.y + pt_high.y,
                z: origin.z
            },
        },

        VPos { //2
            position: V3 {
                x: origin0.x,
                y: origin0.y + pt_high.y,
                z: origin0.z
            },
        },
        VPos { //3
            position: V3 {
                x: origin_c.x,
                y: origin_c.y + pt_high.y,
                z: origin_c.z
            },
        },
        VPos { //4
            position: V3 {
                x: origin_c.x,
                y: origin_c.y + pt_high.y,
                z: origin_c.z
            },
        },
        VPos { //5
            position: V3 {
                x: origin1.x,
                y: origin1.y + pt_high.y,
                z: origin1.z
            },
        },
        VPos { //6
            position: V3 {
                x: origin0.x,
                y: origin0.y + pt_high.y,
                z: origin0.z
            },
        },
        VPos { //7
            position: V3 {
                x: origin_c.x,
                y: origin_c.y + pt_high.y,
                z: origin_c.z
            },
        },
        VPos { //8
            position: V3 {
                x: origin_c.x,
                y: origin_c.y + pt_high.y,
                z: origin_c.z
            },
        },
        VPos { //9
            position: V3 {
                x: origin1.x,
                y: origin1.y + pt_high.y,
                z: origin1.z
            },
        },

        VPos { //10
            position: V3 {
                x: origin0.x + basis_1_x * points.pt0.x,
                y: origin0.y + points.pt0.y,
                z: origin0.z + basis_1_z * points.pt0.x
            },
        },
        VPos { //11
            position: V3 {
                x: origin_c.x + central_basis_x * points.pt0.x,
                y: origin_c.y + points.pt0.y,
                z: origin_c.z + central_basis_z * points.pt0.x
            },
        },
        VPos { //12
            position: V3 {
                x: origin_c.x + central_basis_x * points.pt0.x,
                y: origin_c.y + points.pt0.y,
                z: origin_c.z + central_basis_z * points.pt0.x
            },
        },
        VPos { //13
            position: V3 {
                x: origin1.x + basis_2_x * points.pt0.x,
                y: origin1.y + points.pt0.y,
                z: origin1.z + basis_2_z * points.pt0.x
            },
        },
        VPos { //14
            position: V3 {
                x: origin0.x + basis_1_x * points.pt0.x,
                y: origin0.y + points.pt0.y,
                z: origin0.z + basis_1_z * points.pt0.x
            },
        },
        VPos { //15
            position: V3 {
                x: origin_c.x + central_basis_x * points.pt0.x,
                y: origin_c.y + points.pt0.y,
                z: origin_c.z + central_basis_z * points.pt0.x
            },
        },
        VPos { //16
            position: V3 {
                x: origin_c.x + central_basis_x * points.pt0.x,
                y: origin_c.y + points.pt0.y,
                z: origin_c.z + central_basis_z * points.pt0.x
            },
        },
        VPos { //17
            position: V3 {
                x: origin1.x + basis_2_x * points.pt0.x,
                y: origin1.y + points.pt0.y,
                z: origin1.z + basis_2_z * points.pt0.x
            },
        },

        VPos { //18
            position: V3 {
                x: origin0.x + basis_1_x * points.pt1.x,
                y: origin0.y + points.pt1.y,
                z: origin0.z + basis_1_z * points.pt1.x
            },
        },
        VPos { //19
            position: V3 {
                x: origin_c.x + central_basis_x * points.pt1.x,
                y: origin_c.y + points.pt1.y,
                z: origin_c.z + central_basis_z * points.pt1.x
            },
        },
        VPos { //20
            position: V3 {
                x: origin_c.x + central_basis_x * points.pt1.x,
                y: origin_c.y + points.pt1.y,
                z: origin_c.z + central_basis_z * points.pt1.x
            },
        },
        VPos { //21
            position: V3 {
                x: origin1.x + basis_2_x * points.pt1.x,
                y: origin1.y + points.pt1.y,
                z: origin1.z + basis_2_z * points.pt1.x
            },
        },
        VPos { //22
            position: V3 {
                x: origin0.x + basis_1_x * points.pt1.x,
                y: origin0.y + points.pt1.y,
                z: origin0.z + basis_1_z * points.pt1.x
            },
        },
        VPos { //23
            position: V3 {
                x: origin_c.x + central_basis_x * points.pt1.x,
                y: origin_c.y + points.pt1.y,
                z: origin_c.z + central_basis_z * points.pt1.x
            },
        },
        VPos { //24
            position: V3 {
                x: origin_c.x + central_basis_x * points.pt1.x,
                y: origin_c.y + points.pt1.y,
                z: origin_c.z + central_basis_z * points.pt1.x
            },
        },
        VPos { //25
            position: V3 {
                x: origin1.x + basis_2_x * points.pt1.x,
                y: origin1.y + points.pt1.y,
                z: origin1.z + basis_2_z * points.pt1.x
            },
        },

        VPos { //26
            position: V3 {
                x: origin0.x + basis_1_x * points.pt2.x,
                y: origin0.y + points.pt2.y,
                z: origin0.z + basis_1_z * points.pt2.x
            },
        },
        VPos { //27
            position: V3 {
                x: origin_c.x + central_basis_x * points.pt2.x,
                y: origin_c.y + points.pt2.y,
                z: origin_c.z + central_basis_z * points.pt2.x
            },
        },
        VPos { //28
            position: V3 {
                x: origin_c.x + central_basis_x * points.pt2.x,
                y: origin_c.y + points.pt2.y,
                z: origin_c.z + central_basis_z * points.pt2.x
            },
        },
        VPos { //29
            position: V3 {
                x: origin1.x + basis_2_x * points.pt2.x,
                y: origin1.y + points.pt2.y,
                z: origin1.z + basis_2_z * points.pt2.x
            },
        },
        VPos { //30
            position: V3 {
                x: origin0.x + basis_1_x * points.pt2.x,
                y: origin0.y + points.pt2.y,
                z: origin0.z + basis_1_z * points.pt2.x
            },
        },
        VPos { //31
            position: V3 {
                x: origin_c.x + central_basis_x * points.pt2.x,
                y: origin_c.y + points.pt2.y,
                z: origin_c.z + central_basis_z * points.pt2.x
            },
        },
        VPos { //32
            position: V3 {
                x: origin_c.x + central_basis_x * points.pt2.x,
                y: origin_c.y + points.pt2.y,
                z: origin_c.z + central_basis_z * points.pt2.x
            },
        },
        VPos { //33
            position: V3 {
                x: origin1.x + basis_2_x * points.pt2.x,
                y: origin1.y + points.pt2.y,
                z: origin1.z + basis_2_z * points.pt2.x
            },
        },

        VPos { //34
            position: V3 {
                x: origin1.x + basis_2_x * pt_low.x,
                y: origin1.y + pt_low.y,
                z: origin1.z + basis_2_z * pt_low.x
            },
        },
        VPos { //35
            position: V3 {
                x: origin1.x + basis_2_x * pt_low.x,
                y: origin1.y + pt_low.y,
                z: origin1.z + basis_2_z * pt_low.x
            },
        },
    ];

    let faces = vec![
        vec![0, 2, 3],
        vec![1, 4, 5],
        vec![10, 11, 7, 6],
        vec![12, 13, 9, 8],
        vec![18, 19, 15, 14],
        vec![20, 21, 17, 16],
        vec![26, 27, 23, 22],
        vec![28, 29, 25, 24],
        vec![30, 34, 31],
        vec![32, 35, 33],
    ];

    PlyData { vertices, faces }
}

pub fn gen_side(
    points: &ControlPoints,
    origin: V3,
    basis_1_x: f32, basis_1_z: f32,
    basis_2_x: f32, basis_2_z: f32,
    central_k: f32
) -> PlyData<VPos> {
    let pt_high = Point { x: 0.0, y: 1.0 };
    let pt_low = Point { x: 1.0, y: 0.0 };

    let origin_0 = origin;
    let origin_1 = V3 { x: origin.x + basis_1_x, z: origin.z + basis_1_z, ..origin };
    let origin_c = V3 { x: origin.x + basis_1_x * 0.5, z: origin.z + basis_1_z * 0.5, ..origin };

    let vertices = vec![
        VPos { position: V3 { //0
            x: origin_0.x + pt_high.x * basis_2_x,
            y: origin.y + pt_high.y,
            z: origin_0.z + pt_high.x * basis_2_z
        } },
        VPos { position: V3 { //1
            x: origin_1.x + pt_high.x * basis_2_x,
            y: origin.y + pt_high.y,
            z: origin_1.z + pt_high.x * basis_2_z
        } },
        VPos { position: V3 { //2
            x: origin_0.x + points.pt0.x * basis_2_x,
            y: origin.y + points.pt0.y,
            z: origin_0.z + points.pt0.x * basis_2_z
        } },
        VPos { position: V3 { //3
            x: origin_1.x + points.pt0.x * basis_2_x,
            y: origin.y + points.pt0.y,
            z: origin_1.z + points.pt0.x * basis_2_z
        } },
        VPos { position: V3 { //4
            x: origin_0.x + points.pt0.x * basis_2_x,
            y: origin.y + points.pt0.y,
            z: origin_0.z + points.pt0.x * basis_2_z
        } },
        VPos { position: V3 { //5
            x: origin_1.x + points.pt0.x * basis_2_x,
            y: origin.y + points.pt0.y,
            z: origin_1.z + points.pt0.x * basis_2_z
        } },
        VPos { position: V3 { //6
            x: origin_0.x + points.pt1.x * basis_2_x,
            y: origin.y + points.pt1.y,
            z: origin_0.z + points.pt1.x * basis_2_z
        } },
        VPos { position: V3 { //7
            x: origin_1.x + points.pt1.x * basis_2_x,
            y: origin.y + points.pt1.y,
            z: origin_1.z + points.pt1.x * basis_2_z
        } },
        VPos { position: V3 { //8
            x: origin_0.x + points.pt1.x * basis_2_x,
            y: origin.y + points.pt1.y,
            z: origin_0.z + points.pt1.x * basis_2_z
        } },
        VPos { position: V3 { //9
            x: origin_1.x + points.pt1.x * basis_2_x,
            y: origin.y + points.pt1.y,
            z: origin_1.z + points.pt1.x * basis_2_z
        } },
        VPos { position: V3 { //10
            x: origin_0.x + points.pt2.x * basis_2_x,
            y: origin.y + points.pt2.y,
            z: origin_0.z + points.pt2.x * basis_2_z
        } },
        VPos { position: V3 { //11
            x: origin_1.x + points.pt2.x * basis_2_x,
            y: origin.y + points.pt2.y,
            z: origin_1.z + points.pt2.x * basis_2_z
        } },
        VPos { position: V3 { //12
            x: origin_0.x + points.pt2.x * basis_2_x,
            y: origin.y + points.pt2.y,
            z: origin_0.z + points.pt2.x * basis_2_z
        } },
        VPos { position: V3 { //13
            x: origin_1.x + points.pt2.x * basis_2_x,
            y: origin.y + points.pt2.y,
            z: origin_1.z + points.pt2.x * basis_2_z
        } },
        VPos { position: V3 { //14
            x: origin_0.x + pt_low.x * basis_2_x,
            y: origin.y + pt_low.y,
            z: origin_0.z + pt_low.x * basis_2_z
        } },
        VPos { position: V3 { //15
            x: origin_1.x + pt_low.x * basis_2_x,
            y: origin.y + pt_low.y,
            z: origin_1.z + pt_low.x * basis_2_z
        } },
        VPos { position: V3 { //16
            x: origin_c.x + points.pt0.x * basis_2_x * central_k,
            y: origin.y + points.pt0.y,
            z: origin_c.z + points.pt0.x * basis_2_z * central_k
        } },
        VPos { position: V3 { //17
            x: origin_c.x + points.pt0.x * basis_2_x * central_k,
            y: origin.y + points.pt0.y,
            z: origin_c.z + points.pt0.x * basis_2_z * central_k
        } },
        VPos { position: V3 { //18
            x: origin_c.x + points.pt0.x * basis_2_x * central_k,
            y: origin.y + points.pt0.y,
            z: origin_c.z + points.pt0.x * basis_2_z * central_k
        } },
        VPos { position: V3 { //19
            x: origin_c.x + points.pt0.x * basis_2_x * central_k,
            y: origin.y + points.pt0.y,
            z: origin_c.z + points.pt0.x * basis_2_z * central_k
        } },
        VPos { position: V3 { //20
            x: origin_c.x + points.pt0.x * basis_2_x * central_k,
            y: origin.y + points.pt0.y,
            z: origin_c.z + points.pt0.x * basis_2_z * central_k
        } },
        VPos { position: V3 { //21
            x: origin_c.x + points.pt1.x * basis_2_x * central_k,
            y: origin.y + points.pt1.y,
            z: origin_c.z + points.pt1.x * basis_2_z * central_k
        } },
        VPos { position: V3 { //22
            x: origin_c.x + points.pt1.x * basis_2_x * central_k,
            y: origin.y + points.pt1.y,
            z: origin_c.z + points.pt1.x * basis_2_z * central_k
        } },
        VPos { position: V3 { //23
            x: origin_c.x + points.pt1.x * basis_2_x * central_k,
            y: origin.y + points.pt1.y,
            z: origin_c.z + points.pt1.x * basis_2_z * central_k
        } },
        VPos { position: V3 { //24
            x: origin_c.x + points.pt1.x * basis_2_x * central_k,
            y: origin.y + points.pt1.y,
            z: origin_c.z + points.pt1.x * basis_2_z * central_k
        } },
        VPos { position: V3 { //25
            x: origin_c.x + points.pt2.x * basis_2_x * central_k,
            y: origin.y + points.pt2.y,
            z: origin_c.z + points.pt2.x * basis_2_z * central_k
        } },
        VPos { position: V3 { //26
            x: origin_c.x + points.pt2.x * basis_2_x * central_k,
            y: origin.y + points.pt2.y,
            z: origin_c.z + points.pt2.x * basis_2_z * central_k
        } },
        VPos { position: V3 { //27
            x: origin_c.x + points.pt2.x * basis_2_x * central_k,
            y: origin.y + points.pt2.y,
            z: origin_c.z + points.pt2.x * basis_2_z * central_k
        } },
        VPos { position: V3 { //28
            x: origin_c.x + points.pt2.x * basis_2_x * central_k,
            y: origin.y + points.pt2.y,
            z: origin_c.z + points.pt2.x * basis_2_z * central_k
        } },
        VPos { position: V3 { //29
            x: origin_c.x + points.pt2.x * basis_2_x * central_k,
            y: origin.y + points.pt2.y,
            z: origin_c.z + points.pt2.x * basis_2_z * central_k
        } },
    ];

    let faces = vec![
        vec![1, 0, 16],
        vec![1, 17, 3],
        vec![0, 2, 18],
        vec![5, 19, 21, 7],
        vec![20, 4, 6, 22],
        vec![9, 23, 25, 11],
        vec![24, 8, 10, 26],
        vec![13, 27, 15],
        vec![15, 28, 14],
        vec![29, 12, 14]
    ];

    PlyData {
        vertices,
        faces
    }
}

#[derive(Copy, Clone)]
pub enum SlideHasSide {
    AtLeft,
    AtRight,
    Nowhere
}
pub fn gen_slide(
    points: &ControlPoints,
    origin: V3,
    basis_1_x: f32, basis_1_z: f32,
    basis_2_x: f32, basis_2_z: f32,
    has_side: SlideHasSide
) -> PlyData<VPos> {
    let origin_0 = origin;
    let origin_1 = V3 { x: origin.x + basis_1_x, z: origin.z + basis_1_z, ..origin };

    let mut vertices = vec![
        VPos{ position: V3 { x: origin_0.x, y: origin_0.y + 1.0, z: origin_0.z }}, //0
        VPos{ position: V3 { x: origin_1.x, y: origin_1.y + 1.0, z: origin_1.z }}, //1
        VPos{ position: V3 { //2
            x: origin_0.x + basis_2_x * 0.25,
            y: origin_0.y + 0.75,
            z: origin_0.z + basis_2_z * 0.25 }},
        VPos{ position: V3 { //3
            x: origin_1.x + basis_2_x * 0.25,
            y: origin_1.y + 0.75,
            z: origin_1.z + basis_2_z * 0.25
        }},
        VPos{ position: V3 { //4
            x: origin_0.x + basis_2_x * 0.25,
            y: origin_0.y + 0.75,
            z: origin_0.z + basis_2_z * 0.25 }},
        VPos{ position: V3 { //5
            x: origin_1.x + basis_2_x * 0.25,
            y: origin_1.y + 0.75,
            z: origin_1.z + basis_2_z * 0.25
        }},
        VPos{ position: V3 { //6
            x: origin_0.x + basis_2_x * 0.5,
            y: origin_0.y + 0.5,
            z: origin_0.z + basis_2_z * 0.5 }},
        VPos{ position: V3 { //7
            x: origin_1.x + basis_2_x * 0.5,
            y: origin_1.y + 0.5,
            z: origin_1.z + basis_2_z * 0.5
        }},
        VPos{ position: V3 { //8
            x: origin_0.x + basis_2_x * 0.5,
            y: origin_0.y + 0.5,
            z: origin_0.z + basis_2_z * 0.5 }},
        VPos{ position: V3 { //9
            x: origin_1.x + basis_2_x * 0.5,
            y: origin_1.y + 0.5,
            z: origin_1.z + basis_2_z * 0.5
        }},
        VPos{ position: V3 { //10
            x: origin_0.x + basis_2_x * 0.75,
            y: origin_0.y + 0.25,
            z: origin_0.z + basis_2_z * 0.75 }},
        VPos{ position: V3 { //11
            x: origin_1.x + basis_2_x * 0.75,
            y: origin_1.y + 0.25,
            z: origin_1.z + basis_2_z * 0.75
        }},
        VPos{ position: V3 { //12
            x: origin_0.x + basis_2_x * 0.75,
            y: origin_0.y + 0.25,
            z: origin_0.z + basis_2_z * 0.75 }},
        VPos{ position: V3 { //13
            x: origin_1.x + basis_2_x * 0.75,
            y: origin_1.y + 0.25,
            z: origin_1.z + basis_2_z * 0.75
        }},
        VPos{ position: V3 { //14
            x: origin_0.x + basis_2_x,
            y: origin_0.y,
            z: origin_0.z + basis_2_z }},
        VPos{ position: V3 { //15
            x: origin_1.x + basis_2_x,
            y: origin_1.y,
            z: origin_1.z + basis_2_z
        }},
    ];

    let no_side = match has_side {
        SlideHasSide::AtLeft => {
            for i in 1..=6 {
                vertices[i * 2].position.x += 0.1 * basis_1_x;
                vertices[i * 2].position.z += 0.1 * basis_1_z;
            }

            vertices.push(vertices[0]);
            for _ in 0..2 { vertices.push(VPos{ position: V3 {
                x: origin_0.x + points.pt0.x * basis_2_x,
                y: origin_0.y + points.pt0.y,
                z: origin_0.z + points.pt0.x * basis_2_z
            }})}
            for _ in 0..3 { vertices.push(VPos{ position: V3 {
                x: origin_0.x + points.pt1.x * basis_2_x,
                y: origin_0.y + points.pt1.y,
                z: origin_0.z + points.pt1.x * basis_2_z
            }})}
            for _ in 0..3 { vertices.push(VPos{ position: V3 {
                x: origin_0.x + points.pt2.x * basis_2_x,
                y: origin_0.y + points.pt2.y,
                z: origin_0.z + points.pt2.x * basis_2_z
            }})}
            vertices.push(vertices[14]);
            for _ in 0..3 { vertices.push(vertices[2]); }
            for _ in 0..3 { vertices.push(vertices[6]); }
            for _ in 0..2 { vertices.push(vertices[10]); }
            false
        },
        SlideHasSide::AtRight => {
            for i in 1..=6 {
                vertices[i * 2 + 1].position.x -= 0.1 * basis_1_x;
                vertices[i * 2 + 1].position.z -= 0.1 * basis_1_z;
            }

            vertices.push(vertices[1]);
            for _ in 0..2 { vertices.push(vertices[3]); }
            for _ in 0..3 { vertices.push(vertices[7]); }
            for _ in 0..3 { vertices.push(vertices[11]); }
            vertices.push(vertices[15]);
            for _ in 0..3 { vertices.push(VPos{ position: V3 {
                x: origin_1.x + points.pt0.x * basis_2_x,
                y: origin_1.y + points.pt0.y,
                z: origin_1.z + points.pt0.x * basis_2_z
            }})}
            for _ in 0..3 { vertices.push(VPos{ position: V3 {
                x: origin_1.x + points.pt1.x * basis_2_x,
                y: origin_1.y + points.pt1.y,
                z: origin_1.z + points.pt1.x * basis_2_z
            }})}
            for _ in 0..2 { vertices.push(VPos{ position: V3 {
                x: origin_1.x + points.pt2.x * basis_2_x,
                y: origin_1.y + points.pt2.y,
                z: origin_1.z + points.pt2.x * basis_2_z
            }})}
            false
        },
        SlideHasSide::Nowhere => {
            true
        }
    };

    let mut faces = vec![
        vec![0, 1, 3, 2],
        vec![4, 5, 7, 6],
        vec![8, 9, 11, 10],
        vec![12, 13, 15, 14]
    ];

    if !no_side {
        faces.push(vec![16, 26, 17]);
        faces.push(vec![18, 27, 19]);
        faces.push(vec![28, 29, 20]);
        faces.push(vec![21, 30, 22]);
        faces.push(vec![31, 32, 23]);
        faces.push(vec![24, 33, 25]);
    }

    let mut res = PlyData {
        vertices,
        faces
    };

    flip_normals(&mut res);

    res
}