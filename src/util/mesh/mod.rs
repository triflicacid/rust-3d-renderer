pub mod obj;
pub mod shape;

use colorsys::Rgb;

use crate::{Vec3d, util::triangle::Triangle};

pub struct Mesh {
    vertices: Vec<Vec3d>,
    faces: Vec<(usize, usize, usize, Option<usize>)>, // Faces consist of three vectors describing a triangle, clockwise, and a faceSettings index
    settings: Vec<FaceSettings>,
    pub default_fill: Option<Rgb>,
    pub default_stroke: Option<Rgb>,
}

impl std::fmt::Debug for Mesh {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Mesh object ({} faces)", self.face_count())
    }
}

pub struct FaceSettings {
    pub fill: Option<Rgb>,
    pub stroke: Option<Rgb>,
}

impl Clone for FaceSettings {
    fn clone(&self) -> Self {
        FaceSettings { fill: self.fill.clone(), stroke: self.stroke.clone(), }
    }
}

impl Mesh {
    pub fn new() -> Mesh {
        Mesh {
            vertices: Vec::new(),
            faces: Vec::new(),
            settings: Vec::new(),
            default_fill: Some(Rgb::new(255.0,255.0, 255.0, None)),
            default_stroke: Some(Rgb::new(0.0,0.0, 0.0, None)),
        }
    }

    /// Return index of a vertex (add vertex if not found)
    pub fn add_vertex(&mut self, vertex: &Vec3d) -> usize {
        let index = self.vertices.iter().position(|v| v.equals(&vertex));
        if index.is_some() {
            index.unwrap()
        } else {
            self.add_vertex_force(vertex)
        }
    }

    /// Add a vertex, even if it exists already
    pub fn add_vertex_force(&mut self, vertex: &Vec3d) -> usize {
        self.vertices.push(vertex.clone());
        self.vertices.len() - 1
    }

    /// Add a new style
    pub fn add_style(&mut self, style: &FaceSettings) -> usize {
        self.settings.push(style.clone());
        self.settings.len() - 1
    }

    /// Remove all styles
    pub fn clear_styles(&mut self) {
        self.settings.clear();
        for i in 0..self.faces.len() {
            self.faces[i].3 = None;
        }
    }

    /// Set faceSettings index of every face
    pub fn set_global_style(&mut self, style: Option<usize>) {
        for i in 0..self.faces.len() {
            self.faces[i].3 = style.clone();
        }
    }

    /// Add a triangle (even if it exists, add a new one)
    pub fn add_tri(&mut self, a: usize, b: usize, c: usize, style: &Option<usize>) -> &mut Self {
        let face = (a, b, c, style.clone());
        self.faces.push(face);
        self
    }

    /// Add a quadrilateral (even if it exists, add a new one). Start hottom-letf clockwise
    pub fn add_quad(&mut self, a: usize, b: usize, c: usize, d: usize, style: &Option<usize>) -> &mut Self {
        self.faces.push((a, b, c, style.clone()));
        self.faces.push((a, c, d, style.clone()));
        self
    }

    /// Get the given Vertex
    pub fn compile_vertex(&mut self, i: usize) -> Option<Vec3d> {
        if i < self.vertices.len() {
            Some(self.vertices[i].clone())
        } else {
            None
        }
    }

    /// Count faces
    pub fn face_count(&self) -> usize {
        self.faces.len()
    }

    /// Is this mesh empty?
    pub fn is_empty(&self) -> bool {
        self.faces.len() == 0
    }

    /// Get the given face
    pub fn compile_face(&mut self, i: usize) -> Option<Triangle> {
        if i < self.faces.len() {
            let face = self.faces[i];
            let mut tri = Triangle::new(
                self.vertices[face.0].clone(),
                self.vertices[face.1].clone(),
                self.vertices[face.2].clone(),
            );
            if face.3.is_some() {
                let settings = &self.settings[face.3.unwrap()];
                tri.fill = settings.fill.clone();
                tri.stroke = settings.stroke.clone();
            } else {
                tri.fill = self.default_fill.clone();
                tri.stroke = self.default_stroke.clone();
            }
            Some(tri)
        } else {
            None
        }
    }

    /// Merge another mesh builder. Mutate current builder, and return self. Doesn't mutate other builder.
    pub fn merge(&mut self, other: &Mesh) -> &mut Self {
        if other.is_empty() {
            return self;
        }

        let vertices_start = self.vertices.len();
        let settings_start = self.settings.len();

        // Copy over other's vertices
        for i in 0..other.vertices.len() {
            self.vertices.push(other.vertices[i].clone());
        }

        // Copy over other's stylings
        if other.settings.len() > 0 {
            for i in 0..other.settings.len() {
                self.settings.push(other.settings[i].clone());
            }
        }

        // Copy over default style?
        let default_style = if other.default_fill.is_some() || other.default_stroke.is_some() {
            let style = FaceSettings {
                fill: other.default_fill.clone(),
                stroke: other.default_stroke.clone(),
            };
            self.settings.push(style);
            Some(self.settings.len() - 1)
        } else {
            None
        };

        // Copy other other's faces
        for i in 0..other.faces.len() {
            let face = &other.faces[i];
            let new_face = (
                vertices_start + face.0,
                vertices_start + face.1,
                vertices_start + face.2,
                if face.3.is_some() {
                    Some(settings_start + face.3.unwrap())
                } else {
                    default_style.clone()
                }
            );
            self.faces.push(new_face);
        }

        self
    }

    /// Translate all vertices by another vector
    pub fn translate(&mut self, v: &Vec3d) -> &mut Self {
        for i in 0..self.vertices.len() {
            self.vertices[i] = self.vertices[i].add(v);
        }
        self
    }

    /// Translate all vertices by another vector
    pub fn translatek(&mut self, k: f32) -> &mut Self {
        for i in 0..self.vertices.len() {
            self.vertices[i] = self.vertices[i].addk(k);
        }
        self
    }

    /// Scale every vector by another vector
    pub fn scale(&mut self, v: &Vec3d) -> &mut Self {
        for i in 0..self.vertices.len() {
            self.vertices[i] = self.vertices[i].mul(v);
        }
        self
    }

    /// Scale every vector by a constant
    pub fn scalek(&mut self, k: f32) -> &mut Self {
        for i in 0..self.vertices.len() {
            self.vertices[i] = self.vertices[i].mulk(k);
        }
        self
    }

}
