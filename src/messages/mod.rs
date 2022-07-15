mod add_task;
mod clear_task_list;
mod clear_time_profiles;
mod delete_card;
mod delete_cards;
mod get_card_by_id;
mod get_card_by_index;
mod get_cards;
mod get_config;
mod get_door_control_state;
mod get_event;
mod get_event_index;
mod get_listener;
mod get_status;
mod get_time;
mod get_time_profile;
mod open_door;
mod put_card;
mod refresh_task_list;
mod set_address;
mod set_door_control_state;
mod set_event_index;
// mod set_first_card;
mod set_listener;
mod set_record_special_events;
mod set_time;
mod set_time_profile;
mod utils;

pub use self::utils::request::Request;
pub use self::utils::request::Response;
pub use add_task::*;
use anyhow::bail;
pub use clear_task_list::*;
pub use clear_time_profiles::*;
pub use delete_card::*;
pub use delete_cards::*;
pub use get_card_by_id::*;
pub use get_card_by_index::*;
pub use get_cards::*;
pub use get_config::*;
pub use get_door_control_state::*;
pub use get_event::*;
pub use get_event_index::*;
pub use get_listener::*;
pub use get_status::*;
pub use get_time::*;
pub use get_time_profile::*;
pub use open_door::*;
pub use put_card::*;
pub use refresh_task_list::*;
pub use set_address::*;
pub use set_door_control_state::*;
pub use set_event_index::*;
// pub use set_first_card::*;
pub use set_listener::*;
pub use set_record_special_events::*;
pub use set_time::*;
pub use set_time_profile::*;
pub use utils::*;

use uhppote_derive::Request;
use uhppote_derive::Response;

#[derive(Debug)]
pub enum RequestResponseType {
    Status = 0x20,
    SetTime = 0x30,
    GetTime = 0x32,
    OpenDoor = 0x40,
    PutCard = 0x50,
    DeleteCard = 0x52,
    DeleteCards = 0x54,
    GetCards = 0x58,
    GetCardByID = 0x5a,
    GetCardByIndex = 0x5c,
    SetDoorControlState = 0x80,
    GetDoorControlState = 0x82,
    SetTimeProfile = 0x88,
    SetListener = 0x90,
    GetListener = 0x92,
    GetConfig = 0x94,
    SetAddress = 0x96,
    GetTimeProfile = 0x98,
    ClearTaskList = 0xa6,
    AddTask = 0xa8,
    // SetFirstCard = 0xaa,
    RefreshTaskList = 0xac,
    GetEvent = 0xb0,
    SetEventIndex = 0xb2,
    GetEventIndex = 0xb4,
    ClearTimeProfiles = 0x8a,
    SetRecordSpecialEvents = 0x8e,
}

impl From<RequestResponseType> for u8 {
    fn from(t: RequestResponseType) -> Self {
        t as u8
    }
}

impl TryFrom<u8> for RequestResponseType {
    type Error = anyhow::Error;
    fn try_from(value: u8) -> anyhow::Result<Self> {
        match value {
            0x20 => Ok(RequestResponseType::Status),
            0x30 => Ok(RequestResponseType::SetTime),
            0x32 => Ok(RequestResponseType::GetTime),
            0x40 => Ok(RequestResponseType::OpenDoor),
            0x50 => Ok(RequestResponseType::PutCard),
            0x52 => Ok(RequestResponseType::DeleteCard),
            0x54 => Ok(RequestResponseType::DeleteCards),
            0x58 => Ok(RequestResponseType::GetCards),
            0x5a => Ok(RequestResponseType::GetCardByID),
            0x5c => Ok(RequestResponseType::GetCardByIndex),
            0x80 => Ok(RequestResponseType::SetDoorControlState),
            0x82 => Ok(RequestResponseType::GetDoorControlState),
            0x88 => Ok(RequestResponseType::SetTimeProfile),
            0x90 => Ok(RequestResponseType::SetListener),
            0x92 => Ok(RequestResponseType::GetListener),
            0x94 => Ok(RequestResponseType::GetConfig),
            0x96 => Ok(RequestResponseType::SetAddress),
            0x98 => Ok(RequestResponseType::GetTimeProfile),
            0xa6 => Ok(RequestResponseType::ClearTaskList),
            0xa8 => Ok(RequestResponseType::AddTask),
            // 0xaa => Ok(RequestResponseType::SetFirstCard),
            0xac => Ok(RequestResponseType::RefreshTaskList),
            0xb0 => Ok(RequestResponseType::GetEvent),
            0xb2 => Ok(RequestResponseType::SetEventIndex),
            0xb4 => Ok(RequestResponseType::GetEventIndex),
            0x8a => Ok(RequestResponseType::ClearTimeProfiles),
            0x8e => Ok(RequestResponseType::SetRecordSpecialEvents),
            _ => bail!("Invalid request type: {:x}", value),
        }
    }
}

pub const HEADER: u8 = 0x17;
