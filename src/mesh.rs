use bevy::{
    prelude::Mesh,
    render::{
        mesh::{Indices, VertexAttributeValues},
        render_resource::PrimitiveTopology,
    },
};
use bevy_rapier3d::prelude::Real;
use nalgebra::{Point3, Vector3};

pub fn bevy_mesh(buffers: (Vec<Point3<Real>>, Vec<[u32; 3]>)) -> Mesh {
    let (vtx, idx) = buffers;
    let mut normals: Vec<[f32; 3]> = vec![];
    let mut vertices: Vec<[f32; 3]> = vec![];

    for idx in idx {
        let a = vtx[idx[0] as usize];
        let b = vtx[idx[1] as usize];
        let c = vtx[idx[2] as usize];

        vertices.push(a.cast::<f32>().into());
        vertices.push(b.cast::<f32>().into());
        vertices.push(c.cast::<f32>().into());
    }

    for vtx in vertices.chunks(3) {
        let a = Point3::from(vtx[0]);
        let b = Point3::from(vtx[1]);
        let c = Point3::from(vtx[2]);
        let n = (b - a).cross(&(c - a)).normalize();
        normals.push(n.cast::<f32>().into());
        normals.push(n.cast::<f32>().into());
        normals.push(n.cast::<f32>().into());
    }

    normals
        .iter_mut()
        .for_each(|n| *n = Vector3::from(*n).normalize().into());
    let indices: Vec<_> = (0..vertices.len() as u32).collect();
    let uvs: Vec<_> = (0..vertices.len()).map(|_| [0.0, 0.0]).collect();

    // Generate the mesh
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_attribute(
        Mesh::ATTRIBUTE_POSITION,
        VertexAttributeValues::from(vertices),
    );
    mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, VertexAttributeValues::from(normals));
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, VertexAttributeValues::from(uvs));
    mesh.set_indices(Some(Indices::U32(indices)));
    mesh
}
