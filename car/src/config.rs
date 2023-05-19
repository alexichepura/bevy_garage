use bevy::prelude::*;

#[derive(Resource)]
pub struct CarRes {
    pub max_torque: f32,
    pub car_scene: Option<Handle<Scene>>,
    pub wheel_scene: Option<Handle<Scene>>,
    pub show_rays: bool,
}
impl Default for CarRes {
    fn default() -> Self {
        Self {
            max_torque: 1500.,
            car_scene: None,
            wheel_scene: None,
            show_rays: false,
        }
    }
}
