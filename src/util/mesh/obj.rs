use std::fs::File;
use std::io::{self, BufRead};

use super::Mesh;
use super::super::maths::vec::Vec3d;

pub fn parse_file(filename: String) -> Result<Mesh, String> {
    let file_res = File::open(filename.to_string());
    let mut file: File;
    if file_res.is_err() {
        return Err(file_res.unwrap_err().to_string());
    } else {
        file = file_res.unwrap();
    }

    let lines = io::BufReader::new(file).lines();
    let mut mesh = Mesh::new();
    let mut n: u32 = 1;

    for res in lines {
        let line = res.unwrap();
        if line.starts_with("#") { // Comment

        } else if line.starts_with("v") { // Vertex
            let mut iter = line.split_whitespace();
            iter.next(); // Skip "v"
            let a = iter.next().unwrap().parse::<f32>().unwrap();
            let b = iter.next().unwrap().parse::<f32>().unwrap();
            let c = iter.next().unwrap().parse::<f32>().unwrap();
            mesh.add_vertex_force(&mut Vec3d::new(a, b, c));
        } else if line.starts_with("f") { // Face
            let mut iter = line.split_whitespace();
            iter.next(); // Skip "f"
            let a = iter.next().unwrap().parse::<usize>().unwrap();
            let b = iter.next().unwrap().parse::<usize>().unwrap();
            let c = iter.next().unwrap().parse::<usize>().unwrap();
            mesh.add_tri(a - 1, b - 1, c - 1, &None);
        } else {
            // let mut str: String = "Error (line ".to_owned();
            // str.push_str(&n.to_string());
            // str.push_str("): unknown character ");
            // str.push(line.chars().nth(0).unwrap());
            // return Err(str);
        }

        n += 1;
    }

    Ok(mesh)
}