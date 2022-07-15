use chrono::{NaiveDate, NaiveTime};

use super::event::Event;
use crate::messages::GetStatusResponse;
use anyhow::Result;

/// Status of a [`Device`]
#[derive(Debug)]
pub struct Status {
    pub device_id: u32,
    pub system_time: NaiveTime,
    pub system_date: NaiveDate,
    pub doors: Vec<bool>,
    pub buttons: Vec<bool>,
    pub relay_state: u8,
    pub input_state: u8,
    pub system_error: u8,
    pub special_info: u8,
    pub sequence_number: u32,
    pub last_event: Option<Event>,
}

impl TryFrom<GetStatusResponse> for Status {
    type Error = anyhow::Error;
    fn try_from(response: GetStatusResponse) -> Result<Self> {
        let event = match response.event_index {
            0 => None,
            x => Some(Event {
                index: x,
                event_type: response.event_type.into(),
                granted: response.granted,
                door: response.door,
                direction: response.direction.into(),
                card_number: response.card_number,
                timestamp: response.timestamp.try_into()?,
                reason: response.reason.into(),
            }),
        };

        Ok(Status {
            device_id: response.device_id,
            system_time: response.system_time.try_into()?,
            system_date: response.system_date.try_into()?,
            doors: vec![
                response.door1_state,
                response.door2_state,
                response.door3_state,
                response.door4_state,
            ],
            buttons: vec![
                response.door1_button,
                response.door2_button,
                response.door3_button,
                response.door4_button,
            ],
            relay_state: response.relay_state,
            input_state: response.input_state,
            system_error: response.system_error,
            special_info: response.special_info,
            sequence_number: response.sequence_id,
            last_event: event,
        })
    }
}
