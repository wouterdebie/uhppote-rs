use bincode::{Decode, Encode};

use super::{Request, RequestResponseType, Response, HEADER};

#[derive(Encode, Request)]
pub struct ClearTimeProfilesRequest {
    header: u8,
    message_type: u8,
    _unused: u16,
    device_id: u32,
    magic_word: u32,
}

impl ClearTimeProfilesRequest {
    pub fn new(device_id: u32, magic_word: u32) -> Self {
        ClearTimeProfilesRequest {
            header: HEADER,
            message_type: RequestResponseType::ClearTimeProfiles.into(),
            _unused: 0,
            device_id,
            magic_word,
        }
    }
}

#[test]
fn clear_time_profile_request_to_bytes() {
    let expected = [
        0x17, 0x8a, 0x00, 0x00, 0x2d, 0x55, 0x39, 0x19, 0x55, 0xaa, 0xaa, 0x55, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ];

    let r = ClearTimeProfilesRequest::new(423187757, 0x55aaaa55);

    let actual = r.to_bytes();
    assert_eq!(expected, actual);
}

#[derive(Decode, Response, Debug)]
pub struct ClearTimeProfilesResponse {
    pub header: u8,
    pub message_type: u8,
    _unused: u16,
    pub device_id: u32,
    pub magic_word: u32,
}

#[test]
fn clear_time_profile_response_from_bytes() {
    let bytes: [u8; 64] = [
        0x17, 0x8a, 0x00, 0x00, 0x2d, 0x55, 0x39, 0x19, 0x55, 0xaa, 0xaa, 0x55, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ];

    let r = ClearTimeProfilesResponse::from_bytes(&bytes).unwrap();
    assert_eq!(
        r.message_type,
        RequestResponseType::ClearTimeProfiles.into()
    );
    assert_eq!(r.device_id, 423187757);
    assert_eq!(r.magic_word, 0x55aaaa55);
}