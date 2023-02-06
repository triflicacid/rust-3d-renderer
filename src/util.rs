use colorsys::Rgb;

// 3 dimensional vector
pub struct Vec3d {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3d {
    /// Construct a new 3D vector
    pub fn new(x: f32, y: f32, z: f32) -> Vec3d {
        Vec3d { x, y, z }
    }

    /// Construct a diagonal vector (vector where all components are equal)
    pub fn diag(k: f32) -> Vec3d {
        Vec3d::new(k, k, k)
    }

    /// Copy
    pub fn copy(&mut self) -> Vec3d {
        Vec3d::new(self.x, self.y, self.z)
    }

    /// Return the origin
    pub fn origin() -> Vec3d {
        Vec3d::new(0.0, 0.0, 0.0)
    }

    /// Return the unit vector
    pub fn unit() -> Vec3d {
        Vec3d::new(1.0, 1.0, 1.0)
    }

    /// Return unit vector in `x` direction
    pub fn x() -> Vec3d {
        Vec3d::new(1.0, 0.0, 0.0)
    }

    /// Return unit vector in `y` direction
    pub fn y() -> Vec3d {
        Vec3d::new(0.0, 1.0, 0.0)
    }

    /// Return unit vector in `z` direction
    pub fn z() -> Vec3d {
        Vec3d::new(0.0, 0.0, 1.0)
    }

    /// Add a scalar and self
    pub fn addk(&mut self, k: f32) -> Vec3d {
        Vec3d::new(
            self.x + k,
            self.y + k,
            self.z + k,
        )
    }

    /// Add vector and self
    pub fn add(&mut self, vec: &Vec3d) -> Vec3d {
        Vec3d::new(
            self.x + vec.x,
            self.y + vec.y,
            self.z + vec.z,
        )
    }

    /// Subtract a vector from self
    pub fn sub(&mut self, vec: &Vec3d) -> Vec3d {
        Vec3d::new(
            self.x - vec.x,
            self.y - vec.y,
            self.z - vec.z,
        )
    }

    /// Multiple self by a vector (x' = x1 * x2, ...)
    pub fn mul(&mut self, vec: &Vec3d) -> Vec3d {
        Vec3d::new(
            self.x * vec.x,
            self.y * vec.y,
            self.z * vec.z,
        )
    }

    /// Multiply self by a scalar
    pub fn mulk(&mut self, k: f32) -> Vec3d {
        Vec3d::new(
            self.x * k,
            self.y * k,
            self.z * k,
        )
    }

    /// Multiple a vector with a matrix. Divide vector components by `w` component.
    pub fn mult_mat(vec: &Vec3d, mat: &Mat4x4) -> Vec3d {
        let mut w: f32 = vec.x * mat.0.3 + vec.y * mat.1.3 + vec.z * mat.2.3 + mat.3.3;
        if w == 0.0 {
            w = 1.0;
        }
        let mut vec = Vec3d::new(
            vec.x * mat.0.0 + vec.y * mat.1.0 + vec.z * mat.2.0 + mat.3.0,
            vec.x * mat.0.1 + vec.y * mat.1.1 + vec.z * mat.2.1 + mat.3.1,
            vec.x * mat.0.2 + vec.y * mat.1.2 + vec.z * mat.2.2 + mat.3.2,
        );
        if w != 0.0 {
            vec.x /= w;
            vec.y /= w;
            vec.z /= w;
        }
        vec
    }

    /// Return normal of two vectors
    pub fn normal(v1: &Vec3d, v2: &Vec3d) -> Vec3d {
        Vec3d::new(
            v1.y * v2.z - v1.z * v2.y,
            v1.z * v2.x - v1.x * v2.z,
            v1.x * v2.y - v1.y * v2.x,
        )
    }

    /// Return the dot product between two vectors
    pub fn dot_product(v1: &Vec3d, v2: &Vec3d) -> f32 {
        v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
    }

