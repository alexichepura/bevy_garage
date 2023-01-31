use bevy::{
    prelude::{Mesh, Vec2},
    render::{
        mesh::{Indices, VertexAttributeValues},
        render_resource::PrimitiveTopology,
    },
};
use bevy_rapier3d::{na::Point3, prelude::Real};

pub fn _bevy_mesh(buffers: (Vec<Point3<Real>>, Vec<[u32; 3]>)) -> Mesh {
    let (vtx, idx) = buffers;
    let mut vertices: Vec<[f32; 3]> = vec![];

    for idx in idx {
        let a = vtx[idx[0] as usize];
        let b = vtx[idx[1] as usize];
        let c = vtx[idx[2] as usize];

        vertices.push(a.cast::<f32>().into());
        vertices.push(b.cast::<f32>().into());
        vertices.push(c.cast::<f32>().into());
    }

    let indices: Vec<_> = (0..vertices.len() as u32).collect();
    let uvs: Vec<_> = (0..vertices.len()).map(|_| [0.0, 0.0]).collect();

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        VertexAttributeValues::from(vertices),
    );
    mesh.compute_flat_normals();
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, VertexAttributeValues::from(uvs));
    mesh.set_indices(Some(Indices::U32(indices)));
    mesh
}

#[derive(Debug, Copy, Clone)]
pub struct QuadPlane {
    pub size: Vec2,
}

impl Default for QuadPlane {
    fn default() -> Self {
        QuadPlane { size: Vec2::ONE }
    }
}

impl QuadPlane {
    pub fn new(size: Vec2) -> Self {
        Self { size }
    }
}

impl From<QuadPlane> for Mesh {
    fn from(quad: QuadPlane) -> Self {
        let extent_x = quad.size.x / 2.0;
        let extent_z = quad.size.y / 2.0;

        // let vertices = [
        //     ([-extent_x, -extent_y, 0.0], [0.0, 0.0, 1.0], [0.0, 1.0]),
        //     ([-extent_x, extent_y, 0.0], [0.0, 0.0, 1.0], [0.0, 0.0]),
        //     ([extent_x, extent_y, 0.0], [0.0, 0.0, 1.0], [ 1.0, 0.0]),
        //     ([extent_x, -extent_y, 0.0], [0.0, 0.0, 1.0], [ 1.0, 1.0]),
        // ];

        let vertices = [
            ([extent_x, 0.0, -extent_z], [0.0, 1.0, 0.0], [1.0, 1.0]),
            ([extent_x, 0.0, extent_z], [0.0, 1.0, 0.0], [1.0, 0.0]),
            ([-extent_x, 0.0, extent_z], [0.0, 1.0, 0.0], [0.0, 0.0]),
            ([-extent_x, 0.0, -extent_z], [0.0, 1.0, 0.0], [0.0, 1.0]),
        ];

        let indices = Indices::U32(vec![0, 2, 1, 0, 3, 2]);

        let positions: Vec<_> = vertices.iter().map(|(p, _, _)| *p).collect();
        let normals: Vec<_> = vertices.iter().map(|(_, n, _)| *n).collect();
        let uvs: Vec<_> = vertices.iter().map(|(_, _, uv)| *uv).collect();

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.set_indices(Some(indices));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh
    }
}
