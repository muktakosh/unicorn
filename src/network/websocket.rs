//! `WebSocket` implementation for unicorn.

use ws::{WebSocket as WS, Factory, Sender, Handler, Result, Message, Handshake, CloseCode};
use serde_json;

use std::sync::{Arc, Mutex};
use std::fmt;
use std::collections::HashMap;
use std::clone::Clone;

use schema::message_schema::{MessageRequest, MessageResponse};

/// Trait to implement handling of Sender
pub trait APIHandlerCommand {
    fn execute(&mut self, ws: Sender, msg: Message) -> Option<MessageResponse>;
}

/// Handler for websocket
#[derive(Debug, Clone)]
pub struct APIHandler<'a, H: APIHandlerCommand + 'static> {
    handlers: HashMap<&'a str, H>,
}

impl<'a, H: APIHandlerCommand + 'static> APIHandler<'a, H> {
    pub fn new() -> Self {
        APIHandler { handlers: HashMap::new() }
    }

    pub fn add_handler(&mut self, id: &'a str, handler: H) {
        self.handlers.insert(id, handler);
    }

    pub fn handle(&mut self, s: Sender, m: Message) -> Option<MessageResponse> {
        if m.is_text() {
            let req = match serde_json::from_str::<MessageRequest>(m.as_text().unwrap_or("")) {
                Ok(req) => req,
                Err(e) => {
                    error!("{:}", e);
                    return Some(MessageResponse::error("unicorn.error", "InvalidRequest"));
                }
            };
            match self.handlers.get_mut(&req.method[..]) {
                Some(h) => return h.execute(s, m),
                None => {
                    return Some(MessageResponse::error("unicorn.error", "MethodNotFound"));
                }
            }
        }
        None
    }
}

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

/// `WebSocket` handler that handles each connection
struct SocketHandler<'a, H: APIHandlerCommand + 'static> {
    id: u64,
    sender: Sender,
    handler: Arc<Mutex<APIHandler<'a, H>>>,
    conn_type: SocketType,
}

impl<'a, H: APIHandlerCommand + 'static> Handler for SocketHandler<'a, H> {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        debug!("[socket] Opening connection. sender: {}. Type: {}",
               self.id,
               self.conn_type);
        // TODO: Perform tasks immediately after connection open.
        Ok(())
    }

    fn on_message(&mut self, m: Message) -> Result<()> {
        if let Ok(mut l) = self.handler.lock() {
            if let Some(res) = l.handle(self.sender.clone(), m) {
                if let Ok(t) = serde_json::to_string(&res) {
                    let _ = self.sender.send(Message::Text(t));
                }
            }
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
struct SocketFactory<'a, H: APIHandlerCommand + 'static> {
    handler: Arc<Mutex<APIHandler<'a, H>>>,
    counter: u64,
}

impl<'a, H: APIHandlerCommand + 'static> SocketFactory<'a, H> {
    fn new_handler(&mut self, s: Sender, t: SocketType) -> SocketHandler<'a, H> {
        self.counter += 1;
        SocketHandler {
            id: self.counter,
            sender: s,
            handler: self.handler.clone(),
            conn_type: t,
        }
    }
}

impl<'a, H: APIHandlerCommand + 'static> Factory for SocketFactory<'a, H> {
    type Handler = SocketHandler<'a, H>;

    fn connection_made(&mut self, s: Sender) -> SocketHandler<'a, H> {
        self.new_handler(s, SocketType::Client)
    }

    fn server_connected(&mut self, s: Sender) -> SocketHandler<'a, H> {
        self.new_handler(s, SocketType::Server)
    }
}

/// JSON over `WebSockets` implementation with multi-client support
pub struct WebSocket<'a, H: APIHandlerCommand + 'static> {
    sock: Option<WS<SocketFactory<'a, H>>>,
    handler: APIHandler<'a, H>,
}

impl<'a, H: APIHandlerCommand + 'static> WebSocket<'a, H> {
    pub fn new() -> Self {
        // Do more stuff here
        WebSocket {
            sock: None,
            handler: APIHandler::new(),
        }
    }

    pub fn add_method(&mut self, name: &'a str, command: H) {
        self.handler.add_handler(name, command);
    }

    pub fn listen(mut self, addr: &str) -> Result<()> {
        let factory = SocketFactory {
            handler: Arc::new(Mutex::new(self.handler)),
            counter: 0,
        };
        let s = WS::new(factory)?;

        self.sock = Some(s.listen(addr).unwrap());
        Ok(())
    }
}
