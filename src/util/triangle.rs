use crate::util::maths::vec::Vec3d;
use colorsys::Rgb;

pub struct Triangle {
    /// Vertices of the triangle. Defined in a clockwise orientation
    pub vertices: (Vec3d, Vec3d, Vec3d),
    // Fill color of the triangle. If nothing, no fill
    pub fill: Option<Rgb>,
    // Stroke color of the triangle. If nothing, no stroke
    pub stroke: Option<Rgb>,
    // Luminance value
    pub lum: f64,
}

impl Clone for Triangle {
    fn clone(&self) -> Self {
        Triangle {
            vertices: (
                self.vertices.0.clone(),
                self.vertices.1.clone(),
                self.vertices.2.clone(),
            ),
            fill: self.fill.clone(),
            stroke: self.stroke.clone(),
            lum: self.lum,
        }
    }
}

impl Triangle {
    /// Construct a new triangle
    pub fn new(a: Vec3d, b: Vec3d, c: Vec3d) -> Triangle {
        Triangle {
            vertices: (a, b, c),
            fill: Some(Rgb::new(255.0, 255.0, 255.0, None)),
            stroke: Some(Rgb::new(0.0, 0.0, 0.0, None)),
            lum: 1.0,
        }
    }

    /// Get normal vector
    pub fn normal(&mut self) -> Vec3d {
        let v1 = self.vertices.1.sub(&self.vertices.0);
        let v2 = self.vertices.2.sub(&self.vertices.0);
        Vec3d::normal(&v1, &v2)
    }

    /// Link together a square face of vertices
    pub fn link_sq(a: &Vec3d, b: &Vec3d, c: &Vec3d, d: &Vec3d) -> Vec<Triangle> {
        vec![
            Triangle::new(a.clone(), b.clone(), c.clone()),
            Triangle::new(a.clone(), c.clone(), d.clone()),
        ]
    }

    /// Get avg `z` component
    pub fn mid_z(&self) -> f32 {
        (self.vertices.0.z + self.vertices.1.z + self.vertices.2.z) / 3.0
    }

    /// Get fill color
    pub fn get_fill(&mut self) -> Option<Rgb> {
        if self.fill.is_some() {
            let rgb = self.fill.clone().unwrap();
            let mut hsl: colorsys::Hsl = rgb.as_ref().into();

            if rgb.red() > 200.0 && rgb.green() > 200.0 && rgb.blue() > 200.0 {
                hsl.set_lightness(self.lum * 100.0);
            } else {
                hsl.set_lightness(self.lum * 50.0);
            }
            
            Some(hsl.as_ref().into())
        } else {
            None
        }
    }

    /// Get stroke color
    pub fn get_stroke(&mut self) -> Option<Rgb> {
        self.stroke.clone()
    }
}
