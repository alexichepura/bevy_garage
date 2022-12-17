use bevy::prelude::*;
use crossbeam_channel::{bounded, Receiver, Sender};
use tokio::runtime::Runtime;

#[derive(Resource)]
pub struct ApiClient {
    runtime: Runtime,
}

impl ApiClient {
    pub(crate) fn new() -> ApiClient {
        ApiClient {
            runtime: tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .expect("Could not build tokio runtime"),
        }
    }
    pub fn hello(&self, sender: Sender<String>) {
        self.runtime.spawn(async move {
            let api_result = reqwest::get("http://localhost:3000/hello/bevy").await;
            if api_result.is_ok() {
                println!("r, {:?}", api_result);
            } else {
                println!("api_result error");
            }
            let api_response_text = api_result.unwrap().text().await.unwrap();
            sender.send(api_response_text);
        });
    }
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

pub fn api_send_system(input: Res<Input<KeyCode>>, api: Res<ApiClient>, sender: Res<StreamSender>) {
    if !input.just_pressed(KeyCode::T) {
        return;
    }
    let sender = sender.clone();
    api.hello(sender);
}
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
