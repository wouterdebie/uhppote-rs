use super::{Request, RequestResponseType, Response, HEADER};
use crate::messages::utils::types::{DateBCD, TimeWithoutSecondsBCD};
use bincode::{Decode, Encode};

#[derive(Encode, Request)]
pub struct GetTimeProfileRequest {
    header: u8,
    message_type: u8,
    _unused: u16,
    device_id: u32,
    profile_id: u8,
}

impl GetTimeProfileRequest {
    pub fn new(device_id: u32, profile_id: u8) -> Self {
        Self {
            header: HEADER,
            message_type: RequestResponseType::GetTimeProfile.into(),
            _unused: 0,
            device_id,
            profile_id,
        }
    }
}
#[test]
fn get_time_profile_request_to_bytes() {
    let expected = [
        0x17, 0x98, 0x00, 0x00, 0x2d, 0x55, 0x39, 0x19, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ];

    let r = GetTimeProfileRequest::new(423187757, 4);

    let actual = r.to_bytes();
    assert_eq!(expected, actual);
}

#[derive(Decode, Response, Debug)]
pub struct GetTimeProfileResponse {
    pub header: u8,
    pub message_type: u8,
    _unused: u16,
    pub device_id: u32,
    pub profile_id: u8,
    pub from: DateBCD,
    pub to: DateBCD,
    pub monday: bool,
    pub tuesday: bool,
    pub wednesday: bool,
    pub thursday: bool,
    pub friday: bool,
    pub saturday: bool,
    pub sunday: bool,
    pub segment1_start: TimeWithoutSecondsBCD,
    pub segment1_end: TimeWithoutSecondsBCD,
    pub segment2_start: TimeWithoutSecondsBCD,
    pub segment2_end: TimeWithoutSecondsBCD,
    pub segment3_start: TimeWithoutSecondsBCD,
    pub segment3_end: TimeWithoutSecondsBCD,
    pub linked_profile_id: u8,
}

#[test]
fn get_time_profile_response_from_bytes() {
    let bytes: [u8; 64] = [
        0x17, 0x98, 0x00, 0x00, 0x2d, 0x55, 0x39, 0x19, 0x04, 0x20, 0x21, 0x04, 0x01, 0x20, 0x21,
        0x12, 0x29, 0x01, 0x01, 0x00, 0x01, 0x00, 0x01, 0x01, 0x08, 0x30, 0x09, 0x45, 0x11, 0x35,
        0x13, 0x15, 0x14, 0x01, 0x17, 0x59, 0x13, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ];

    let r = GetTimeProfileResponse::from_bytes(&bytes).unwrap();
    assert_eq!(r.message_type, RequestResponseType::GetTimeProfile.into());
    assert_eq!(r.device_id, 423187757);
    assert_eq!(r.profile_id, 4);
    assert_eq!(r.from, DateBCD::new(2021, 4, 1));
    assert_eq!(r.to, DateBCD::new(2021, 12, 29));
    assert!(r.monday);
    assert!(r.tuesday);
    assert!(!r.wednesday);
    assert!(r.thursday);
    assert!(!r.friday);
    assert!(r.saturday);
    assert!(r.sunday);
    assert_eq!(r.segment1_start, TimeWithoutSecondsBCD::new(8, 30));
    assert_eq!(r.segment1_end, TimeWithoutSecondsBCD::new(9, 45));
    assert_eq!(r.segment2_start, TimeWithoutSecondsBCD::new(11, 35));
    assert_eq!(r.segment2_end, TimeWithoutSecondsBCD::new(13, 15));
    assert_eq!(r.segment3_start, TimeWithoutSecondsBCD::new(14, 1));
    assert_eq!(r.segment3_end, TimeWithoutSecondsBCD::new(17, 59));
    assert_eq!(r.linked_profile_id, 19);
}
