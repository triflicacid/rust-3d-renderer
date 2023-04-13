use crate::lib::maths::vec::Vec3d;
use colorsys::Rgb;

pub struct Triangle {
    /// Vertices of the triangle. Defined in a clockwise orientation
    pub vertices: (Vec3d, Vec3d, Vec3d),
    // Fill color of the triangle. If nothing, no fill
    pub fill: Option<Rgb>,
    // Stroke color of the triangle. If nothing, no stroke
    pub stroke: Option<Rgb>,
}

impl Triangle {
    /// Construct a new triangle
    pub fn new(a: Vec3d, b: Vec3d, c: Vec3d) -> Triangle {
        Triangle {
            vertices: (a, b, c),
            fill: Some(Rgb::new(255.0, 255.0, 255.0, None)),
            stroke: Some(Rgb::new(0.0, 0.0, 0.0, None)),
        }
    }

    /// Get normal vector
    pub fn normal(&mut self) -> Vec3d {
        let v1 = self.vertices.1.sub(&self.vertices.0);
        let v2 = self.vertices.2.sub(&self.vertices.0);
        Vec3d::normal(&v1, &v2)
    }

    /// Copy
    pub fn copy(&mut self) -> Triangle {
        Triangle {
            vertices: (
                self.vertices.0.copy(),
                self.vertices.1.copy(),
                self.vertices.2.copy(),
            ),
            fill: self.fill.clone(),
            stroke: self.stroke.clone(),
        }
    }

    /// Link together a square face of vertices
    pub fn link_sq(mut a: Vec3d, mut b: Vec3d, mut c: Vec3d, mut d: Vec3d) -> Vec<Triangle> {
        vec![
            Triangle::new(a.copy(), b.copy(), c.copy()),
            Triangle::new(a.copy(), c.copy(), d.copy()),
        ]
    }

}
