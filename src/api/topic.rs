//! Topic API

use ws::Sender as WSSender;
use ws::Message as WSMessage;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

use serde_json::from_str;

use network::websocket::APIHandlerCommand;
use router::RouterCommand;
use schema::message_schema::MessageResponse;
use schema::message_schema::MessageRequestText;
use schema::topic_schema::{TopicCreate, TopicMessage, TopicSubscribe};

#[derive(Clone)]
enum ActionType {
    Create,
    Send,
    Subscribe,
}

#[derive(Clone)]
pub struct TopicAPI {
    tx: Arc<Mutex<Sender<RouterCommand>>>,
    actiontype: Option<ActionType>,
}

impl TopicAPI {
    pub fn with_tx(tx: Arc<Mutex<Sender<RouterCommand>>>) -> Self {
        TopicAPI {
            tx: tx,
            actiontype: None,
        }
    }

    pub fn set_type(mut self, t: &str) -> Self {
        self.actiontype = match t {
            "create" => Some(ActionType::Create),
            "send" => Some(ActionType::Send),
            "subscribe" => Some(ActionType::Subscribe),
            _ => None,
        };
        self
    }

    fn transmit(&self, c: RouterCommand) {
        if let Ok(t) = self.tx.lock() {
            let _ = t.send(c);
        }
    }
}

impl APIHandlerCommand for TopicAPI {
    fn execute(&mut self, s: WSSender, m: WSMessage) -> Option<MessageResponse> {
        match self.actiontype {
            Some(ActionType::Create) => {
                if let Ok(q) = from_str::<MessageRequestText<TopicCreate>>(m.as_text().unwrap_or("")) {
                    self.transmit(RouterCommand::CreateTopic(q.payload.unwrap().topic_id));
                    return None;
                } else {
                    return Some(MessageResponse::error("unicorn.topic.create", "InvalidPayload"));
                }
            }
            Some(ActionType::Send) => {
                if let Ok(q) = from_str::<MessageRequestText<TopicMessage>>(m.as_text().unwrap_or("")) {
                    let payload = q.payload.unwrap();
                    self.transmit(RouterCommand::Send(payload.topic_id,
                                                      payload.sender_id,
                                                      WSMessage::text(payload.message)));
                    return None;
                } else {
                    return Some(MessageResponse::error("unicorn.topic.send", "InvalidPayload"));
                }
            }
            Some(ActionType::Subscribe) => {
                if let Ok(q) = from_str::<MessageRequestText<TopicSubscribe>>(m.as_text().unwrap_or("")) {
                    let payload = q.payload.unwrap();
                    self.transmit(RouterCommand::Subscribe(payload.topic_id,
                                                           payload.subscriber_id,
                                                           s.clone()));
                    return None;
                } else {
                    return Some(MessageResponse::error("unicorn.topic.send", "InvalidPayload"));
                }
            }
            None => return None,
        }
    }
}
