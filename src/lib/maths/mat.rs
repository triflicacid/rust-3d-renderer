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