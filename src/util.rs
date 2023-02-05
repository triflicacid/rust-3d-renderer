// 3 dimensional vector
pub struct Vec3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3d {
    /// Construct a new 3D vector
    pub fn new(x: f64, y: f64, z: f64) -> Vec3d {
        Vec3d { x, y, z }
    }

    /// Add a scalar to self
    pub fn addk(&mut self, k: f64) -> &mut Self {
        self.x += k;
        self.y += k;
        self.z += k;
        self
    }

    /// Add a vector to self
    pub fn add(&mut self, vec: &Vec3d) -> &mut Self {
        self.x += vec.x;
        self.y += vec.y;
        self.z += vec.z;
        self
    }

    // Multiply self by a scalar
    pub fn mulk(&mut self, k: f64) -> &mut Self {
        self.x *= k;
        self.y *= k;
        self.z *= k;
        self
    }

    /// Multiple a vector with a matrix. Divide vector components by `w` component.
    pub fn mult_mat(vec: &Vec3d, mat: &Mat4x4) -> Vec3d {
        let mut w: f64 = vec.x * mat.0.3 + vec.y * mat.1.3 + vec.z * mat.2.3 + mat.3.3;
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
}

pub struct Mat4x4(pub (f64,f64,f64,f64), pub (f64,f64,f64,f64), pub (f64,f64,f64,f64), pub (f64,f64,f64,f64));

impl Mat4x4 {
    /// Construct a 3D rotation matrix around the X axis. Theta in radians
    pub fn rot_x(theta: f64) -> Mat4x4 {
        Mat4x4(
            (1.0, 0.0, 0.0, 0.0),
            (0.0, theta.cos(), -theta.sin(), 0.0),
            (0.0, theta.sin(), theta.cos(), 0.0),
            (0.0, 0.0, 0.0, 1.0),
        )
    }

    /// Construct a 3D rotation matrix around the Y axis. Theta in radians
    pub fn rot_y(theta: f64) -> Mat4x4 {
        Mat4x4(
            (theta.cos(), 0.0, theta.sin(), 0.0),
            (0.0, 1.0, 0.0, 0.0),
            (-theta.sin(), 0.0, theta.cos(), 0.0),
            (0.0, 0.0, 0.0, 1.0),
        )
    }

    /// Construct a 3D rotation matrix around the Z axis. Theta in radians
    pub fn rot_z(theta: f64) -> Mat4x4 {
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

/// Add two vectors
pub fn add_vec(v1: &Vec3d, v2: &Vec3d) -> Vec3d {
    Vec3d {
        x: v1.x + v2.x,
        y: v1.y + v2.y,
        z: v1.z + v2.z,
    }
}

pub struct Triangle {
    pub vertices: (Vec3d, Vec3d, Vec3d),
}

impl Triangle {
    /// Construct a new triangle
    pub fn new(a: Vec3d, b: Vec3d, c: Vec3d) -> Triangle {
        Triangle {
            vertices: (a, b, c),
        }
    }
}

pub struct Mesh {
    pub tris: Vec<Triangle>,
}

impl Mesh {
    // Construct a new mesh
    pub fn new() -> Mesh {
        Mesh { tris: Vec::new() }
    }

    // Creates a unit cube at (0,0,0)
    pub fn new_unit_cube() -> Mesh {
        Mesh {
            tris: vec![
                // S
                Triangle::new(
                    Vec3d::new(0.0, 0.0, 0.0),
                    Vec3d::new(0.0, 1.0, 0.0),
                    Vec3d::new(1.0, 1.0, 0.0),
                ),
                Triangle::new(
                    Vec3d::new(0.0, 0.0, 0.0),
                    Vec3d::new(1.0, 1.0, 0.0),
                    Vec3d::new(1.0, 1.0, 1.0),
                ),
                // E
                Triangle::new(
                    Vec3d::new(1.0, 0.0, 0.0),
                    Vec3d::new(1.0, 1.0, 0.0),
                    Vec3d::new(1.0, 1.0, 1.0),
                ),
                Triangle::new(
                    Vec3d::new(1.0, 0.0, 0.0),
                    Vec3d::new(1.0, 1.0, 1.0),
                    Vec3d::new(1.0, 0.0, 1.0),
                ),
                // N
                Triangle::new(
                    Vec3d::new(1.0, 0.0, 1.0),
                    Vec3d::new(1.0, 1.0, 1.0),
                    Vec3d::new(0.0, 1.0, 1.0),
                ),
                Triangle::new(
                    Vec3d::new(1.0, 0.0, 1.0),
                    Vec3d::new(0.0, 1.0, 1.0),
                    Vec3d::new(0.0, 0.0, 1.0),
                ),
                // W
                Triangle::new(
                    Vec3d::new(0.0, 0.0, 1.0),
                    Vec3d::new(0.0, 1.0, 1.0),
                    Vec3d::new(0.0, 1.0, 0.0),
                ),
                Triangle::new(
                    Vec3d::new(0.0, 0.0, 1.0),
                    Vec3d::new(0.0, 1.0, 0.0),
                    Vec3d::new(0.0, 0.0, 0.0),
                ),
                // T
                Triangle::new(
                    Vec3d::new(0.0, 1.0, 0.0),
                    Vec3d::new(0.0, 1.0, 1.0),
                    Vec3d::new(1.0, 1.0, 1.0),
                ),
                Triangle::new(
                    Vec3d::new(0.0, 1.0, 0.0),
                    Vec3d::new(1.0, 1.0, 1.0),
                    Vec3d::new(1.0, 1.0, 0.0),
                ),
                // B
                Triangle::new(
                    Vec3d::new(1.0, 0.0, 1.0),
                    Vec3d::new(0.0, 0.0, 1.0),
                    Vec3d::new(0.0, 0.0, 0.0),
                ),
                Triangle::new(
                    Vec3d::new(1.0, 0.0, 1.0),
                    Vec3d::new(0.0, 0.0, 0.0),
                    Vec3d::new(1.0, 0.0, 0.0),
                ),
            ],
        }
    }
}