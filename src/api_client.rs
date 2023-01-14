use bevy::prelude::*;
use crossbeam_channel::{bounded, Receiver, Sender};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ReplayBufferRecord {
    pub state: Vec<f32>,
    pub action: i32,
    pub reward: f64,
    pub next_state: Vec<f32>,
    pub done: bool,
}

#[derive(Resource)]
pub struct ApiClient {
    #[cfg(all(not(target_arch = "wasm32"), not(target_os = "android")))]
    runtime: tokio::runtime::Runtime,
}
impl ApiClient {
    pub(crate) fn new() -> ApiClient {
        ApiClient {
            #[cfg(all(not(target_arch = "wasm32"), not(target_os = "android")))]
            runtime: tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .expect("Could not build tokio runtime"),
        }
    }
    pub fn save_replay_buffer(&self, rb: Vec<ReplayBufferRecord>) {
        dbg!(rb.len());
    }
    // #[cfg(any(target_arch = "wasm32", target_os = "android"))]
    // pub fn save_replay_buffer(&self, rb: Vec<ReplayBufferRecord>) {
    //     bevy::tasks::IoTaskPool::get()
    //         .spawn(async move {
    //             println!("rb batch seding {:?}", rb.len());
    //             let client = reqwest::Client::new();
    //             let api_result = client
    //                 .post("http://localhost:3000/api/replay")
    //                 .json(&rb)
    //                 .send()
    //                 .await;
    //             let api_response_text = api_result.unwrap().text().await.unwrap();
    //             println!("rb batch sent {:?}", api_response_text);
    //         })
    //         .detach();
    // }
    // #[cfg(all(not(target_arch = "wasm32"), not(target_os = "android")))]
    // pub fn save_replay_buffer(&self, rb: Vec<ReplayBufferRecord>) {
    //     self.runtime.spawn(async move {
    //         let client = reqwest::Client::new();
    //         let api_result = client
    //             .post("http://localhost:3000/api/replay")
    //             .json(&rb)
    //             .send()
    //             .await;
    //         let api_response_text = api_result.unwrap().text().await.unwrap();
    //         println!("rb batch sent {:?}", api_response_text);
    //     });
    // }
}

#[derive(Resource, Deref)]
pub struct StreamReceiver(Receiver<String>);
#[derive(Resource, Deref)]
pub struct StreamSender(Sender<String>);
pub struct StreamEvent(String);

pub fn api_start_system(mut commands: Commands) {
    let (tx, rx) = bounded::<String>(10);
    commands.insert_resource(ApiClient::new());
    commands.insert_resource(StreamReceiver(rx));
    commands.insert_resource(StreamSender(tx));
}

// pub fn api_send_system(input: Res<Input<KeyCode>>, api: Res<ApiClient>, sender: Res<StreamSender>) {
//     if !input.just_pressed(KeyCode::T) {
//         return;
//     }
//     let sender = sender.clone();
//     api.hello(sender);
// }
pub fn api_read_stream_event_writer_system(
    receiver: Res<StreamReceiver>,
    mut events: EventWriter<StreamEvent>,
) {
    for from_stream in receiver.try_iter() {
        events.send(StreamEvent(from_stream));
    }
}
pub fn api_event_reader_system(mut reader: EventReader<StreamEvent>) {
    for event in reader.iter() {
        dbg!(&event.0);
    }
}
