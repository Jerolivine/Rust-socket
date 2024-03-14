use rust_socketio::{ClientBuilder, Event, Payload, RawClient};
use serde_json::json;
use std::time::Duration;

use std::{
    sync::atomic::{AtomicUsize, Ordering},
    time::UNIX_EPOCH,
};


fn main() {

    static CONNECT_NUM: AtomicUsize = AtomicUsize::new(0);
    static CLOSE_NUM: AtomicUsize = AtomicUsize::new(0);
    static MESSAGE_NUM: AtomicUsize = AtomicUsize::new(0);

    let socket = ClientBuilder::new("http://localhost:3000")
        .reconnect(true)
        .max_reconnect_attempts(255)
        .reconnect_delay(100, 100)
        .on("error", |err: Payload, _| eprintln!("Error: {:?}", err))
        .on("send-client-message", move |_, _socket| {
            println!("send-client-message");
            MESSAGE_NUM.fetch_add(1, Ordering::Release);
        })
        .connect();

    loop{

    }

}