    /// Return length of self
    pub fn length(&mut self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Return a normalised varient of this vector
    pub fn normalise(&mut self) -> Vec3d {
        let length = self.length();
        Vec3d::new(self.x / length, self.y / length, self.z / length)
    }
}

pub struct Mat4x4(pub (f32,f32,f32,f32), pub (f32,f32,f32,f32), pub (f32,f32,f32,f32), pub (f32,f32,f32,f32));

impl Mat4x4 {
    /// Construct a 3D rotation matrix around the X axis. Theta in radians
    pub fn rot_x(theta: f32) -> Mat4x4 {
        Mat4x4(
            (1.0, 0.0, 0.0, 0.0),
            (0.0, theta.cos(), -theta.sin(), 0.0),
            (0.0, theta.sin(), theta.cos(), 0.0),
            (0.0, 0.0, 0.0, 1.0),
        )
    }

    /// Construct a 3D rotation matrix around the Y axis. Theta in radians
    pub fn rot_y(theta: f32) -> Mat4x4 {
        Mat4x4(
            (theta.cos(), 0.0, theta.sin(), 0.0),
            (0.0, 1.0, 0.0, 0.0),
            (-theta.sin(), 0.0, theta.cos(), 0.0),
            (0.0, 0.0, 0.0, 1.0),
        )
    }

    /// Construct a 3D rotation matrix around the Z axis. Theta in radians
    pub fn rot_z(theta: f32) -> Mat4x4 {
        Mat4x4(
            (theta.cos(), -theta.sin(), 0.0, 0.0),
            (theta.sin(), theta.cos(), 0.0, 0.0),
            (0.0, 0.0, 1.0, 0.0),
            (0.0, 0.0, 0.0, 1.0),
        )
    }

