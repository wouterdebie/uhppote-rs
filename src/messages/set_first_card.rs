use super::{utils::types::TimeWithoutSecondsBCD, Request, RequestResponseType, Response, HEADER};
use bincode::{Decode, Encode};

#[derive(Encode, Request)]
pub struct SetFirstCardRequest {
    header: u8,
    message_type: u8,
    _unused: u16,
    device_id: u32,
    door: u8,
    start: TimeWithoutSecondsBCD,
    start_door_control: u8,
    end: TimeWithoutSecondsBCD,
    end_door_control: u8,
    monday: bool,
    tuesday: bool,
    wednesday: bool,
    thursday: bool,
    friday: bool,
    saturday: bool,
    sunday: bool,
}

#[allow(clippy::too_many_arguments)]
impl SetFirstCardRequest {
    pub fn new(
        device_id: u32,
        door: u8,
        start: TimeWithoutSecondsBCD,
        start_door_control: u8,
        end: TimeWithoutSecondsBCD,
        end_door_control: u8,
        monday: bool,
        tuesday: bool,
        wednesday: bool,
        thursday: bool,
        friday: bool,
        saturday: bool,
        sunday: bool,
    ) -> Self {
        SetFirstCardRequest {
            header: HEADER,
            message_type: RequestResponseType::SetFirstCard.into(),
            _unused: 0,
            device_id,
            door,
            start,
            start_door_control,
            end,
            end_door_control,
            monday,
            tuesday,
            wednesday,
            thursday,
            friday,
            saturday,
            sunday,
        }
    }
}

#[derive(Decode, Response, Debug)]
pub struct SetFirstCardResponse {
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
    fn set_listener_request_to_bytes() {
        let expected = [
            0x17, 0xaa, 0x00, 0x00, 0x2d, 0x55, 0x39, 0x19, 0x03, 0x08, 0x30, 0x01, 0x17, 0x45,
            0x02, 0x01, 0x01, 0x00, 0x01, 0x00, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        let r = SetFirstCardRequest::new(
            423187757,
            3,
            TimeWithoutSecondsBCD::new(8, 30),
            1,
            TimeWithoutSecondsBCD::new(17, 45),
            2,
            true,
            true,
            false,
            true,
            false,
            true,
            true,
        );

        let actual = r.to_bytes();
        assert_eq!(expected, actual);
    }

    #[test]
    fn set_listener_response_from_bytes() {
        let bytes: [u8; 64] = [
            0x17, 0xaa, 0x00, 0x00, 0x2d, 0x55, 0x39, 0x19, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        let r = SetFirstCardResponse::from_bytes(&bytes).unwrap();
        assert_eq!(r.message_type, RequestResponseType::SetFirstCard.into());
        assert!(r.success);
    }
}
