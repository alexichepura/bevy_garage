//! Adapted from parry https://github.com/dimforge/parry/blob/e058023d33976628731cc7421b57c38f53b6b2ec/src/transformation/ear_clipping.rs
//! Ear-clipping algorithm for creating a triangle mesh from a simple polygon.
//! Based on https://github.com/ivanfratric/polypartition, contributed by embotech AG.
use nalgebra::Point2;
use parry3d::math::Real;
type Point<T> = Point2<T>;
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum Orient {
    Ccw,
    Cw,
    None,
}
pub fn corner_direction(p1: &Point<Real>, p2: &Point<Real>, p3: &Point<Real>) -> Orient {
    let v1 = p1 - p2;
    let v2 = p3 - p2;
    let cross: Real = v1.perp(&v2);

    match cross
        .partial_cmp(&0.0)
        .expect("Found NaN while computing corner direction.")
    {
        std::cmp::Ordering::Less => Orient::Ccw,
        std::cmp::Ordering::Equal => Orient::None,
        std::cmp::Ordering::Greater => Orient::Cw,
    }
}
pub fn is_point_in_triangle(
    p: &Point<Real>,
    v1: &Point<Real>,
    v2: &Point<Real>,
    v3: &Point<Real>,
) -> Option<bool> {
    let d1 = corner_direction(p, v1, v2);
    let d2 = corner_direction(p, v2, v3);
    let d3 = corner_direction(p, v3, v1);
    let has_cw = d1 == Orient::Cw || d2 == Orient::Cw || d3 == Orient::Cw;
    let has_ccw = d1 == Orient::Ccw || d2 == Orient::Ccw || d3 == Orient::Ccw;
    if d1 == Orient::None && d2 == Orient::None && d3 == Orient::None {
        None
    } else {
        Some(!(has_cw && has_ccw))
    }
}
#[derive(Clone, Default)]
struct VertexInfo {
    is_active: bool,
    is_ear: bool,
    pointiness: Real,
    p_prev: usize,
    p_next: usize,
}
fn update_vertex(idx: usize, vertex_info: &mut VertexInfo, points: &[Point<Real>]) -> bool {
    let p = points[idx];
    let p1 = points[vertex_info.p_prev];
    let p3 = points[vertex_info.p_next];
    let vec1 = (p1 - p).normalize();
    let vec3 = (p3 - p).normalize();
    vertex_info.pointiness = vec1.dot(&vec3);
    if vertex_info.pointiness.is_nan() {
        return false;
    }
    let mut error = false;
    vertex_info.is_ear = corner_direction(&p1, &p, &p3) == Orient::Ccw
        && (0..points.len())
            .filter(|&i| i != vertex_info.p_prev && i != idx && i != vertex_info.p_next)
            .all(|i| {
                if let Some(is) = is_point_in_triangle(&points[i], &p1, &p, &p3) {
                    !is
                } else {
                    error = true;
                    true
                }
            });
    !error
}
pub(crate) fn triangulate_ear_clipping(vertices: &[Point<Real>]) -> Option<Vec<[u32; 3]>> {
    let n_vertices = vertices.len();
    let mut vertex_info = vec![VertexInfo::default(); n_vertices];
    let success = vertex_info.iter_mut().enumerate().all(|(i, info)| {
        info.is_active = true;
        info.p_prev = if i == 0 { n_vertices - 1 } else { i - 1 };
        info.p_next = if i == n_vertices - 1 { 0 } else { i + 1 };
        update_vertex(i, info, vertices)
    });
    if !success {
        return None;
    }
    let mut output_indices = Vec::new();
    for i in 0..n_vertices - 3 {
        let maybe_ear = vertex_info
            .iter()
            .enumerate()
            .filter(|(_, info)| info.is_active && info.is_ear)
            .max_by(|(_, info1), (_, info2)| {
                info1.pointiness.partial_cmp(&info2.pointiness).unwrap()
            });
        let (ear_i, _) = match maybe_ear {
            Some(ear) => ear,
            None => return None,
        };
        vertex_info[ear_i].is_active = false;
        let VertexInfo { p_prev, p_next, .. } = vertex_info[ear_i];
        let triangle_points = [p_prev as u32, ear_i as u32, p_next as u32];
        output_indices.push(triangle_points);
        vertex_info[p_prev].p_next = vertex_info[ear_i].p_next;
        vertex_info[p_next].p_prev = vertex_info[ear_i].p_prev;
        if i == n_vertices - 4 {
            break;
        };
        if !update_vertex(p_prev, &mut vertex_info[p_prev], vertices)
            || !update_vertex(p_next, &mut vertex_info[p_next], vertices)
        {
            return None;
        }
    }
    if let Some((i, info)) = vertex_info
        .iter()
        .enumerate()
        .find(|(_, info)| info.is_active)
    {
        let triangle_points = [info.p_prev as u32, i as u32, info.p_next as u32];
        output_indices.push(triangle_points);
    }
    Some(output_indices)
}
