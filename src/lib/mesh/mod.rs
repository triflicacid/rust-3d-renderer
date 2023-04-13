pub mod create;

use colorsys::Rgb;

use crate::lib::triangle::Triangle;
use crate::lib::maths::vec::Vec3d;

pub struct Mesh {
    pub tris: Vec<Triangle>,
}

impl Mesh {
    /// Construct a new mesh
    pub fn new() -> Mesh {
        Mesh { tris: Vec::new() }
    }

    /// Construct a mesh from triangles
    pub fn from(tris: Vec<Triangle>) -> Mesh {
        Mesh { tris }
    }

    /// Merge a mesh with self
    pub fn merge(&mut self, mut other: Mesh) {
        self.tris.append(&mut other.tris);
    }

    /// Merge given triangle into mesh
    pub fn merge_tri(&mut self, tri: Triangle) {
        self.tris.push(tri);
    }

    /// Merge given triangles into mesh
    pub fn merge_tris(&mut self, mut tris: Vec<Triangle>) {
        self.tris.append(&mut tris);
    }

    /// Set fill of each triangle
    pub fn fill(&mut self, color: Option<Rgb>) {
        for i in 0..self.tris.len() {
            self.tris[i].fill = color.clone();
        }
    }

    /// Set stroke of each triangle
    pub fn stroke(&mut self, color: Option<Rgb>) {
        for i in 0..self.tris.len() {
            self.tris[i].stroke = color.clone();
        }
    }
}
