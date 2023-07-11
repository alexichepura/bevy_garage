use bevy::prelude::*;
use crossbeam_channel::{bounded, Receiver, Sender};
use serde::{Deserialize, Serialize};

use super::replay::ReplayBuffer;
use std::ops::RangeFrom;

pub const PERSIST_BATCH_SIZE: usize = 100;

#[derive(Deserialize, Serialize)]
pub struct ReplayBufferRecord {
    pub state: Vec<f32>,
    pub action: i32,
    pub reward: f64,
    pub next_state: Vec<f32>,
    pub done: bool,
}

pub fn get_replay_buffer_to_persist(rb: &ReplayBuffer) -> Vec<ReplayBufferRecord> {
    let save_start_index = rb.state.len() - PERSIST_BATCH_SIZE;
    let r: RangeFrom<usize> = save_start_index..;

    let records: Vec<ReplayBufferRecord> = rb.state.as_slice()[r]
        .iter()
        .enumerate()
        .map(|t| {
            let i = save_start_index + t.0;
            return ReplayBufferRecord {
                state: rb.state[i].to_vec(),
                action: rb.action[i] as i32,
                reward: rb.reward[i] as f64,
                next_state: rb.next_state[i].to_vec(),
                done: rb.done[i] == 1.,
            };
        })
        .collect();
    return records;
}

#[derive(Resource)]
pub struct ApiClient {
    #[cfg(not(target_arch = "wasm32"))]
    runtime: tokio::runtime::Runtime,
}
impl ApiClient {
    pub(crate) fn new() -> ApiClient {
        ApiClient {
            #[cfg(not(target_arch = "wasm32"))]
            runtime: tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .expect("Could not build tokio runtime"),
        }
    }
    // pub fn save_replay_buffer(&self, rb: Vec<ReplayBufferRecord>) {
    //     dbg!(rb.len());
    // }
    #[cfg(target_arch = "wasm32")]
    pub fn save_replay_buffer(&self, rb: Vec<ReplayBufferRecord>) {
        bevy::tasks::IoTaskPool::get()
            .spawn(async move {
                println!("rb batch seding {:?}", rb.len());
                let client = reqwest::Client::new();
                let api_result = client
                    .post("http://localhost:3000/api/replay")
                    .json(&rb)
                    .send()
                    .await;
                match api_result {
                    Ok(api_result) => {
                        let api_response_text = api_result.text().await.unwrap();
                        println!("rb batch sent {:?}", api_response_text);
                    }
                    Err(e) => println!("rb batch sending error: {}", e),
                }
            })
            .detach();
    }
    #[cfg(not(target_arch = "wasm32"))]
    pub fn save_replay_buffer(&self, rb: Vec<ReplayBufferRecord>) {
        self.runtime.spawn(async move {
            let client = reqwest::Client::new();
            let api_result = client
                .post("http://localhost:3000/api/replay")
                .json(&rb)
                .send()
                .await;

            match api_result {
                Ok(api_result) => {
                    let api_response_text = api_result.text().await.unwrap();
                    println!("rb batch sent {:?}", api_response_text);
                }
                Err(e) => println!("rb batch sending error: {}", e),
            }
        });
    }
}

#[derive(Resource, Deref)]
pub struct StreamReceiver(Receiver<String>);
#[derive(Resource, Deref)]
pub struct StreamSender(Sender<String>);
#[derive(Event)]
pub struct StreamEvent(String);

pub fn api_start_system(mut cmd: Commands) {
    let (tx, rx) = bounded::<String>(10);
    cmd.insert_resource(ApiClient::new());
    cmd.insert_resource(StreamReceiver(rx));
    cmd.insert_resource(StreamSender(tx));
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
