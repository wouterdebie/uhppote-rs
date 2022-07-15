use super::{Request, RequestResponseType, Response, HEADER};
use crate::messages::utils::types::{DateBCD, TimeWithoutSecondsBCD};
use bincode::{Decode, Encode};

#[derive(Encode, Request)]
pub struct SetTimeProfileRequest {
    header: u8,
    message_type: u8,
    _unused: u16,
    device_id: u32,
    profile_id: u8,
    from: DateBCD,
    to: DateBCD,
    monday: bool,
    tuesday: bool,
    wednesday: bool,
    thursday: bool,
    friday: bool,
    saturday: bool,
    sunday: bool,
    segment1_start: TimeWithoutSecondsBCD,
    segment1_end: TimeWithoutSecondsBCD,
    segment2_start: TimeWithoutSecondsBCD,
    segment2_end: TimeWithoutSecondsBCD,
    segment3_start: TimeWithoutSecondsBCD,
    segment3_end: TimeWithoutSecondsBCD,
    linked_profile_id: u8,
}

#[allow(clippy::too_many_arguments)]
impl SetTimeProfileRequest {
    pub fn new(
        device_id: u32,
        profile_id: u8,
        from: DateBCD,
        to: DateBCD,
        monday: bool,
        tuesday: bool,
        wednesday: bool,
        thursday: bool,
        friday: bool,
        saturday: bool,
        sunday: bool,
        segment1_start: TimeWithoutSecondsBCD,
        segment1_end: TimeWithoutSecondsBCD,
        segment2_start: TimeWithoutSecondsBCD,
        segment2_end: TimeWithoutSecondsBCD,
        segment3_start: TimeWithoutSecondsBCD,
        segment3_end: TimeWithoutSecondsBCD,
        linked_profile_id: u8,
    ) -> Self {
        SetTimeProfileRequest {
            header: HEADER,
            message_type: RequestResponseType::SetTimeProfile.into(),
            _unused: 0,
            device_id,
            profile_id,
            from,
            to,
            monday,
            tuesday,
            wednesday,
            thursday,
            friday,
            saturday,
            sunday,
            segment1_start,
            segment1_end,
            segment2_start,
            segment2_end,
            segment3_start,
            segment3_end,
            linked_profile_id,
        }
    }
}

#[test]
fn set_time_profile_request_to_bytes() {
    let expected = [
        0x17, 0x88, 0x00, 0x00, 0x2d, 0x55, 0x39, 0x19, 0x04, 0x20, 0x21, 0x04, 0x01, 0x20, 0x21,
        0x12, 0x29, 0x01, 0x01, 0x00, 0x01, 0x00, 0x01, 0x01, 0x08, 0x30, 0x09, 0x45, 0x11, 0x35,
        0x13, 0x15, 0x14, 0x01, 0x17, 0x59, 0x13, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ];

    let r = SetTimeProfileRequest::new(
        423187757,
        4,
        DateBCD::new(2021, 4, 1),
        DateBCD::new(2021, 12, 29),
        true,
        true,
        false,
        true,
        false,
        true,
        true,
        TimeWithoutSecondsBCD::new(8, 30),
        TimeWithoutSecondsBCD::new(9, 45),
        TimeWithoutSecondsBCD::new(11, 35),
        TimeWithoutSecondsBCD::new(13, 15),
        TimeWithoutSecondsBCD::new(14, 1),
        TimeWithoutSecondsBCD::new(17, 59),
        19,
    );

    let actual = r.to_bytes();
    assert_eq!(expected, actual);
}

#[derive(Decode, Response, Debug)]
pub struct SetTimeProfileResponse {
    pub header: u8,
    pub message_type: u8,
    _unused: u16,
    pub device_id: u32,
    pub success: bool,
}

#[test]
fn set_time_profile_response_from_bytes() {
    let bytes: [u8; 64] = [
        0x17, 0x88, 0x00, 0x00, 0x2d, 0x55, 0x39, 0x19, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ];

    let r = SetTimeProfileResponse::from_bytes(&bytes).unwrap();
    assert_eq!(r.message_type, RequestResponseType::SetTimeProfile.into());
    assert_eq!(r.device_id, 423187757);
    assert!(r.success);
}
