//! WebSocket implementation for unicorn.

use ws::{WebSocket as WS, Factory, Sender, Handler, Result, Message, Handshake, CloseCode};
use jsonrpc_core::{IoHandler, SyncMethodCommand, AsyncMethodCommand};

use std::sync::Arc;
use std::fmt;

/// Types of `Socket`
pub enum SocketType {
    Server,
    Client,
}

impl fmt::Display for SocketType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let t = match *self {
            SocketType::Server => "Server".to_string(),
            SocketType::Client => "Client".to_string(),
        };
        write!(f, "{}", t)
    }
}

/// WebSocket handler that handles each connection
struct SocketHandler {
    id: u64,
    sender: Sender,
    jrpc_handler: Arc<IoHandler>,
    conn_type: SocketType,
}

impl Handler for SocketHandler {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        debug!("[socket] Opening connection. sender: {}. Type: {}",
               self.id,
               self.conn_type);
        // TODO: Perform tasks immediately after connection open.
        Ok(())
    }

    fn on_message(&mut self, m: Message) -> Result<()> {
        let sclone = self.sender.clone();

        if let Some(ps) = self.jrpc_handler.handle_request(m.as_text().unwrap_or("")) {
            ps.on_result(move |s: String| {
                let _ = sclone.send(Message::text(s));
            });
        }
        Ok(())
    }

    fn on_close(&mut self, _: CloseCode, _: &str) {
        debug!("[socket] Removing sender: {}. Type: {}",
               self.id,
               self.conn_type);
        // TODO: Do connection cleanup here
    }
}

/// Factory for generating `SocketHandler`
struct SocketFactory {
    jrpc_handler: Arc<IoHandler>,
    counter: u64,
}

impl SocketFactory {
    fn new_handler(&mut self, s: Sender, t: SocketType) -> SocketHandler {
        self.counter = self.counter + 1;
        SocketHandler {
            id: self.counter,
            sender: s,
            jrpc_handler: self.jrpc_handler.clone(),
            conn_type: t,
        }
    }
}

impl Factory for SocketFactory {
    type Handler = SocketHandler;

    fn connection_made(&mut self, s: Sender) -> SocketHandler {
        self.new_handler(s, SocketType::Client)
    }

    fn server_connected(&mut self, s: Sender) -> SocketHandler {
        self.new_handler(s, SocketType::Server)
    }
}

/// JSON-RPC over WebSockets implementation with multi-client support
pub struct WebSocket {
    sock: Option<WS<SocketFactory>>,
    jrpc_handler: Arc<IoHandler>,
}

impl WebSocket {
    pub fn new() -> Self {
        // Do more stuff here
        WebSocket {
            sock: None,
            jrpc_handler: Arc::new(IoHandler::new()),
        }
    }

    pub fn add_sync_method<C>(&mut self, name: &str, command: C)
        where C: SyncMethodCommand + 'static
    {
        self.jrpc_handler.add_method(name, command);
    }

    pub fn add_method<AC>(&mut self, name: &str, command: AC)
        where AC: AsyncMethodCommand + 'static
    {
        self.jrpc_handler.add_async_method(name, command);
    }

    pub fn listen(mut self, addr: &str) -> Result<Self> {
        self = try!(self.build());

        self.sock = Some(try!(self.sock.unwrap().listen(addr)));
        Ok(self)
    }

    fn build(mut self) -> Result<Self> {
        let factory = SocketFactory {
            jrpc_handler: self.jrpc_handler.clone(),
            counter: 0,
        };
        let s = try!(WS::new(factory));
        self.sock = Some(s);
        Ok(self)
    }
}
