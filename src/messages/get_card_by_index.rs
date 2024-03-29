use super::{utils::types::DateBCD, Request, RequestResponseType, Response, HEADER};
use bincode::{Decode, Encode};

#[derive(Encode, Request)]
pub struct GetCardByIndexRequest {
    header: u8,
    message_type: u8,
    _unused: u16,
    device_id: u32,
    index: u32,
}

impl GetCardByIndexRequest {
    pub fn new(device_id: u32, index: u32) -> Self {
        GetCardByIndexRequest {
            header: HEADER,
            message_type: RequestResponseType::GetCardByIndex.into(),
            _unused: 0,
            device_id,
            index,
        }
    }
}

#[derive(Decode, Response, Debug)]
pub struct GetCardByIndexResponse {
    pub header: u8,
    pub message_type: u8,
    _unused: u16,
    pub device_id: u32,
    pub card_number: u32,
    pub from: DateBCD,
    pub to: DateBCD,
    pub door_1: u8,
    pub door_2: u8,
    pub door_3: u8,
    pub door_4: u8,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_card_by_index_request_to_bytes() {
        let expected = [
            0x17, 0x5c, 0x00, 0x00, 0x2D, 0x55, 0x39, 0x19, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        let r = GetCardByIndexRequest::new(423187757, 4);

        let actual = r.to_bytes();
        assert_eq!(expected, actual);
    }

    #[test]
    fn get_card_by_index_response_from_bytes() {
        let bytes: [u8; 64] = [
            0x17, 0x5c, 0x00, 0x00, 0x2d, 0x55, 0x39, 0x19, 0xac, 0xe8, 0x5d, 0x00, 0x20, 0x19,
            0x02, 0x03, 0x20, 0x19, 0x12, 0x29, 0x00, 0x00, 29, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        let r = GetCardByIndexResponse::from_bytes(&bytes).unwrap();
        assert_eq!(r.message_type, RequestResponseType::GetCardByIndex.into());
        assert_eq!(r.device_id, 423187757);
        assert_eq!(r.card_number, 6154412);
        assert_eq!(r.from, DateBCD::new(2019, 2, 3));
        assert_eq!(r.to, DateBCD::new(2019, 12, 29));
        assert_eq!(r.door_1, 0);
        assert_eq!(r.door_2, 0);
        assert_eq!(r.door_3, 29);
        assert_eq!(r.door_4, 1);
    }
}
