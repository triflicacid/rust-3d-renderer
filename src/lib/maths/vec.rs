use crate::lib::maths::mat::Mat4x4;

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
    pub fn unit_x() -> Vec3d {
        Vec3d::new(1.0, 0.0, 0.0)
    }

    /// Return unit vector in `y` direction
    pub fn unit_y() -> Vec3d {
        Vec3d::new(0.0, 1.0, 0.0)
    }

    /// Return unit vector in `z` direction
    pub fn unit_z() -> Vec3d {
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

    /// Add to x component
    pub fn add_x(&mut self, x: f32) -> Vec3d {
        Vec3d::new(
            self.x + x,
            self.y,
            self.z,
        )
    }

    /// Add to y component
    pub fn add_y(&mut self, y: f32) -> Vec3d {
        Vec3d::new(
            self.x,
            self.y + y,
            self.z,
        )
    }

    /// Add to x component
    pub fn add_z(&mut self, z: f32) -> Vec3d {
        Vec3d::new(
            self.x,
            self.y,
            self.z + z,
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