//! Orchestration and task management layer for `unicorn`.

use network::websocket::WebSocket;
use api;
use schema::config_schema::{Config, Service};

/// Entry point for `kernel`
pub fn run(conf: Config) {
    debug!("Starting kernel...");

    let kernelconf: &Service = conf.services.get("api").unwrap();

    let mut socket = WebSocket::new();

    // Add methods
    socket.add_method("unicorn.register", api::register::RegisterAPI{});

    // Start the listener
    socket.listen(kernelconf.address().as_ref()).unwrap();
}
