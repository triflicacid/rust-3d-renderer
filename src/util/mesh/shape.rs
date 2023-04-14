use crate::util::maths::vec::Vec3d;
use crate::util::mesh::Mesh;

/// Create a square with sides of length `l`
pub fn square(l: f32) -> Mesh {
    let mut mesh = Mesh::new();
    let hl = l / 2.0;
    let v = (
        mesh.add_vertex_force(&Vec3d::new(-hl, -hl, 0.0)),
        mesh.add_vertex_force(&Vec3d::new(-hl, hl, 0.0)),
        mesh.add_vertex_force(&Vec3d::new(hl, hl, 0.0)),
        mesh.add_vertex_force(&Vec3d::new(hl, -hl, 0.0)),
    );
    
    mesh.add_quad(v.0, v.1, v.2, v.3, &None);
    mesh
}

/// Create a triangle with the given dimensions
pub fn triangle(w: f32, h: f32) -> Mesh {
    let mut mesh = Mesh::new();
    let hw = w / 2.0;
    let hh = h / 2.0;
    let v = (
        mesh.add_vertex_force(&Vec3d::new(-hw, -hh, 0.0)),
        mesh.add_vertex_force(&Vec3d::new(0.0, hh, 0.0)),
        mesh.add_vertex_force(&Vec3d::new(hw, -hh, 0.0)),
    );

    mesh.add_tri(v.0, v.1, v.2, &None);
    mesh
}

/// Create a circle with radius `r`
pub fn circle(r: f32, dtheta: f32) -> Mesh {
    let mut mesh = Mesh::new();
    let mut theta: f32 = 0.0;
    let mut done = false;
    let two_pi = 2.0 * std::f32::consts::PI;

    let o = mesh.add_vertex_force(&Vec3d::origin());

    while !done {
        let mut next = theta + dtheta;

        if next > two_pi {
            done = true;
            next = two_pi;
        }

        let a = mesh.add_vertex(&Vec3d::new(-r * theta.cos(), r * theta.sin(), 0.0));
        let b = mesh.add_vertex(&Vec3d::new(-r * next.cos(), r * next.sin(), 0.0));

        mesh.add_tri(o, a, b, &None);
        theta = next;
    }
    
    mesh
}

/// Create a tetrahedron with dimensions `dim`
pub fn tetrahedron(dim: &Vec3d) -> Mesh {
    let mut mesh = Mesh::new();
    let d = dim.mulk(0.5);
    let v = (
        mesh.add_vertex_force(&Vec3d::new(-d.x, -d.y, -d.z)),
        mesh.add_vertex_force(&Vec3d::new(d.x, -d.y, d.z)),
        mesh.add_vertex_force(&Vec3d::new(d.x, -d.y, -d.z)),
        mesh.add_vertex_force(&Vec3d::new(0.0, d.y, 0.0)),
    );

    mesh.add_tri(v.0, v.1, v.2, &None) // Botom
        .add_tri(v.0, v.3, v.2, &None) // Front
        .add_tri(v.2, v.3, v.1, &None) // Right
        .add_tri(v.1, v.3, v.0, &None); // Back-Left

    mesh
}

/// Create a cube with sides of length `l`
pub fn cube(l: f32) -> Mesh {
    cuboid(&Vec3d::diag(l))
}

/// Create a cuboid with dimensions `dim`
pub fn cuboid(dim: &Vec3d) -> Mesh {
    let mut mesh = Mesh::new();
    let hl = dim.mulk(0.5);
    let v = (
        mesh.add_vertex(&Vec3d::new(-hl.x, -hl.y, -hl.z)),
        mesh.add_vertex(&Vec3d::new(-hl.x, hl.y, -hl.z)),
        mesh.add_vertex(&Vec3d::new(hl.x, hl.y, -hl.z)),
        mesh.add_vertex(&Vec3d::new(hl.x, -hl.y, -hl.z)),
        mesh.add_vertex(&Vec3d::new(-hl.x, -hl.y, hl.z)),
        mesh.add_vertex(&Vec3d::new(-hl.x, hl.y, hl.z)),
        mesh.add_vertex(&Vec3d::new(hl.x, hl.y, hl.z)),
        mesh.add_vertex(&Vec3d::new(hl.x, -hl.y, hl.z)),
    );

    mesh.add_quad(v.0, v.1, v.2, v.3, &None) // Front
        .add_quad(v.3, v.2, v.6, v.7, &None) // Right
        .add_quad(v.7, v.6, v.5, v.4, &None) // Back
        .add_quad(v.4, v.5, v.1, v.0, &None) // Left
        .add_quad(v.1, v.5, v.6, v.2, &None) // Top
        .add_quad(v.4, v.0, v.3, v.7, &None); // Bottom

    mesh
}

