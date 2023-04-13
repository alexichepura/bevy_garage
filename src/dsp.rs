use crate::car::Car;
use bevy_rapier3d::prelude::Velocity;
use {bevy::prelude::*, bevy_fundsp::prelude::*, uuid::Uuid};

pub struct EngineSoundPlugin;
struct PianoDsp<F>(F);

impl<T: AudioUnit32 + 'static, F: Send + Sync + 'static + Fn() -> T> DspGraph for PianoDsp<F> {
    fn id(&self) -> Uuid {
        Uuid::from_u128(0xa1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8u128)
    }

    fn generate_graph(&self) -> Box<dyn AudioUnit32> {
        Box::new((self.0)())
    }
}

#[derive(Debug, Resource)]
struct PianoId(Uuid);

#[derive(Resource)]
struct EngineSound(Shared<f32>);

impl EngineSound {
    fn set_freq(&self, freq: Freq) {
        self.0.set_value(freq.into());
    }
}
#[derive(Debug, Clone, Copy)]
struct Freq {
    freq: f32,
}
impl Freq {
    fn to_f32(self) -> f32 {
        self.freq
    }

    fn from_freq(freq: f32) -> Freq {
        Freq { freq }
    }
}
impl From<Freq> for f32 {
    fn from(freq: Freq) -> Self {
        freq.to_f32()
    }
}

impl Plugin for EngineSoundPlugin {
    fn build(&self, app: &mut App) {
        let pitch = shared((100.).into());
        let pitch2 = pitch.clone();

        let piano = move || var(&pitch2) >> square() >> split::<U2>() * 0.2;
        let piano_dsp = PianoDsp(piano.clone());
        let piano_id = piano_dsp.id();

        app.add_dsp_source(piano_dsp, SourceType::Dynamic)
            .insert_resource(EngineSound(pitch))
            .insert_resource(PianoId(piano_id))
            .add_system(engine_sound)
            .add_startup_system(play_piano.in_base_set(StartupSet::PostStartup));
    }
}

const VELOCITY_FREQ_K: f32 = 30.;
fn engine_sound(mut car_query: Query<&Velocity, With<Car>>, pitch_var: Res<EngineSound>) {
    for velocity in car_query.iter_mut() {
        let vel = velocity.linvel.length();
        let freq: f32 = if vel < 0.1 {
            VELOCITY_FREQ_K
        } else {
            VELOCITY_FREQ_K + vel * 2.
        };
        pitch_var.set_freq(Freq::from_freq(freq));
    }
}

fn play_piano(
    mut assets: ResMut<Assets<DspSource>>,
    dsp_manager: Res<DspManager>,
    mut audio: ResMut<Audio<DspSource>>,
    piano_id: Res<PianoId>,
) {
    let source = dsp_manager
        .get_graph_by_id(&piano_id.0)
        .unwrap_or_else(|| panic!("DSP source not found!"));
    audio.play_dsp(assets.as_mut(), source);
}
