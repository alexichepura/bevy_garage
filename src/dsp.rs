use bevy_garage_car::Car;
use bevy_rapier3d::prelude::Velocity;
use {bevy::prelude::*, bevy_fundsp::prelude::*, uuid::Uuid};

// https://github.com/harudagondi/bevy_fundsp/blob/main/examples/bevy_audio/pitch.rs

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
struct CarSound {
    pitch: Shared<f32>,
    vol: Shared<f32>,
}

impl CarSound {
    fn set_pitch(&self, pitch: f32) {
        self.pitch.set_value(pitch.into());
    }
    fn set_vol(&self, vol: f32) {
        self.vol.set_value(vol.into());
    }
}

#[derive(Resource, Default)]
pub struct Dsp {
    pub engine_sink: Option<Handle<AudioSink>>,
}

const VELOCITY_PITCH_K: f32 = 30.;
impl Plugin for EngineSoundPlugin {
    fn build(&self, app: &mut App) {
        let pitch = shared(VELOCITY_PITCH_K);
        let pitch2 = pitch.clone();

        let vol = shared(0.5);
        let vol_clone = vol.clone();

        let piano = move || var(&pitch2) >> var(&vol_clone) * square() >> split::<U2>() * 0.2;
        let piano_dsp = PianoDsp(piano.clone());
        let piano_id = piano_dsp.id();

        app.add_dsp_source(piano_dsp, SourceType::Dynamic)
            .insert_resource(CarSound { pitch, vol })
            .insert_resource(PianoId(piano_id))
            .insert_resource(Dsp::default())
            .add_startup_system(engine_sound_start.in_base_set(StartupSet::PostStartup))
            .add_systems((
                engine_sound,
                engine_sound_vol,
                // engine_sound_switch
            ));
    }
}

fn engine_sound(mut car_query: Query<&Velocity, With<Car>>, car_sound: Res<CarSound>) {
    for velocity in car_query.iter_mut() {
        let vel = velocity.linvel.length();
        let pitch: f32 = if vel < 0.1 {
            VELOCITY_PITCH_K
        } else {
            VELOCITY_PITCH_K + vel * 2.
        };
        car_sound.set_pitch(pitch);
    }
}
fn engine_sound_vol(input: Res<Input<KeyCode>>, car_sound: Res<CarSound>) {
    if input.just_pressed(KeyCode::Z) {
        let vol = car_sound.vol.value();
        println!("volume {vol:.1}-0.1");
        car_sound.set_vol(vol - 0.1);
    } else if input.just_pressed(KeyCode::C) {
        let vol = car_sound.vol.value();
        println!("volume {vol:.1}+0.1");
        car_sound.set_vol(vol + 0.1);
    }
}
// fn engine_sound_switch(
//     input: Res<Input<KeyCode>>,
//     dsp_manager: Res<DspManager>,
//     mut audio: ResMut<Audio<DspSource>>,
//     mut dsp_assets: ResMut<Assets<DspSource>>,
//     piano_id: Res<PianoId>,
//     audio_sinks: Res<Assets<AudioSink>>,
//     mut dsp: ResMut<Dsp>,
// ) {
//     if input.just_pressed(KeyCode::X) {
//         if let Some(sink_handle) = &dsp.engine_sink {
//             dbg!(sink_handle);
//             // TODO investigate this doesn't work
//             if let Some(sink) = audio_sinks.get(&sink_handle) {
//                 dbg!(sink.volume());
//             } else {
//                 println!("no sink in audio_sinks");
//             }
//         } else {
//             let source = dsp_manager
//                 .get_graph_by_id(&piano_id.0)
//                 .unwrap_or_else(|| panic!("DSP source not found!"));
//             let sink = audio.play_dsp(dsp_assets.as_mut(), source);
//             dsp.engine_sink = Some(sink);
//         }
//     }
// }
fn engine_sound_start(
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
