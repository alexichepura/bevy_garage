use bevy::prelude::*;

#[derive(Resource)]
pub struct CarConfig {
    pub max_torque: f32,
    pub max_toi: f32,
    pub car_scene: Option<Handle<Scene>>,
    pub wheel_scene: Option<Handle<Scene>>,
}
impl Default for CarConfig {
    fn default() -> Self {
        Self {
            max_torque: 1500.,
            max_toi: 100.,
            car_scene: None,
            wheel_scene: None,
        }
    }
}
