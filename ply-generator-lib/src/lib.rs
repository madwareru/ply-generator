use std::fmt::{Formatter};

#[derive(Copy, Clone)]
pub struct V3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}
impl std::fmt::Display for V3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} {} {}", self.x, self.y, self.z))
    }
}

#[derive(Copy, Clone)]
pub struct V2 {
    pub x: f32,
    pub y: f32
}
impl std::fmt::Display for V2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} {}", self.x, self.y))
    }
}

#[derive(Copy, Clone)]
pub struct V4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}
impl std::fmt::Display for V4 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} {} {} {}", self.x, self.y, self.z, self.w))
    }
}

pub trait Vertex {
    fn description() -> String;
    fn print(&self) -> String;
    fn negate_z(&mut self);
}

pub trait NormalCarrier {
    fn flip_normals(&mut self);
}

macro_rules! vertex_impl {
    ($typ:ident |> $($prop:literal )>+ |> $($prop2:ident)>+) => {
        impl Vertex for $typ {
            fn negate_z(&mut self) {
                self.position.z = -self.position.z;
            }
            fn description() -> String {
                let mut s = String::new();
                $(s += &format!("property float {}\n", $prop);)+
                s
            }
            fn print(&self) -> String {
                let mut s = String::new();
                $(if(s.len() > 0) { s += " "; } s += &format!("{}", self.$prop2);)+
                s += "\n";
                s
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct VPosNorm {
    pub position: V3,
    pub normal: V3,
}
vertex_impl! [
    VPosNorm |>
    "x" > "y" > "z" >
    "nx" > "ny" > "nz" |>
    position > normal
];
impl NormalCarrier for VPosNorm {
    fn flip_normals(&mut self) {
        self.normal.x = -self.normal.x;
        self.normal.y = -self.normal.y;
        self.normal.z = -self.normal.z;
    }
}

#[derive(Copy, Clone)]
pub struct VPosNormColor {
    pub position: V3,
    pub normal: V3,
    pub color: V4
}
vertex_impl! [
    VPosNormColor |>
    "x" > "y" > "z" >
    "nx" > "ny" > "nz" >
    "red" > "green" > "blue" > "alpha" |>
    position > normal > color
];
impl NormalCarrier for VPosNormColor {
    fn flip_normals(&mut self) {
        self.normal.x = -self.normal.x;
        self.normal.y = -self.normal.y;
        self.normal.z = -self.normal.z;
    }
}

#[derive(Copy, Clone)]
pub struct VPosNormUV {
    pub position: V3,
    pub normal: V3,
    pub uv: V2
}
vertex_impl! [
    VPosNormUV |>
    "x" > "y" > "z" >
    "nx" > "ny" > "nz" >
    "u" > "v" |>
    position > normal > uv
];
impl NormalCarrier for VPosNormUV {
    fn flip_normals(&mut self) {
        self.normal.x = -self.normal.x;
        self.normal.y = -self.normal.y;
        self.normal.z = -self.normal.z;
    }
}

#[derive(Copy, Clone)]
pub struct VPosNormUVColor {
    pub position: V3,
    pub normal: V3,
    pub uv: V2,
    pub color: V4
}
vertex_impl! [
    VPosNormUVColor |>
    "x" > "y" > "z" >
    "nx" > "ny" > "nz" >
    "u" > "v" >
    "red" > "green" > "blue" > "alpha" |>
    position > normal > uv > color
];
impl NormalCarrier for VPosNormUVColor {
    fn flip_normals(&mut self) {
        self.normal.x = -self.normal.x;
        self.normal.y = -self.normal.y;
        self.normal.z = -self.normal.z;
    }
}

#[derive(Copy, Clone)]
pub struct VPos {
    pub position: V3
}
vertex_impl! [VPos |> "x" > "y" > "z" |> position];
impl NormalCarrier for VPos {
    fn flip_normals(&mut self) {}
}

#[derive(Copy, Clone)]
pub struct VPosUV {
    pub position: V3,
    pub uv: V2
}
vertex_impl! [VPosUV |> "x" > "y" > "z" > "u" > "v" |> position > uv];
impl NormalCarrier for VPosUV {
    fn flip_normals(&mut self) {}
}

pub struct PlyData<V: Vertex> {
    pub vertices: Vec<V>,
    pub faces: Vec<Vec<usize>>
}

pub fn merge_meshes<V: Vertex + Copy>(meshes: &[PlyData<V>]) -> PlyData<V> {
    let mut total_v_capacity = 0;
    let mut total_f_capacity = 0;

    for m in meshes.iter() {
        total_v_capacity += m.vertices.len();
        total_f_capacity += m.faces.len();
    }

    let mut vertices = Vec::with_capacity(total_v_capacity);
    let mut faces = Vec::with_capacity(total_f_capacity);
    let mut mesh_face_offset = 0;

    for m in meshes.iter() {
        for v in m.vertices.iter() {
            vertices.push(*v);
        }
        for f in m.faces.iter() {
            let mut face = Vec::with_capacity(4);
            for idx in f.iter() {
                face.push(mesh_face_offset + *idx);
            }
            faces.push(face);
        }
        mesh_face_offset += m.vertices.len();
    }

    PlyData {
        vertices,
        faces
    }

}

pub fn negate_z_coord<V: Vertex>(ply: &mut PlyData<V>) {
    for vertex in ply.vertices.iter_mut() {
        vertex.negate_z();
    }
}

pub fn flip_normals<V: Vertex + NormalCarrier>(ply: &mut PlyData<V>) {
    for face in ply.faces.iter_mut() {
        face.reverse();
    }
    for v in ply.vertices.iter_mut() {
        v.flip_normals();
    }
}

pub fn print_ply<V: Vertex>(ply: &PlyData<V>, comment: Option<&str>) -> String {
    let mut s = String::with_capacity(0xFFFFFFFFFF);
    s += "ply\n";
    s += "format ascii 1.0\n";
    if let Some(comm) = comment {
        s += &format!("comment {}\n", comm);
    }
    s += &format!("element vertex {}\n", ply.vertices.len());
    s += &V::description();

    s += &format!("element face {}\n", ply.faces.len());
    s += "property list uchar uint vertex_indices\n";
    s += "end_header\n";

    for v in ply.vertices.iter() {
        s += &v.print();
    }

    let mut first = true;
    for f in ply.faces.iter() {
        if !first {
            s += "\n";
        } else {
            first = false;
        }
        s += &format!("{}", f.len());
        for index in f {
            s += &format!(" {}", index);
        }
    }
    s
}

