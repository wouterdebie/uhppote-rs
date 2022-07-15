use std::time::Duration;

use crate::messages::GetDoorControlStateResponse;
use crate::messages::SetDoorControlStateResponse;

#[derive(Debug)]
pub struct DoorControl {
    pub mode: DoorControlMode,
    pub delay: Duration,
}

impl From<GetDoorControlStateResponse> for DoorControl {
    fn from(response: GetDoorControlStateResponse) -> Self {
        DoorControl {
            mode: response.control_state.into(),
            delay: Duration::new(response.delay as u64, 0),
        }
    }
}

impl From<SetDoorControlStateResponse> for DoorControl {
    fn from(response: SetDoorControlStateResponse) -> Self {
        DoorControl {
            mode: response.control_state.into(),
            delay: Duration::new(response.delay as u64, 0),
        }
    }
}

#[derive(Debug)]
pub enum DoorControlMode {
    NormallyOpen = 1,
    NormallyClosed = 2,
    Controlled = 3,
    Unknown,
}

impl From<u8> for DoorControlMode {
    fn from(mode: u8) -> DoorControlMode {
        match mode {
            1 => DoorControlMode::NormallyOpen,
            2 => DoorControlMode::NormallyClosed,
            3 => DoorControlMode::Controlled,
            _ => DoorControlMode::Unknown,
        }
    }
}
