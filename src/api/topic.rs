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
use schema::topic_schema::{TopicCreate, TopicPublish, TopicSubscribe};

#[derive(Clone)]
enum ActionType {
    Create,
    Subscribe,
    Publish,
    Unsubscribe
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
            "publish" => Some(ActionType::Publish),
            "subscribe" => Some(ActionType::Subscribe),
            "unsubscribe" => Some(ActionType::Unsubscribe),
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
        let invalid_payload = Some(MessageResponse::error("topic", "InvalidPayload"));

        match self.actiontype {
            Some(ActionType::Create) => {
                if let Ok(q) = from_str::<MessageRequestText<TopicCreate>>(m.as_text().unwrap_or("")) {
                    self.transmit(RouterCommand::CreateTopic(q.payload.unwrap().topic_id));
                    return None;
                } else {
                    return invalid_payload;
                }
            }
            Some(ActionType::Publish) => {
                if let Ok(q) = from_str::<MessageRequestText<TopicPublish>>(m.as_text().unwrap_or("")) {
                    let payload = q.payload.unwrap();
                    self.transmit(RouterCommand::Send(payload.topic_id,
                                                      payload.publisher_id,
                                                      WSMessage::text(payload.message)));
                    return None;
                } else {
                    return invalid_payload;
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
                    return invalid_payload;
                }
            }
            Some(ActionType::Unsubscribe) => {
                if let Ok(q) = from_str::<MessageRequestText<TopicSubscribe>>(m.as_text().unwrap_or("")) {
                    let payload = q.payload.unwrap();
                    self.transmit(RouterCommand::Unsubscribe(payload.topic_id,
                                                             payload.subscriber_id));
                    return None;
                } else {
                    return invalid_payload;
                }
            }
            None => None,
        }
    }
}
