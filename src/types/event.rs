use chrono::NaiveDateTime;

use super::direction::Direction;
use crate::messages::GetEventResponse;
use anyhow::Result;

/// Event that occurred on a [`Device`]
#[derive(Debug)]
pub struct Event {
    pub timestamp: NaiveDateTime,
    pub index: u32,
    pub event_type: EventType,
    pub granted: bool,
    pub door: u8,
    pub direction: Direction,
    pub card_number: u32,
    pub reason: EventReason,
}

impl TryFrom<GetEventResponse> for Event {
    type Error = anyhow::Error;
    fn try_from(response: GetEventResponse) -> Result<Self> {
        Ok(Event {
            timestamp: response.timestamp.try_into()?,
            index: response.index,
            event_type: response.type_.into(),
            granted: response.granted,
            door: response.door,
            direction: response.direction.into(),
            card_number: response.card_number,
            reason: response.reason.into(),
        })
    }
}

#[derive(Debug)]
pub enum EventType {
    None = 0,
    Swipe = 1,
    Door = 2,
    Alarm = 3,
    Overwritten = 255,
}

impl From<u8> for EventType {
    fn from(event_type: u8) -> EventType {
        match event_type {
            0 => EventType::None,
            1 => EventType::Swipe,
            2 => EventType::Door,
            3 => EventType::Alarm,
            255 => EventType::Overwritten,
            _ => EventType::None,
        }
    }
}

#[derive(Debug)]
pub enum EventReason {
    None = 0,
    Swipe = 1,
    Denied = 5,
    NoAccessRights = 6,
    IncorrectPassword = 7,
    AntiPassback = 8,
    MoreCards = 9,
    FirstCardOpen = 10,
    DoorIsNormallyClosed = 11,
    Interlock = 12,
    NotInAllowedTimePeriod = 13,
    InvalidTimeZone = 15,
    AccessDenied = 18,
    PushButtonOk = 20,
    DoorOpen = 23,
    DoorClosed = 24,
    DoorOpenedSupervisorPassword = 25,
    ControllerPowerOn = 28,
    ControllerReset = 29,
    PushbuttonInvalidDoorLocked = 31,
    PushbuttonInvalidDoorOffline = 32,
    PushbuttonInvalidDoorInterlock = 33,
    PushbuttonInvalidDoorThreat = 34,
    DoorOpenTooLong = 37,
    ForcedOpen = 38,
    Fire = 39,
    ForcedClosed = 40,
    TheftPrevention = 41,
    TwentyFourSevenZone = 42,
    Emergency = 43,
    RemoteOpenDoor = 44,
    RemoteOpenDoorUsbReader = 45,
}

impl From<u8> for EventReason {
    fn from(reason: u8) -> EventReason {
        match reason {
            0 => EventReason::None,
            1 => EventReason::Swipe,
            5 => EventReason::Denied,
            6 => EventReason::NoAccessRights,
            7 => EventReason::IncorrectPassword,
            8 => EventReason::AntiPassback,
            9 => EventReason::MoreCards,
            10 => EventReason::FirstCardOpen,
            11 => EventReason::DoorIsNormallyClosed,
            12 => EventReason::Interlock,
            13 => EventReason::NotInAllowedTimePeriod,
            15 => EventReason::InvalidTimeZone,
            18 => EventReason::AccessDenied,
            20 => EventReason::PushButtonOk,
            23 => EventReason::DoorOpen,
            24 => EventReason::DoorClosed,
            25 => EventReason::DoorOpenedSupervisorPassword,
            28 => EventReason::ControllerPowerOn,
            29 => EventReason::ControllerReset,
            31 => EventReason::PushbuttonInvalidDoorLocked,
            32 => EventReason::PushbuttonInvalidDoorOffline,
            33 => EventReason::PushbuttonInvalidDoorInterlock,
            34 => EventReason::PushbuttonInvalidDoorThreat,
            37 => EventReason::DoorOpenTooLong,
            38 => EventReason::ForcedOpen,
            39 => EventReason::Fire,
            40 => EventReason::ForcedClosed,
            41 => EventReason::TheftPrevention,
            42 => EventReason::TwentyFourSevenZone,
            43 => EventReason::Emergency,
            44 => EventReason::RemoteOpenDoor,
            45 => EventReason::RemoteOpenDoorUsbReader,
            _ => EventReason::None,
        }
    }
}
