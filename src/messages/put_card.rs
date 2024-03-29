use super::{utils::types::DateBCD, Request, RequestResponseType, Response, HEADER};
use bincode::{Decode, Encode};

#[derive(Encode, Request)]

pub struct PutCardRequest {
    header: u8,
    message_type: u8,
    _unused: u16,
    device_id: u32,
    card_id: u32,
    from: DateBCD,
    to: DateBCD,
    door_1: u8,
    door_2: u8,
    door_3: u8,
    door_4: u8,
}

#[allow(clippy::too_many_arguments)]
impl PutCardRequest {
    pub fn new(
        device_id: u32,
        card_id: u32,
        from: DateBCD,
        to: DateBCD,
        door_1: u8,
        door_2: u8,
        door_3: u8,
        door_4: u8,
    ) -> Self {
        PutCardRequest {
            header: HEADER,
            message_type: RequestResponseType::PutCard.into(),
            _unused: 0,
            device_id,
            card_id,
            from,
            to,
            door_1,
            door_2,
            door_3,
            door_4,
        }
    }
}

#[derive(Decode, Response, Debug)]
pub struct PutCardResponse {
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
    fn put_card_request_to_bytes() {
        let expected = [
            0x17, 0x50, 0x00, 0x00, 0x2d, 0x55, 0x39, 0x19, 0xac, 0xe8, 0x5d, 0x00, 0x20, 0x19,
            0x01, 0x02, 0x20, 0x19, 0x12, 0x31, 0x01, 0x00, 29, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        let r = PutCardRequest::new(
            423187757,
            6154412,
            DateBCD::new(2019, 1, 2),
            DateBCD::new(2019, 12, 31),
            1,
            0,
            29,
            1,
        );

        let actual = r.to_bytes();
        assert_eq!(expected, actual);
    }

    #[test]
    fn put_card_response_from_bytes() {
        let bytes: [u8; 64] = [
            0x17, 0x50, 0x00, 0x00, 0x2d, 0x55, 0x39, 0x19, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        let r = PutCardResponse::from_bytes(&bytes).unwrap();
        assert_eq!(r.message_type, RequestResponseType::PutCard.into());
        assert!(r.success);
    }
}