    /// Multiply two matrices
    pub fn mult(mat1: &Mat4x4, mat2: &Mat4x4) -> Mat4x4 {
        Mat4x4(
            (
                mat1.0.0 * mat2.0.0 + mat1.0.1 * mat2.1.0 + mat1.0.2 * mat2.2.0 + mat1.0.3 * mat2.3.0,
                mat1.0.0 * mat2.0.1 + mat1.0.1 * mat2.1.1 + mat1.0.2 * mat2.2.1 + mat1.0.3 * mat2.3.1,
                mat1.0.0 * mat2.0.2 + mat1.0.1 * mat2.1.2 + mat1.0.2 * mat2.2.2 + mat1.0.3 * mat2.3.2,
                mat1.0.0 * mat2.0.3 + mat1.0.1 * mat2.1.3 + mat1.0.2 * mat2.2.3 + mat1.0.3 * mat2.3.3,
            ),
            (
                mat1.1.0 * mat2.0.0 + mat1.1.1 * mat2.1.0 + mat1.1.2 * mat2.2.0 + mat1.1.3 * mat2.3.0,
                mat1.1.0 * mat2.0.1 + mat1.1.1 * mat2.1.1 + mat1.1.2 * mat2.2.1 + mat1.1.3 * mat2.3.1,
                mat1.1.0 * mat2.0.2 + mat1.1.1 * mat2.1.2 + mat1.1.2 * mat2.2.2 + mat1.1.3 * mat2.3.2,
                mat1.1.0 * mat2.0.3 + mat1.1.1 * mat2.1.3 + mat1.1.2 * mat2.2.3 + mat1.1.3 * mat2.3.3,
            ),
            (
                mat1.2.0 * mat2.0.0 + mat1.2.1 * mat2.1.0 + mat1.2.2 * mat2.2.0 + mat1.2.3 * mat2.3.0,
                mat1.2.0 * mat2.0.1 + mat1.2.1 * mat2.1.1 + mat1.2.2 * mat2.2.1 + mat1.2.3 * mat2.3.1,
                mat1.2.0 * mat2.0.2 + mat1.2.1 * mat2.1.2 + mat1.2.2 * mat2.2.2 + mat1.2.3 * mat2.3.2,
                mat1.2.0 * mat2.0.3 + mat1.2.1 * mat2.1.3 + mat1.2.2 * mat2.2.3 + mat1.2.3 * mat2.3.3,
            ),
            (
                mat1.3.0 * mat2.0.0 + mat1.3.1 * mat2.1.0 + mat1.3.2 * mat2.2.0 + mat1.3.3 * mat2.3.0,
                mat1.3.0 * mat2.0.1 + mat1.3.1 * mat2.1.1 + mat1.3.2 * mat2.2.1 + mat1.3.3 * mat2.3.1,
                mat1.3.0 * mat2.0.2 + mat1.3.1 * mat2.1.2 + mat1.3.2 * mat2.2.2 + mat1.3.3 * mat2.3.2,
                mat1.3.0 * mat2.0.3 + mat1.3.1 * mat2.1.3 + mat1.3.2 * mat2.2.3 + mat1.3.3 * mat2.3.3,
            ),
        )
    }
}

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
}

pub struct Mesh {
    pub tris: Vec<Triangle>,
}

impl Mesh {
    /// Construct a new mesh
    pub fn new() -> Mesh {
        Mesh { tris: Vec::new() }
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

    /// Link together a square face of vertices
    fn link_sq(mut a: Vec3d, mut b: Vec3d, mut c: Vec3d, mut d: Vec3d) -> Vec<Triangle> {
        vec![
            Triangle::new(a.copy(), b.copy(), c.copy()),
            Triangle::new(a.copy(), c.copy(), d.copy()),
        ]
    }

    /// Create a square with centre at `c` and sides of length `l`
    pub fn square(c: Vec3d, l: f32) -> Vec<Triangle> {
        let hl = l / 2.0;
        Self::link_sq(
            Vec3d::new(c.x - hl, c.y - hl, c.z),
            Vec3d::new(c.x - hl, c.y + hl, c.z),
            Vec3d::new(c.x + hl, c.y + hl, c.z),
            Vec3d::new(c.x + hl, c.y - hl, c.z),
        )
    }

    /// Create a triangle with centre at `c` and given dimensions
    pub fn triangle(c: Vec3d, w: f32, h: f32) -> Triangle {
        let hw = w / 2.0;
        let hh = h / 2.0;
        Triangle::new(
            Vec3d::new(c.x - hw, c.y - hh, c.z),
            Vec3d::new(c.x, c.y + hh, c.z),
            Vec3d::new(c.x + hw, c.y - hh, c.z),
        )
    }

    /// Create a circle with centre at `c` with radius `r`
    pub fn circle(c: Vec3d, r: f32, dtheta: f32) -> Vec<Triangle> {
        let mut tris: Vec<Triangle> = Vec::new();
        let mut theta: f32 = 0.0;
        let mut done = false;
        while !done {
            let mut next = theta + dtheta;
            if next > 2.0 * std::f32::consts::PI {
                done = true;
                next = 2.0 * std::f32::consts::PI;
            }

            tris.push(Triangle::new(
                Vec3d::new(c.x, c.y, c.z),
                Vec3d::new(c.x - r * theta.cos(), c.y + r * theta.sin(), c.z),
                Vec3d::new(c.x - r * next.cos(), c.y + r * next.sin(), c.z),
            ));
            theta = next;
        }
        tris
    }

    /// Create a cube with centre at `c` and sides of length `l`
    pub fn cube(c: Vec3d, l: f32) -> Vec<Triangle> {
        Self::cuboid(c, Vec3d::diag(l))
    }

    /// Create a cuboid with centre at `c` and lengths `l`
    pub fn cuboid(c: Vec3d, mut l: Vec3d) -> Vec<Triangle> {
        let hl = l.mulk(0.5);
        let mut vertices = (
            Vec3d::new(c.x - hl.x, c.y - hl.y, c.z - hl.z),
            Vec3d::new(c.x - hl.x, c.y + hl.y, c.z - hl.z),
            Vec3d::new(c.x + hl.x, c.y + hl.y, c.z - hl.z),
            Vec3d::new(c.x + hl.x, c.y - hl.y, c.z - hl.z),
            Vec3d::new(c.x - hl.x, c.y - hl.y, c.z + hl.z),
            Vec3d::new(c.x - hl.x, c.y + hl.y, c.z + hl.z),
            Vec3d::new(c.x + hl.x, c.y + hl.y, c.z + hl.z),
            Vec3d::new(c.x + hl.x, c.y - hl.y, c.z + hl.z),
        );
        let faces = [
            ( // Front
                vertices.0.copy(),
                vertices.1.copy(),
                vertices.2.copy(),
                vertices.3.copy(),
            ),
            ( // Right
                vertices.3.copy(),
                vertices.2.copy(),
                vertices.6.copy(),
                vertices.7.copy(),
            ),
            ( // Back
                vertices.7.copy(),
                vertices.6.copy(),
                vertices.5.copy(),
                vertices.4.copy(),
            ),
            ( // Left
                vertices.4.copy(),
                vertices.5.copy(),
                vertices.1.copy(),
                vertices.0.copy(),
            ),
            ( // Top
                vertices.1.copy(),
                vertices.5.copy(),
                vertices.6.copy(),
                vertices.2.copy(),
            ),
            ( // Bottom
                vertices.4.copy(),
                vertices.0.copy(),
                vertices.3.copy(),
                vertices.7.copy(),
            ),
        ];
        let mut tris: Vec<Triangle> = Vec::new();
        for (a, b, c, d) in faces {
            let mut t = Self::link_sq(a, b, c, d);
            tris.append(&mut t);
        }
        tris
    }

    /// Create a triangular prism with centre at `c` and given dimensions
    pub fn prism(c: Vec3d, mut dim: Vec3d) -> Vec<Triangle> {
        let d = dim.mulk(0.5);
        let mut vertices = (
            Vec3d::new(c.x - d.x, c.y - d.y, c.z - d.z),
            Vec3d::new(c.x, c.y + d.y, c.z - d.z),
            Vec3d::new(c.x + d.x, c.y - d.y, c.z - d.z),
            Vec3d::new(c.x - d.x, c.y - d.y, c.z + d.z),
            Vec3d::new(c.x, c.y + d.y, c.z + d.z),
            Vec3d::new(c.x + d.x, c.y - d.y, c.z + d.z),
        );
        let mut tris: Vec<Triangle> = Vec::new();
        // Front
        tris.push(Triangle::new(
            vertices.0.copy(),
            vertices.1.copy(),
            vertices.2.copy(),
        ));
        // Right
        tris.append(&mut Self::link_sq(
            vertices.2.copy(),
            vertices.1.copy(),
            vertices.4.copy(),
            vertices.5.copy(),
        ));
        // Back
        tris.push(Triangle::new(
            vertices.5.copy(),
            vertices.4.copy(),
            vertices.3.copy(),
        ));
        // Left
        tris.append(&mut Self::link_sq(
            vertices.3.copy(),
            vertices.4.copy(),
            vertices.1.copy(),
            vertices.0.copy(),
        ));
        // Bottom
        tris.append(&mut Self::link_sq(
            vertices.3.copy(),
            vertices.0.copy(),
            vertices.2.copy(),
            vertices.5.copy(),
        ));
        tris
    }

    /// Create a cyclinder with centre at `c` with radius `r` and depth `d`
    /// NOT TESTED
    pub fn cylinder(c: Vec3d, r: f32, d: f32, dtheta: f32) -> Vec<Triangle> {
        let d2 = d / 2.0;
        let mut tris: Vec<Triangle> = Vec::new();
        let mut theta: f32 = 0.0;
        let mut done = false;
        while !done {
            let mut next = theta + dtheta;
            if next > 2.0 * std::f32::consts::PI {
                done = true;
                next = 2.0 * std::f32::consts::PI;
            }

            // Front
            tris.push(Triangle::new(
                Vec3d::new(c.x, c.y, c.z - d2),
                Vec3d::new(c.x - r * theta.cos(), c.y + r * theta.sin(), c.z - d2),
                Vec3d::new(c.x - r * next.cos(), c.y + r * next.sin(), c.z - d2),
            ));
            // Side
            tris.append(&mut Self::link_sq(
                Vec3d::new(c.x - r * theta.cos(), c.y + r * theta.sin(), c.z - d2),
                Vec3d::new(c.x - r * next.cos(), c.y + r * next.sin(), c.z - d2),
                Vec3d::new(c.x - r * next.cos(), c.y + r * next.sin(), c.z + d2),
                Vec3d::new(c.x - r * theta.cos(), c.y + r * theta.sin(), c.z + d2),
            ));
            // Back
            tris.push(Triangle::new(
                Vec3d::new(c.x, c.y, c.z + d2),
                Vec3d::new(c.x - r * theta.cos(), c.y + r * theta.sin(), c.z + d2),
                Vec3d::new(c.x - r * next.cos(), c.y + r * next.sin(), c.z + d2),
            ));
            theta = next;
        }
        tris
    }

}