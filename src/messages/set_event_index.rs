use super::{Request, RequestResponseType, Response, HEADER};
use bincode::{Decode, Encode};

#[derive(Encode, Request)]
pub struct SetEventIndexRequest {
    header: u8,
    message_type: u8,
    _unused: u16,
    device_id: u32,
    index: u32,
    magic_word: u32,
}

impl SetEventIndexRequest {
    pub fn new(device_id: u32, index: u32, magic_word: u32) -> Self {
        Self {
            header: HEADER,
            message_type: RequestResponseType::SetEventIndex.into(),
            _unused: 0,
            device_id,
            index,
            magic_word,
        }
    }
}

#[derive(Decode, Response, Debug)]
pub struct SetEventIndexResponse {
    pub header: u8,
    pub message_type: u8,
    _unused: u16,
    pub device_id: u32,
    pub success: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn set_event_index_request_to_bytes() {
        let expected = [
            0x17, 0xb2, 0x00, 0x00, 0x2d, 0x55, 0x39, 0x19, 0x11, 0x00, 0x00, 0x00, 0x55, 0xaa,
            0xaa, 0x55, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        let r = SetEventIndexRequest::new(423187757, 17, 0x55aaaa55);

        let actual = r.to_bytes();
        assert_eq!(expected, actual);
    }

    #[test]
    fn set_event_index_response_from_bytes() {
        let bytes: [u8; 64] = [
            0x17, 0xb2, 0x00, 0x00, 0x2d, 0x55, 0x39, 0x19, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        let r = SetEventIndexResponse::from_bytes(&bytes).unwrap();
        assert_eq!(r.message_type, RequestResponseType::SetEventIndex.into());
        assert!(r.success);
    }
}
