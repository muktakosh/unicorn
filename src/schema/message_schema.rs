/// Schema for messages

use std::string::String;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessageRequest {
    pub method: String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessageRequestText<T> {
    pub method: String,
    pub payload: Option<T>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessageRequestBinary {
    pub method: String,
    pub payload: Vec<u8>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessageResponse {
    pub event: String,
    pub payload: Option<String>,
    pub error: Option<String>,
}

impl MessageResponse {
    pub fn error(ev: &str, err: &str) -> Self {
        MessageResponse {
            event: String::from(ev),
            payload: None,
            error: Some(String::from(err)),
        }
    }

    pub fn success(ev: &str, payload: String) -> Self {
        MessageResponse {
            event: String::from(ev),
            payload: Some(payload),
            error: None,
        }
    }
}
