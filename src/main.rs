use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color};
use ggez::{Context, ContextBuilder, GameResult};
use ggez::glam::*;

// 3 dimensional vector
struct Vec3d {
    x: f64,
    y: f64,
    z: f64,
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
    fn mult_mat(vec: &Vec3d, mat: &Mat4x4) -> Vec3d {
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

struct Mat4x4((f64,f64,f64,f64),(f64,f64,f64,f64),(f64,f64,f64,f64),(f64,f64,f64,f64));

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
fn add_vec(v1: &Vec3d, v2: &Vec3d) -> Vec3d {
    Vec3d {
        x: v1.x + v2.x,
        y: v1.y + v2.y,
        z: v1.z + v2.z,
    }
}

struct Triangle {
    vertices: (Vec3d, Vec3d, Vec3d),
}

impl Triangle {
    /// Construct a new triangle
    pub fn new(a: Vec3d, b: Vec3d, c: Vec3d) -> Triangle {
        Triangle {
            vertices: (a, b, c),
        }
    }
}

struct Mesh {
    tris: Vec<Triangle>,
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

const WIN_WIDTH: f32 = 1440.0;
const WIN_HEIGHT: f32 = 960.0;

fn main() {
    // Make a Context.
    let cb = ContextBuilder::new("3D Rendered", "Ruben Saunders")
        .window_setup(WindowSetup::default().title("3D Renderer"))
        .window_mode(WindowMode::default().dimensions(WIN_WIDTH, WIN_HEIGHT));
    let (mut ctx, event_loop) = cb.build().expect("Could not create ggez context!");
    let rd = Renderer::new(&mut ctx);

    event::run(ctx, event_loop, rd);
}

struct Renderer {
    mesh: Mesh,
    proj: Mat4x4,
    pos: Vec3d,
    theta: f64,
}

impl Renderer {
    pub fn new(_ctx: &mut Context) -> Renderer {
        let near = 1f64;
        let far = 1000f64;
        let fov = 90f64;
        let fov_rad = 1f64 / (fov * 0.5f64 / 180f64 * std::f64::consts::PI).tan();
        let ar = (WIN_HEIGHT / WIN_WIDTH) as f64;
        Renderer {
            mesh: Mesh::new_unit_cube(),
            proj: Mat4x4(
                (ar * fov_rad, 0.0, 0.0, 0.0),
                (0.0, fov_rad, 0.0, 0.0),
                (0.0, 0.0, far / (far - near), 1.0),
                (0.0, 0.0, (-far * near) / (far - near), 0.0),
            ),
            pos: Vec3d::new(0.0, 0.0, 3.0),
            theta: 0.0,
        }
    }
}

impl EventHandler for Renderer {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.theta = self.theta + 0.02; // % (2.0 * std::f64::consts::PI);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let rot_x = Mat4x4::rot_x(self.theta * 0.5);
        let rot_z = Mat4x4::rot_z(self.theta);
        let rot = Mat4x4::mult(&rot_z, &rot_x);
        let mb = &mut graphics::MeshBuilder::new();

        // Project triangles
        for tri in &self.mesh.tris {
            // Rotate
            let mut tri_trans = Triangle::new(
                Vec3d::mult_mat(&tri.vertices.0, &rot),
                Vec3d::mult_mat(&tri.vertices.1, &rot),
                Vec3d::mult_mat(&tri.vertices.2, &rot),
            );

            // Translate
            tri_trans.vertices.0.add(&self.pos);
            tri_trans.vertices.1.add(&self.pos);
            tri_trans.vertices.2.add(&self.pos);

            let mut tri_proj = Triangle::new(
                Vec3d::mult_mat(&tri_trans.vertices.0, &self.proj),
                Vec3d::mult_mat(&tri_trans.vertices.1, &self.proj),
                Vec3d::mult_mat(&tri_trans.vertices.2, &self.proj),
            );

            tri_proj.vertices.0.addk(1.0);
            tri_proj.vertices.1.addk(1.0);
            tri_proj.vertices.2.addk(1.0);

            tri_proj.vertices.0.x *= 0.5 * WIN_WIDTH as f64;
            tri_proj.vertices.0.y *= 0.5 * WIN_HEIGHT as f64;
            tri_proj.vertices.1.x *= 0.5 * WIN_WIDTH as f64;
            tri_proj.vertices.1.y *= 0.5 * WIN_HEIGHT as f64;
            tri_proj.vertices.2.x *= 0.5 * WIN_WIDTH as f64;
            tri_proj.vertices.2.y *= 0.5 * WIN_HEIGHT as f64;

            let vs = [
                vec2(tri_proj.vertices.0.x as f32, tri_proj.vertices.0.y as f32),
                vec2(tri_proj.vertices.1.x as f32, tri_proj.vertices.1.y as f32),
                vec2(tri_proj.vertices.2.x as f32, tri_proj.vertices.2.y as f32),
                vec2(tri_proj.vertices.0.x as f32, tri_proj.vertices.0.y as f32),
            ];

            mb.line(
                &vs,
                2.0,
                Color::WHITE,
            )?;
        }

        let mesh = graphics::Mesh::from_data(ctx, mb.build());

        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        canvas.draw(&mesh, graphics::DrawParam::default());
        canvas.finish(ctx)?;

        Ok(())
    }
}

