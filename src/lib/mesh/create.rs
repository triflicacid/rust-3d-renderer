use crate::lib::triangle::Triangle;
use crate::lib::maths::vec::Vec3d;

/// Create a square with centre at `c` and sides of length `l`
pub fn square(c: Vec3d, l: f32) -> Vec<Triangle> {
    let hl = l / 2.0;
    Triangle::link_sq(
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
    cuboid(c, Vec3d::diag(l))
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
        let mut t = Triangle::link_sq(a, b, c, d);
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
    tris.append(&mut Triangle::link_sq(
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
    tris.append(&mut Triangle::link_sq(
        vertices.3.copy(),
        vertices.4.copy(),
        vertices.1.copy(),
        vertices.0.copy(),
    ));
    // Bottom
    tris.append(&mut Triangle::link_sq(
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
        tris.append(&mut Triangle::link_sq(
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
