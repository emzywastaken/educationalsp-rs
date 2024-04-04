use std::{error::Error, fmt};

use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum DecodeError {
    SeparatorNotFound,
    InvalidContentLenght,
    InvalidContent,
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SeparatorNotFound => write!(f, "separator not found in message"),
            Self::InvalidContentLenght => {
                write!(f, "content-length is not allowed to contain letters")
            }
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

    let message = format!("Content-length: {}\r\n\r\n{}", content.len(), content);

    Ok(message)
}

/// Decodes a Json String to a struct
pub fn decode_message(msg: &str) -> Result<DecodeResponse, DecodeError> {
    let (header, content) = msg
        .split_once("\r\n\r\n")
        .ok_or(DecodeError::SeparatorNotFound)?;

    let content_length = &header["Content-length: ".len()..];
    let content_length = content_length
        .parse::<usize>()
        .map_err(|_| DecodeError::InvalidContentLenght)?;

    let base_message: BaseMessage =
        serde_json::from_str(content).map_err(|_| DecodeError::InvalidContent)?;

    Ok(DecodeResponse {
        method: base_message.method,
        content: content[..content_length].into(),
    })
}

pub fn split(msg: &str) -> Option<usize> {
    let (header, _content) = msg.split_once("\r\n\r\n")?;

    let content_lenght = &header["Content-Length: ".len()..];
    let content_lenght = content_lenght.parse::<usize>().ok()?;

    let total_lenght = header.len() + 4 + content_lenght;
    Some(total_lenght)
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

        let expected = "Content-length: 16\r\n\r\n{\"testing\":true}";
        let actual = encode_message(&encoding_example).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_decoding() {
        let incoming_msg = "Content-length: 15\r\n\r\n{\"method\":\"hi\"}";
        let resp = decode_message(&incoming_msg).unwrap();

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
