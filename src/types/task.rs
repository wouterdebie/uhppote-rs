use chrono::{NaiveDate, NaiveTime};

#[derive(Debug)]
pub struct Task {
    pub task: TaskID,
    pub door: u8,
    pub from: NaiveDate,
    pub to: NaiveDate,
    pub monday: bool,
    pub tuesday: bool,
    pub wednesday: bool,
    pub thursday: bool,
    pub friday: bool,
    pub saturday: bool,
    pub sunday: bool,
    pub at: NaiveTime,
    pub more_cards: u8,
}

#[derive(Debug)]
#[repr(u8)]
pub enum TaskID {
    ControlDoor = 1,
    UnlockDoor = 2,
    LockDoor = 3,
    DisableTimeProfile = 4,
    EnableTimeProfile = 5,
    EnableCardNoPassword = 6,
    EnableCardWithInPassword = 7,
    EnableCardWithPassword = 8,
    EnableMoreCards = 9,
    DisableMoreCards = 10,
    TriggerOnce = 11,
    DisablePushButton = 12,
    EnablePushButton = 13,
}

impl From<u8> for TaskID {
    fn from(task: u8) -> TaskID {
        match task {
            1 => TaskID::ControlDoor,
            2 => TaskID::UnlockDoor,
            3 => TaskID::LockDoor,
            4 => TaskID::DisableTimeProfile,
            5 => TaskID::EnableTimeProfile,
            6 => TaskID::EnableCardNoPassword,
            7 => TaskID::EnableCardWithInPassword,
            8 => TaskID::EnableCardWithPassword,
            9 => TaskID::EnableMoreCards,
            10 => TaskID::DisableMoreCards,
            11 => TaskID::TriggerOnce,
            12 => TaskID::DisablePushButton,
            13 => TaskID::EnablePushButton,
            _ => TaskID::ControlDoor,
        }
    }
}
