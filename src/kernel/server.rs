/// Run the `kernel` service

use network::websocket::WebSocket;
use api;

pub fn run() {
    debug!("Starting kernel...");

    let mut socket = WebSocket::new();

    // Add methods
    socket.add_method("unicorn.register", api::register::RegisterAPI{});

    // Start the listener
    socket.listen("0.0.0.0:60000").unwrap();
}
