/// Data structure for topics

/// Create a topic
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TopicCreate {
    pub topic_id: String,
}

/// Subscribe to a topic
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TopicSubscribe {
    pub topic_id: String,
    pub subscriber_id: String,
}

/// Send message to topic
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TopicPublish {
    pub topic_id: String,
    pub publisher_id: String,
    pub message: String,
}
