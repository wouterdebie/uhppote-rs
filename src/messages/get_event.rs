use super::{utils::types::DateTime, Request, RequestResponseType, Response, HEADER};
use bincode::{Decode, Encode};

#[derive(Encode, Request)]
pub struct GetEventRequest {
    header: u8,
    message_type: u8,
    _unused: u16,
    device_id: u32,
    index: u32,
}

impl GetEventRequest {
    pub fn new(device_id: u32, index: u32) -> Self {
        Self {
            header: HEADER,
            message_type: RequestResponseType::GetEvent.into(),
            _unused: 0,
            device_id,
            index,
        }
    }
}

#[derive(Decode, Response, Debug)]
pub struct GetEventResponse {
    pub header: u8,
    pub message_type: u8,
    _unused: u16,
    pub device_id: u32,
    pub index: u32,
    pub type_: u8,
    pub granted: bool,
    pub door: u8,
    pub direction: u8,
    pub card_number: u32,
    pub timestamp: DateTime,
    pub reason: u8,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_event_request_to_bytes() {
        let expected = [
            0x17, 0xb0, 0x00, 0x00, 0x2d, 0x55, 0x39, 0x19, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        let r = GetEventRequest::new(423187757, 1);

        let actual = r.to_bytes();
        assert_eq!(expected, actual);
    }

    #[test]
    fn get_event_response_from_bytes() {
        let bytes: [u8; 64] = [
            0x17, 0xb0, 0x00, 0x00, 0x2d, 0x55, 0x39, 0x19, 0x08, 0x00, 0x00, 0x00, 0x02, 0x01,
            0x03, 0x01, 0xad, 0xe8, 0x5d, 0x00, 0x20, 0x19, 0x02, 0x10, 0x07, 0x12, 0x01, 0x06,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x4a, 0x26, 0x80, 0x39, 0x08, 0x92, 0x00, 0x00,
        ];

        let r = GetEventResponse::from_bytes(&bytes).unwrap();
        assert_eq!(r.message_type, RequestResponseType::GetEvent.into());
        assert_eq!(r.device_id, 423187757);
        assert_eq!(r.index, 8);
        assert_eq!(r.type_, 2);
        assert!(r.granted);
        assert_eq!(r.door, 3);
        assert_eq!(r.direction, 1);
        assert_eq!(r.card_number, 6154413);
        assert_eq!(r.timestamp, DateTime::new(2019, 2, 10, 7, 12, 1));
        assert_eq!(r.reason, 6);
    }
}
