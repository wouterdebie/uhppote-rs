use super::{Request, RequestResponseType, Response, HEADER};
use bincode::{Decode, Encode};

#[derive(Encode, Request)]
pub struct GetEventIndexRequest {
    header: u8,
    message_type: u8,
    _unused: u16,
    device_id: u32,
}

impl GetEventIndexRequest {
    pub fn new(device_id: u32) -> Self {
        Self {
            header: HEADER,
            message_type: RequestResponseType::GetEventIndex.into(),
            _unused: 0,
            device_id,
        }
    }
}

#[derive(Decode, Response, Debug)]
pub struct GetEventIndexResponse {
    pub header: u8,
    pub message_type: u8,
    _unused: u16,
    pub device_id: u32,
    pub index: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_event_index_request_to_bytes() {
        let expected = [
            0x17, 0xb4, 0x00, 0x00, 0x2d, 0x55, 0x39, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        let r = GetEventIndexRequest::new(423187757);

        let actual = r.to_bytes();
        assert_eq!(expected, actual);
    }

    #[test]
    fn get_event_index_response_from_bytes() {
        let bytes: [u8; 64] = [
            0x17, 0xb4, 0x00, 0x00, 0x2d, 0x55, 0x39, 0x19, 0x11, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        let r = GetEventIndexResponse::from_bytes(&bytes).unwrap();
        assert_eq!(r.message_type, RequestResponseType::GetEventIndex.into());
        assert_eq!(r.device_id, 423187757);
        assert_eq!(r.index, 17);
    }
}
