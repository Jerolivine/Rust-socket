use futures_util::FutureExt;
use rust_socketio::{
    asynchronous::{Client, ClientBuilder},
    Payload,
};
use serde_json::json;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let socket = ClientBuilder::new("http://localhost:3000/")
        // .namespace("/admin")
        .on("error", |err, _| {
            async move { eprintln!("Error: {:#?}", err) }.boxed()
        })
        .connect()
        .await
        .expect("Connection failed");

    let json_payload = json!({"myAckData": 123});

    socket
        .emit("test", json_payload)
        .await
        .expect("Server unreachable");
}
