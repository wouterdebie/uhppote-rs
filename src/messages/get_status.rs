use super::{Request, RequestResponseType, Response, HEADER};
use crate::messages::utils::types::{DateShortBCD, DateTime, TimeWithSecondsBCD};
use bincode::{Decode, Encode};

#[derive(Encode, Request)]
pub struct GetStatusRequest {
    header: u8,
    message_type: u8,
    _unused: u16,
    device_id: u32,
}

impl GetStatusRequest {
    pub fn new(device_id: u32) -> Self {
        Self {
            header: HEADER,
            message_type: RequestResponseType::Status.into(),
            _unused: 0,
            device_id,
        }
    }
}

#[test]
fn get_status_request_to_bytes() {
    let expected = [
        0x17, 0x20, 0x00, 0x00, 0x2d, 0x55, 0x39, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ];

    let r = GetStatusRequest::new(423187757);

    let actual = r.to_bytes();
    assert_eq!(expected, actual);
}

#[derive(Encode, Decode, Request, Response, Debug)]
pub struct GetStatusResponse {
    pub header: u8,
    pub message_type: u8,
    _unused: u16,
    pub device_id: u32,
    pub event_index: u32,
    pub event_type: u8,
    pub granted: bool,
    pub door: u8,
    pub direction: u8,
    pub card_number: u32,
    pub timestamp: DateTime,
    pub reason: u8,
    pub door1_state: bool,
    pub door2_state: bool,
    pub door3_state: bool,
    pub door4_state: bool,
    pub door1_button: bool,
    pub door2_button: bool,
    pub door3_button: bool,
    pub door4_button: bool,
    pub system_error: u8,
    pub system_time: TimeWithSecondsBCD,
    pub sequence_id: u32,
    _unused2: u32,
    pub special_info: u8,
    pub relay_state: u8, // bitmap (0=locked, 1=unlocked, 0000:all doors locked)
    pub input_state: u8, // bitmap (bit 0: force locked, bit 1: fire alarm)
    pub system_date: DateShortBCD,
}

#[allow(clippy::too_many_arguments)]
impl GetStatusResponse {
    pub fn new(
        device_id: u32,
        event_index: u32,
        event_type: u8,
        granted: bool,
        door: u8,
        direction: u8,
        card_number: u32,
        timestamp: DateTime,
        reason: u8,
        door1_state: bool,
        door2_state: bool,
        door3_state: bool,
        door4_state: bool,
        door1_button: bool,
        door2_button: bool,
        door3_button: bool,
        door4_button: bool,
        system_error: u8,
        system_time: TimeWithSecondsBCD,
        sequence_id: u32,
        special_info: u8,
        relay_state: u8,
        input_state: u8,
        system_date: DateShortBCD,
    ) -> Self {
        Self {
            header: HEADER,
            message_type: RequestResponseType::Status.into(),
            _unused: 0,
            device_id,
            event_index,
            event_type,
            granted,
            door,
            direction,
            card_number,
            timestamp,
            reason,
            door1_state,
            door2_state,
            door3_state,
            door4_state,
            door1_button,
            door2_button,
            door3_button,
            door4_button,
            system_error,
            system_time,
            sequence_id,
            _unused2: 0,
            special_info,
            relay_state,
            input_state,
            system_date,
        }
    }
}

#[test]
fn get_status_response_to_bytes() {
    let expected = [
        0x17, 0x20, 0x00, 0x00, 0x2d, 0x55, 0x39, 0x19, 0x39, 0x00, 0x00, 0x00, 0x02, 0x01, 0x03,
        0x01, 0xaa, 0xe8, 0x5d, 0x00, 0x20, 0x19, 0x04, 0x19, 0x17, 0x00, 0x09, 0x06, 0x01, 0x00,
        0x01, 0x01, 0x00, 0x00, 0x01, 0x01, 0x09, // Last: SystemError
        0x14, 0x37, 0x02, 0x11, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2b, 0x04, 0x01, 0x19,
        0x04, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];

    let r = GetStatusResponse::new(
        423187757,
        57,
        2,
        true,
        3,
        1,
        6154410,
        DateTime::new(2019, 4, 19, 17, 0, 9),
        6,
        true,
        false,
        true,
        true,
        false,
        false,
        true,
        true,
        9,
        TimeWithSecondsBCD::new(14, 37, 2),
        17,
        43,
        4,
        1,
        DateShortBCD::new(2019, 4, 20),
    );

    let actual = r.to_bytes();
    assert_eq!(expected, actual);
}

#[test]
fn get_status_response_from_bytes() {
    let bytes: [u8; 64] = [
        0x17, 0x20, 0x00, 0x00, 0x2d, 0x55, 0x39, 0x19, 0x39, 0x00, 0x00, 0x00, 0x01, 0x00, 0x03,
        0x01, 0xaa, 0xe8, 0x5d, 0x00, 0x20, 0x19, 0x04, 0x19, 0x17, 0x00, 0x09, 0x06, 0x01, 0x00,
        0x01, 0x01, 0x00, 0x00, 0x01, 0x01, 0x09, 0x14, 0x37, 0x02, 0x11, 0x00, 0x00, 0x00, 0x21,
        0x00, 0x00, 0x00, 0x2b, 0x04, 0x01, 0x19, 0x04, 0x20, 0x00, 0x00, 0x93, 0x26, 0x04, 0x88,
        0x08, 0x92, 0x00, 0x00,
    ];

    let r = GetStatusResponse::from_bytes(&bytes).unwrap();
    assert_eq!(r.message_type, RequestResponseType::Status.into());
    assert_eq!(r.device_id, 423187757);
    assert_eq!(r.event_index, 57);
    assert_eq!(r.event_type, 1);
    assert!(!r.granted);
    assert_eq!(r.door, 3);
    assert_eq!(r.direction, 1);
    assert_eq!(r.card_number, 6154410);
    assert_eq!(r.device_id, 423187757);
    assert_eq!(r.timestamp, DateTime::new(2019, 4, 19, 17, 0, 9));
    assert_eq!(r.reason, 6);
    assert!(r.door1_state);
    assert!(!r.door2_state);
    assert!(r.door3_state);
    assert!(r.door4_state);
    assert!(!r.door1_button);
    assert!(!r.door2_button);
    assert!(r.door3_button);
    assert!(r.door4_button);
    assert_eq!(r.system_error, 9);
    assert_eq!(r.system_time, TimeWithSecondsBCD::new(14, 37, 2));
    assert_eq!(r.system_date, DateShortBCD::new(2019, 4, 20));
    assert_eq!(r.sequence_id, 17);
    assert_eq!(r.special_info, 43);
    assert_eq!(r.relay_state, 4);
    assert_eq!(r.input_state, 1);
}
