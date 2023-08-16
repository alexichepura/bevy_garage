use bevy::prelude::*;

#[derive(Resource)]
pub struct CarRes {
    pub car_scene: Option<Handle<Scene>>,
    pub wheel_scene: Option<Handle<Scene>>,
    pub show_rays: bool,
}

impl Default for CarRes {
    fn default() -> Self {
        Self {
            car_scene: None,
            wheel_scene: None,
            show_rays: false,
        }
    }
}
