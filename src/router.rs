//! Handles routing between topics

use ws::{Sender, Message};
use std::collections::HashMap;

pub enum RouterCommand {
    CreateTopic(String),
    Subscribe(String, String, Sender),
    Send(String, String, Message),
    Broadcast(String, Message),
    Unsubscribe(String, String),
}

pub struct Topic {
    id: String,
    subscribers: HashMap<String, Sender>,
}

impl Topic {
    pub fn new(id: String) -> Self {
        Topic {
            id: id,
            subscribers: HashMap::new(),
        }
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn add_subscriber(&mut self, id: String, sender: Sender) {
        self.subscribers.insert(id, sender);
    }

    pub fn remove_subscriber(&mut self, id: &String) {
        self.subscribers.remove(id);
    }

    pub fn get_subscriber(&self, id: &String) -> Option<&Sender> {
        self.subscribers.get(id)
    }

    pub fn send(&self, sender_id: &String, m: Message) {
        for (id, sender) in &self.subscribers {
            if id != sender_id {
                let _ = sender.send(m.clone());
            }
        }
    }

    pub fn broadcast(&self, m: Message) {
        for s in self.subscribers.values() {
            let _ = s.send(m.clone());
        }
    }
}

pub struct Registry {
    topics: HashMap<String, Topic>,
}

impl Registry {
    pub fn new() -> Self {
        Registry { topics: HashMap::new() }
    }

    pub fn create_topic(&mut self, id: String) {
        self.topics.insert(id.clone(), Topic::new(id));
    }

    pub fn subscribe(&mut self, topic_id: String, subscriber_id: String, sender: Sender) {
        let topic = self.topics.entry(topic_id.clone()).or_insert(Topic::new(topic_id));
        topic.add_subscriber(subscriber_id, sender);
    }

    pub fn unsubscribe(&mut self, topic_id: &String, subscriber_id: &String) {
        if let Some(mut t) = self.topics.get_mut(topic_id) {
            t.remove_subscriber(subscriber_id);
        }
    }

    pub fn send(&mut self, topic_id: &String, sender_id: &String, m: Message) {
        if let Some(t) = self.topics.get(topic_id) {
            t.send(sender_id, m);
        }
    }

    pub fn broadcast(&mut self, topic_id: &String, m: Message) {
        if let Some(t) = self.topics.get(topic_id) {
            t.broadcast(m);
        }
    }

    pub fn parse_command(&mut self, c: RouterCommand) {
        match c {
            RouterCommand::CreateTopic(tid) => self.create_topic(tid),
            RouterCommand::Subscribe(tid, sid, s) => self.subscribe(tid, sid, s),
            RouterCommand::Broadcast(tid, m) => self.broadcast(&tid, m),
            RouterCommand::Send(tid, sid, m) => self.send(&tid, &sid, m),
            RouterCommand::Unsubscribe(tid, sid) => self.unsubscribe(&tid, &sid),
        }
    }
}
