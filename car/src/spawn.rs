#[derive(Debug)]
pub struct SpawnCarEvent {
    pub is_hid: bool,
    pub index: usize,
    pub init_meters: Option<f32>,
}
