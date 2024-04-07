use std::{error::Error, fmt};

use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum DecodeError {
    InvalidContent,
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidContent => write!(f, "message does not match `BaseMessage`"),
        }
    }
}

impl Error for DecodeError {}

#[derive(Deserialize)]
struct BaseMessage {
    method: String,
}

#[derive(Debug, Default, Serialize)]
pub struct DecodeResponse {
    pub method: String,
    pub content: String,
}

/// Encodes a message to a Json String.
/// And calculates its content-length
pub fn encode_message<T: Serialize>(msg: &T) -> serde_json::Result<String> {
    let content = serde_json::to_string(&msg)?;

    let message = format!("Content-Length: {}\r\n\r\n{}", content.len(), content);

    Ok(message)
}

/// Decodes a Json String to a struct
pub fn decode_message(msg: &str) -> Result<DecodeResponse, DecodeError> {
    let base_message: BaseMessage =
        serde_json::from_str(msg).map_err(|_| DecodeError::InvalidContent)?;

    Ok(DecodeResponse {
        method: base_message.method,
        content: msg.into(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encoding() {
        #[derive(Serialize)]
        struct EncodingTest {
            testing: bool,
        }
        let encoding_example = EncodingTest { testing: true };

        let expected = "Content-Length: 16\r\n\r\n{\"testing\":true}";
        let actual = encode_message(&encoding_example).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_decoding() {
        let incoming_msg = "{\"method\":\"hi\"}";
        let resp = decode_message(&incoming_msg).unwrap();

        dbg!(&resp.content);
        assert_eq!(resp.content.len(), 15);
        assert_eq!(resp.method, "hi");
    }

    #[test]
    #[should_panic]
    fn test_decoding_invalid_content_lenght() {
        let incoming_msg = "Content-length: 1s\r\n\r\n{\"method\":\"hi\"}";
        let resp = decode_message(&incoming_msg).unwrap();

        assert_eq!(resp.content.len(), 15);
        assert_eq!(resp.method, "hi");
    }
}
