//! Orchestration and task management layer for `unicorn`.

use network::websocket::WebSocket;
use api;
use router::{Registry, RouterCommand};
use schema::config_schema::{Config, Service};

use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread;

/// Entry point for `kernel`
pub fn run(conf: Config) {
    debug!("Starting kernel...");

    let kernelconf: &Service = &conf.services["api"];

    let mut socket = WebSocket::new();

    let (tx, rx) = channel::<RouterCommand>();

    thread::spawn(move || {
        let mut reg = Registry::new();
        loop {
            match rx.recv() {
                Ok(c) => reg.parse_command(c),
                Err(e) => error!("Error parsing command: {}", e),
            }
        }
    });

    // Add topic methods
    let topicapi = api::topic::TopicAPI::with_tx(Arc::new(Mutex::new(tx.clone())));
    socket.add_method("topic.create", topicapi.clone().set_type("create"));
    socket.add_method("topic.subscribe", topicapi.clone().set_type("subscribe"));
    socket.add_method("topic.publish", topicapi.clone().set_type("publish"));
    socket.add_method("topic.unsubscribe", topicapi.clone().set_type("unsubscribe"));

    // Start the listener
    socket.listen(kernelconf.address().as_ref()).unwrap();
}
