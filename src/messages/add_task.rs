use super::{Request, RequestResponseType, Response, HEADER};
use crate::messages::utils::types::{DateBCD, TimeWithoutSecondsBCD};
use bincode::{Decode, Encode};

#[derive(Encode, Request)]
pub struct AddTaskRequest {
    header: u8,
    message_type: u8,
    _unused: u16,
    device_id: u32,
    from: DateBCD,
    to: DateBCD,
    monday: bool,
    tuesday: bool,
    wednesday: bool,
    thursday: bool,
    friday: bool,
    saturday: bool,
    sunday: bool,
    at: TimeWithoutSecondsBCD,
    door: u8,
    task: u8,
    more_cards: u8,
}

#[allow(clippy::too_many_arguments)]
impl AddTaskRequest {
    pub fn new(
        device_id: u32,
        from: DateBCD,
        to: DateBCD,
        monday: bool,
        tuesday: bool,
        wednesday: bool,
        thursday: bool,
        friday: bool,
        saturday: bool,
        sunday: bool,
        at: TimeWithoutSecondsBCD,
        door: u8,
        task: u8,
        more_cards: u8,
    ) -> Self {
        AddTaskRequest {
            header: HEADER,
            message_type: RequestResponseType::AddTask.into(),
            _unused: 0,
            device_id,
            from,
            to,
            monday,
            tuesday,
            wednesday,
            thursday,
            friday,
            saturday,
            sunday,
            at,
            door,
            task,
            more_cards,
        }
    }
}

#[derive(Decode, Response, Debug)]
pub struct AddTaskResponse {
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
    fn add_task_request_to_bytes() {
        let expected = [
            0x17, // Preamble
            0xa8, // Message type
            0x00, 0x00, // Unused
            0x2d, 0x55, 0x39, 0x19, // Device ID
            0x20, 0x21, 0x04, 0x01, // From
            0x20, 0x21, 0x12, 0x29, // To
            0x01, 0x01, 0x00, 0x01, 0x00, 0x01, 0x01, 0x08, 0x30, 0x03, 0x04, 0x07, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        let r = AddTaskRequest::new(
            423187757,
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
            3,
            4,
            7,
        );

        let actual = r.to_bytes();
        assert_eq!(expected, actual);
    }

    #[test]
    fn add_task_response_from_bytes() {
        let bytes: [u8; 64] = [
            0x17, 0xa8, 0x00, 0x00, 0x2d, 0x55, 0x39, 0x19, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        let r = AddTaskResponse::from_bytes(&bytes).unwrap();
        assert_eq!(r.message_type, RequestResponseType::AddTask.into());
        assert_eq!(r.device_id, 423187757);
        assert!(r.success);
    }
}