/// Create a triangular prism dimensions `dim`
pub fn prism(dim: &Vec3d) -> Mesh {
    let mut mesh = Mesh::new();
    let d = dim.mulk(0.5);
    let v = (
        mesh.add_vertex_force(&Vec3d::new(-d.x, -d.y, -d.z)),
        mesh.add_vertex_force(&Vec3d::new(0.0, d.y, -d.z)),
        mesh.add_vertex_force(&Vec3d::new(d.x, -d.y, -d.z)),
        mesh.add_vertex_force(&Vec3d::new(-d.x, -d.y, d.z)),
        mesh.add_vertex_force(&Vec3d::new(0.0, d.y, d.z)),
        mesh.add_vertex_force(&Vec3d::new(d.x, -d.y, d.z)),
    );

    mesh.add_tri(v.0, v.1, v.2, &None) // Front
        .add_quad(v.2, v.1, v.4, v.5, &None) // Right
        .add_tri(v.5, v.4, v.3, &None) // Back
        .add_quad(v.3, v.4, v.1, v.0, &None) // Left
        .add_quad(v.3, v.0, v.2, v.5, &None); // Bottom
    
    mesh
}

// /// Create a cyclinder with centre at `c` with radius `r` and depth `d`
// /// NOT TESTED
// pub fn cylinder(c: Vec3d, r: f32, d: f32, dtheta: f32) -> Vec<Triangle> {
//     let d2 = d / 2.0;
//     let mut tris: Vec<Triangle> = Vec::new();
//     let mut theta: f32 = 0.0;
//     let mut done = false;
//     while !done {
//         let mut next = theta + dtheta;
//         if next > 2.0 * std::f32::consts::PI {
//             done = true;
//             next = 2.0 * std::f32::consts::PI;
//         }

//         // Front
//         tris.push(Triangle::new(
//             Vec3d::new(c.x, c.y, c.z - d2),
//             Vec3d::new(c.x - r * theta.cos(), c.y + r * theta.sin(), c.z - d2),
//             Vec3d::new(c.x - r * next.cos(), c.y + r * next.sin(), c.z - d2),
//         ));
//         // Side
//         tris.append(&mut Triangle::link_sq(
//             Vec3d::new(c.x - r * theta.cos(), c.y + r * theta.sin(), c.z - d2),
//             Vec3d::new(c.x - r * next.cos(), c.y + r * next.sin(), c.z - d2),
//             Vec3d::new(c.x - r * next.cos(), c.y + r * next.sin(), c.z + d2),
//             Vec3d::new(c.x - r * theta.cos(), c.y + r * theta.sin(), c.z + d2),
//         ));
//         // Back
//         tris.push(Triangle::new(
//             Vec3d::new(c.x, c.y, c.z + d2),
//             Vec3d::new(c.x - r * theta.cos(), c.y + r * theta.sin(), c.z + d2),
//             Vec3d::new(c.x - r * next.cos(), c.y + r * next.sin(), c.z + d2),
//         ));
//         theta = next;
//     }
//     tris
// }

/// Generate a sphere
pub fn sphere(radius: f32, n_slices: usize, n_stacks: usize) -> Mesh {
    let mut mesh = Mesh::new();

    // Top
    let vtop = mesh.add_vertex_force(&Vec3d::new(0.0, radius, 0.0));

    for i in 0..(n_stacks - 1) {
        let phi: f32 = std::f32::consts::PI * (i as f32 + 1.0) / n_stacks as f32;
        for j in 0..n_slices {
            let theta: f32 = std::f32::consts::PI * 2.0 * j as f32 / n_slices as f32;
            let x = phi.sin() * theta.cos();
            let y = phi.cos();
            let z = phi.sin() * theta.sin();
            mesh.add_vertex_force(&Vec3d::new(x, y, z));
        }
    }

    // Bottom
    let vbot = mesh.add_vertex_force(&Vec3d::new(0.0, -radius, 0.0));

    for i in 0..n_slices {
        let i0 = i + 1;
        let i1 = (i + 1) % n_slices + 1;
        mesh.add_tri(vtop, i1, i0, &None);

        let i0 = i + n_slices * (n_stacks - 2) + 1;
        let i1 = (i + 1) % n_slices + n_slices * (n_stacks - 2) + 1;
        mesh.add_tri(vbot, i0, i1, &None);
    }

    for j in 0..(n_stacks - 2) {
        let j0 = j * n_slices + 1;
        let j1 = (j + 1) * n_slices + 1;
        for i in 0..n_slices {
            mesh.add_quad(
                j0 + i,
                j0 + (i + 1) % n_slices,
                j1 + (i + 1) % n_slices,
                j1 + i,
                &None
            );
        }
    }

    mesh
}