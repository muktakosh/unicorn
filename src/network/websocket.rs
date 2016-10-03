//! WebSocket implementation for unicorn.

use ws::{WebSocket as WS, Factory, Sender, Handler, Result, Message, Handshake, CloseCode};
use jsonrpc_core::{IoHandler, SyncMethodCommand, AsyncMethodCommand};

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::fmt;

type SocketRegistry = Rc<RefCell<HashMap<u64, Sender>>>;

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

/// WebSocket handler that handles each client
struct SocketHandler {
    id: u64,
    sender: Sender,
    jrpc_handler: Rc<IoHandler>,
    conn_type: SocketType,
    registry: SocketRegistry,
}

impl SocketHandler {
    fn add_sender(&self, id: u64, sender: Sender) {
        debug!("[socket] Adding sender: {}. Type: {}", id, self.conn_type);
        self.registry.borrow_mut().insert(id, sender);
    }
}

impl Handler for SocketHandler {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        self.add_sender(self.id, self.sender.clone());
        Ok(())
    }

    fn on_message(&mut self, m: Message) -> Result<()> {
        if let Some(s) = self.jrpc_handler.handle_request_sync(m.as_text().unwrap_or("")) {
            try!(self.sender.send(Message::text(s)));
        }
        Ok(())
    }

    fn on_close(&mut self, _: CloseCode, _: &str) {
        debug!("[socket] Removing sender: {}. Type: {}",
               self.id,
               self.conn_type);
        self.registry.borrow_mut().remove(&self.id);
    }
}

/// Factory for generating `SocketHandler`
struct SocketFactory {
    jrpc_handler: Rc<IoHandler>,
    registry: SocketRegistry,
    counter: u64,
}

impl SocketFactory {
    fn new_handler(&mut self, s: Sender, t: SocketType) -> SocketHandler {
        self.counter = self.counter + 1;
        SocketHandler {
            id: self.counter,
            sender: s,
            registry: self.registry.clone(),
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
    jrpc_handler: Rc<IoHandler>,
}

impl WebSocket {
    pub fn new() -> Self {
        // Do more stuff here
        WebSocket {
            sock: None,
            jrpc_handler: Rc::new(IoHandler::new()),
        }
    }

    pub fn add_method<C>(&mut self, name: &str, command: C)
        where C: SyncMethodCommand + 'static
    {
        self.jrpc_handler.add_method(name, command);
    }

    pub fn add_async_method<AC>(&mut self, name: &str, command: AC)
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
            registry: Rc::new(RefCell::new(HashMap::new())),
            counter: 0,
        };
        let s = try!(WS::new(factory));
        self.sock = Some(s);
        Ok(self)
    }
}
