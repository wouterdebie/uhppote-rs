use super::{Request, RequestResponseType, Response, HEADER};
use bincode::{Decode, Encode};

#[derive(Encode, Request)]
pub struct SetDoorControlStateRequest {
    header: u8,
    message_type: u8,
    _unused: u16,
    device_id: u32,
    door: u8,
    control_state: u8,
    delay: u8,
}

impl SetDoorControlStateRequest {
    pub fn new(device_id: u32, door: u8, control_state: u8, delay: u8) -> Self {
        SetDoorControlStateRequest {
            header: HEADER,
            message_type: RequestResponseType::SetDoorControlState.into(),
            _unused: 0,
            device_id,
            door,
            control_state,
            delay,
        }
    }
}

#[derive(Decode, Response, Debug)]
pub struct SetDoorControlStateResponse {
    pub header: u8,
    pub message_type: u8,
    _unused: u16,
    pub device_id: u32,
    pub door: u8,
    pub control_state: u8,
    pub delay: u8,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn set_door_control_state_request_to_bytes() {
        let expected = [
            0x17, 0x80, 0x00, 0x00, 0x2d, 0x55, 0x39, 0x19, 0x04, 0x02, 0x05, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        let r = SetDoorControlStateRequest::new(423187757, 4, 0x02, 5);

        let actual = r.to_bytes();
        assert_eq!(expected, actual);
    }

    #[test]
    fn set_door_control_state_response_from_bytes() {
        let bytes: [u8; 64] = [
            0x17, 0x80, 0x00, 0x00, 0x2d, 0x55, 0x39, 0x19, 0x04, 0x02, 0x05, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        let r = SetDoorControlStateResponse::from_bytes(&bytes).unwrap();
        assert_eq!(
            r.message_type,
            RequestResponseType::SetDoorControlState.into()
        );
        assert_eq!(r.device_id, 423187757);
        assert_eq!(r.door, 4);
        assert_eq!(r.control_state, 2);
        assert_eq!(r.delay, 5);
    }
}
