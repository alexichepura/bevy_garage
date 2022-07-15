use std::{fs::File, io::BufReader};

use bevy::{
    prelude::Mesh,
    render::mesh::{Indices, PrimitiveTopology},
};
use bevy_obj::ObjError;

pub fn load_obj_from_bytes(bytes: BufReader<File>) -> Result<Mesh, ObjError> {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    let raw = obj::raw::parse_obj(bytes)?;

    // Get the most complete vertex representation
    //  1 => Position
    //  2 => Position, Normal
    //  3 => Position, Normal, Texture
    let mut pnt = 4;
    for polygon in &raw.polygons {
        use obj::raw::object::Polygon;
        match polygon {
            Polygon::P(_) => pnt = std::cmp::min(pnt, 1),
            Polygon::PT(_) => pnt = std::cmp::min(pnt, 1),
            Polygon::PN(_) => pnt = std::cmp::min(pnt, 2),
            Polygon::PTN(_) => pnt = std::cmp::min(pnt, 3),
        }
    }

    match pnt {
        1 => {
            let obj: obj::Obj<obj::Position, u32> = obj::Obj::new(raw)?;
            set_position_data(&mut mesh, obj.vertices.iter().map(|v| v.position).collect());
            set_normal_data(
                &mut mesh,
                obj.vertices.iter().map(|_| [0., 0., 0.]).collect(),
            );
            set_uv_data(&mut mesh, obj.vertices.iter().map(|_| [0., 0.]).collect());
            set_mesh_indices(&mut mesh, obj);
        }
        2 => {
            let obj: obj::Obj<obj::Vertex, u32> = obj::Obj::new(raw)?;
            set_position_data(&mut mesh, obj.vertices.iter().map(|v| v.position).collect());
            set_normal_data(&mut mesh, obj.vertices.iter().map(|v| v.normal).collect());
            set_uv_data(&mut mesh, obj.vertices.iter().map(|_| [0., 0.]).collect());
            set_mesh_indices(&mut mesh, obj);
        }
        3 => {
            let obj: obj::Obj<obj::TexturedVertex, u32> = obj::Obj::new(raw)?;
            set_position_data(&mut mesh, obj.vertices.iter().map(|v| v.position).collect());
            set_normal_data(&mut mesh, obj.vertices.iter().map(|v| v.normal).collect());
            set_uv_data(
                &mut mesh,
                obj.vertices
                    .iter()
                    // Flip UV for correct values
                    .map(|v| [v.texture[0], 1.0 - v.texture[1]])
                    .collect(),
            );
            set_mesh_indices(&mut mesh, obj);
        }
        _ => return Err(ObjError::UnknownVertexFormat),
    }

    Ok(mesh)
}

fn set_position_data(mesh: &mut Mesh, data: Vec<[f32; 3]>) {
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, data);
}

fn set_normal_data(mesh: &mut Mesh, data: Vec<[f32; 3]>) {
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, data);
}

fn set_uv_data(mesh: &mut Mesh, data: Vec<[f32; 2]>) {
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, data);
}

fn set_mesh_indices<T>(mesh: &mut Mesh, obj: obj::Obj<T, u32>) {
    mesh.set_indices(Some(Indices::U32(
        obj.indices.iter().map(|i| *i as u32).collect(),
    )));
}
