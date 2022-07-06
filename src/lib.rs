#![doc(html_root_url = "https://docs.rs/uhppote-rs/1/")]
//! uhppote-rs is a safe Rust library for access control systems based on the UHPPOTE UT0311-L0x
//! TCP/IP Wiegand access control boards. This library is based on
//! [uhppoted-dll](https://github.com/uhppoted/uhppoted-dll) that's part of the
//! [uhppoted](https://github.com/uhppoted/uhppoted) project.
//!
//! This library depends on the [uhppote-sys](https://docs.rs/uhppote-sys/1/) crate, which provides
//! FFI bindings to the `uhppoted-dll` library.
//!
//! Most interactions with the system happen through the [`Device`] type.
//!
//! Example:
//! ```no_run
//! use uhppote_rs::Uhppote;
//! let mut uhppoted = Uhppoted::default();
//! let mut device = uhppoted.get_device(423196779).unwrap();
//! let status = device.get_status().unwrap();
//! ```

use anyhow::bail;
use anyhow::Result;
use c_vec::CVec;
pub use chrono::NaiveDate;
pub use chrono::NaiveDateTime;
pub use chrono::NaiveTime;
use std::ffi::CStr;
use std::mem::MaybeUninit;
use std::net::Ipv4Addr;
use std::net::SocketAddr;
use std::os::raw::c_char;
use std::ptr::addr_of_mut;
use std::{ffi::CString, vec};

pub struct Uhppoted {
    u: uhppote_sys::UHPPOTE,
}

impl Uhppoted {
    /// Create a new Uhppote struct
    ///
    /// Example:
    /// ```no_run
    /// use uhppote_rs::Uhppoted;
    /// let mut uhppoted = UUhppoted::new(
    ///     "0.0.0.0:0".parse().unwrap(),
    ///     "255.255.255.255:60000".parse().unwrap(),
    ///     None,
    ///     5000,
    ///     Vec::new(),
    ///     false,
    /// )
    /// .unwrap()
    pub fn new(
        bind: SocketAddr,
        broadcast: SocketAddr,
        listen: Option<SocketAddr>,
        timeout: i32,
        controllers: Vec<Controller>,
        debug: bool,
    ) -> Result<Uhppoted> {
        let devices: Result<Vec<uhppote_sys::udevice>> = controllers
            .iter()
            .map(|c| {
                Ok(uhppote_sys::udevice {
                    id: c.id,
                    address: CString::new(c.address.to_string())?.into_raw(),
                })
            })
            .collect();

        let udevices = uhppote_sys::udevices {
            N: controllers.len() as u32,
            devices: devices?.as_mut_ptr(),
        };

        let l = match listen {
            Some(l) => l.to_string(),
            None => "".to_string(),
        };

        let u = uhppote_sys::UHPPOTE {
            bind: CString::new(bind.to_string())?.into_raw(),
            broadcast: CString::new(broadcast.to_string())?.into_raw(),
            listen: CString::new(l)?.into_raw(),
            timeout,
            debug,
            devices: Box::into_raw(Box::new(udevices)),
        };

        Ok(Uhppoted { u })
    }

    /// Get all device on the network. This uses the broadcast address to find devices and returns a list
    /// of identifiers.
    pub fn get_devices(&mut self) -> Result<Vec<u32>> {
        let mut allocated = 0;
        loop {
            allocated += 16;

            let list = vec![0; allocated].as_mut_ptr();
            let count = Box::into_raw(Box::new(allocated as i32));
            unsafe {
                let err = uhppote_sys::GetDevices(&mut self.u, count, list);
                if !err.is_null() {
                    bail!("GetDevice failed: {}", get_string(err)?);
                }
                if *count as usize <= allocated {
                    return Ok(CVec::new(list, *count as usize).as_ref().to_vec());
                }
            }
        }
    }

    pub fn get_device(&mut self, id: u32) -> Result<Device> {
        Ok(Device::new(&mut self.u, id))
    }
}

#[derive(Debug)]
pub struct Device<'a> {
    u: &'a mut uhppote_sys::UHPPOTE,
    id: u32,
}

impl<'a> Device<'a> {
    pub fn new(u: &'a mut uhppote_sys::UHPPOTE, id: u32) -> Device<'a> {
        Device { u, id }
    }

    /// Get a [`DeviceConfig`] for a device id.
    pub fn get_config(&mut self) -> Result<DeviceConfig> {
        let mut device: MaybeUninit<uhppote_sys::Device> = MaybeUninit::uninit();

        unsafe {
            let err = uhppote_sys::GetDevice(&mut *self.u, device.as_mut_ptr(), self.id);

            if !err.is_null() {
                bail!("GetDevice failed: {}", get_string(err)?);
            }

            let d = device.assume_init();

            Ok(DeviceConfig {
                id: d.ID,
                address: get_string(d.address)?.parse()?,
                subnet: get_string(d.subnet)?.parse()?,
                gateway: get_string(d.gateway)?.parse()?,
                mac: get_string(d.MAC)?,
                version: get_string(d.version)?,
                date: NaiveDate::parse_from_str(get_string(d.date)?.as_str(), "%Y-%m-%d")?,
            })
        }
    }

    /// Set the IP address, subnet mask, and gateway of a device.
    pub fn set_address(&mut self, address: String, subnet: String, gateway: String) -> Result<()> {
        let address = CString::new(address)?.into_raw();
        let subnet = CString::new(subnet)?.into_raw();
        let gateway = CString::new(gateway)?.into_raw();
        unsafe {
            let err = uhppote_sys::SetAddress(&mut *self.u, self.id, address, subnet, gateway);
            if !err.is_null() {
                bail!("SetAddress failed: {}", get_string(err)?);
            }
            Ok(())
        }
    }

    /// Get the [`Status`] of a device.
    pub fn get_status(&mut self) -> Result<Status> {
        let mut status: MaybeUninit<uhppote_sys::Status> = MaybeUninit::uninit();
        let mut event: MaybeUninit<uhppote_sys::Event> = MaybeUninit::uninit();

        let mut doors = [0_u8; 4];
        let mut buttons = [0_u8; 4];

        let status_ptr = status.as_mut_ptr();
        let event_ptr = event.as_mut_ptr();
        unsafe {
            addr_of_mut!((*status_ptr).event).write(event_ptr);
            addr_of_mut!((*status.as_mut_ptr()).doors).write(doors.as_mut_ptr());
            addr_of_mut!((*status.as_mut_ptr()).buttons).write(buttons.as_mut_ptr());
            let err = uhppote_sys::GetStatus(&mut *self.u, status_ptr, self.id);

            if !err.is_null() {
                bail!("GetStatus failed: {}", get_string(err)?);
            }

            let s = status.assume_init();

            let doors = CVec::new(s.doors, 4)
                .as_ref()
                .iter()
                .map(|v| *v != 0)
                .collect::<Vec<bool>>();

            let buttons = CVec::new(s.buttons, 4)
                .as_ref()
                .iter()
                .map(|v| *v != 0)
                .collect::<Vec<bool>>();

            let event = s.event.as_ref().map(|e| Event {
                timestamp: NaiveDateTime::parse_from_str(
                    get_string(e.timestamp).unwrap().as_str(),
                    "%Y-%m-%d %H:%M:%S",
                )
                .unwrap(),
                index: e.index,
                event_type: e.eventType.into(),
                granted: e.granted != 0,
                door: e.door,
                direction: e.direction.into(),
                card: e.card,
                reason: e.reason.into(),
            });

            Ok(Status {
                id: s.ID,
                doors,
                buttons,
                relays: s.relays,
                inputs: s.inputs,
                syserror: s.syserror != 0,
                seqno: s.seqno,
                info: s.info != 0,
                sysdatetime: NaiveDateTime::parse_from_str(
                    get_string(s.sysdatetime)?.as_str(),
                    "%Y-%m-%d %H:%M:%S",
                )?,
                event,
            })
        }
    }

    /// Get the [`NaiveDateTime`] of the device.
    pub fn get_date_time(&mut self) -> Result<NaiveDateTime> {
        unsafe {
            let mut s: *mut c_char = std::ptr::null_mut();
            let err = uhppote_sys::GetTime(&mut *self.u, &mut s, self.id);
            if !err.is_null() {
                bail!("GetTime failed: {}", get_string(err)?);
            }
            let time = std::ffi::CStr::from_ptr(s).to_str()?;
            Ok(NaiveDateTime::parse_from_str(time, "%Y-%m-%d %H:%M:%S")?)
        }
    }

    /// Set the date and time of the device.
    pub fn set_date_time(&mut self, time: NaiveDateTime) -> Result<()> {
        let time = CString::new(time.format("%Y-%m-%d %H:%M:%S").to_string())?.into_raw();
        unsafe {
            let err = uhppote_sys::SetTime(&mut *self.u, self.id, time);
            if !err.is_null() {
                bail!("SetTime failed: {}", get_string(err)?);
            }
            Ok(())
        }
    }

    /// Return the IP address and port to which the selected controller sends events.
    pub fn get_listener(&mut self) -> Result<SocketAddr> {
        unsafe {
            let mut s: *mut c_char = std::ptr::null_mut();
            let err = uhppote_sys::GetListener(&mut *self.u, &mut s, self.id);
            if !err.is_null() {
                bail!("GetTime failed: {}", get_string(err)?);
            }
            let listener = std::ffi::CStr::from_ptr(s).to_str()?.to_string();
            Ok(listener.parse()?)
        }
    }

    // Set the IP address and port to which the selected controller sends events.
    pub fn set_listener(&mut self, listener: SocketAddr) -> Result<()> {
        let time = CString::new(listener.to_string())?.into_raw();
        unsafe {
            let err = uhppote_sys::SetListener(&mut *self.u, self.id, time);
            if !err.is_null() {
                bail!("SetListener failed: {}", get_string(err)?);
            }
            Ok(())
        }
    }

    /// Get the [`DoorControl`] ([`DoorControlMode`] and delay) of a door. Note that the first door id is 1, not 0.
    pub fn get_door_control(&mut self, door: u8) -> Result<DoorControl> {
        let mut door_control: MaybeUninit<uhppote_sys::DoorControl> = MaybeUninit::uninit();

        unsafe {
            let err =
                uhppote_sys::GetDoorControl(&mut *self.u, door_control.as_mut_ptr(), self.id, door);

            if !err.is_null() {
                bail!("GetDoorControl failed: {}", get_string(err)?);
            }

            let d = door_control.assume_init();

            Ok(DoorControl {
                mode: d.mode.into(),
                delay: d.delay,
            })
        }
    }

    /// Set the [`DoorControl`] ([`DoorControlMode`] and delay) of a door.Note that the first door id is 1, not 0.
    pub fn set_door_control(&mut self, door: u8, mode: DoorControlMode, delay: u8) -> Result<()> {
        unsafe {
            let err = uhppote_sys::SetDoorControl(&mut *self.u, self.id, door, mode as u8, delay);

            if !err.is_null() {
                bail!("SetDoorControl failed: {}", get_string(err)?);
            }
            Ok(())
        }
    }

    /// Open a door. Note that the first door id is 1, not 0.
    pub fn open_door(&mut self, door: u8) -> Result<()> {
        unsafe {
            let err = uhppote_sys::OpenDoor(&mut *self.u, self.id, door);

            if !err.is_null() {
                bail!("OpenDoor failed: {}", get_string(err)?);
            }
            Ok(())
        }
    }

    /// Get all the amount of cards in the system.
    pub fn get_cards(&mut self) -> Result<i32> {
        let n = Box::into_raw(Box::new(0));
        unsafe {
            let err = uhppote_sys::GetCards(&mut *self.u, n, self.id);
            if !err.is_null() {
                bail!("GetCards failed: {}", get_string(err)?);
            }
            Ok(*n)
        }
    }

    /// Get a specific [`Card`] by its index.
    pub fn get_card_by_index(&mut self, index: u32) -> Result<Card> {
        let mut card: MaybeUninit<uhppote_sys::Card> = MaybeUninit::uninit();
        let mut doors = [0_u8; 4];

        unsafe {
            addr_of_mut!((*card.as_mut_ptr()).doors).write(doors.as_mut_ptr());
            let err = uhppote_sys::GetCardByIndex(&mut *self.u, card.as_mut_ptr(), self.id, index);

            if !err.is_null() {
                bail!("GetCardByIndex failed: {}", get_string(err)?);
            }

            let c = card.assume_init();

            let doors = CVec::new(c.doors, 4)
                .as_ref()
                .iter()
                .map(|v| v.to_owned())
                .collect::<Vec<u8>>();

            Ok(Card {
                number: c.card_number,
                from: get_string(c.from)?,
                to: get_string(c.to)?,
                doors,
            })
        }
    }

    /// Get a [`Card`] by its card number.
    pub fn get_card(&mut self, card_number: u32) -> Result<Card> {
        let mut card: MaybeUninit<uhppote_sys::Card> = MaybeUninit::uninit();
        let mut doors = [0_u8; 4];

        unsafe {
            addr_of_mut!((*card.as_mut_ptr()).doors).write(doors.as_mut_ptr());
            let err = uhppote_sys::GetCard(&mut *self.u, card.as_mut_ptr(), self.id, card_number);

            if !err.is_null() {
                bail!("GetCardByIndex failed: {}", get_string(err)?);
            }

            let c = card.assume_init();

            let doors = CVec::new(c.doors, 4)
                .as_ref()
                .iter()
                .map(|v| v.to_owned())
                .collect::<Vec<u8>>();

            Ok(Card {
                number: c.card_number,
                from: get_string(c.from)?,
                to: get_string(c.to)?,
                doors,
            })
        }
    }

    /// Add a [`Card`] to the system.
    pub fn add_card(&mut self, mut card: Card) -> Result<()> {
        let card_number = card.number;
        let from = CString::new(card.from)?.into_raw();
        let to = CString::new(card.to)?.into_raw();
        let doors = card.doors.as_mut_ptr();
        unsafe {
            let err = uhppote_sys::PutCard(&mut *self.u, self.id, card_number, from, to, doors);
            if !err.is_null() {
                bail!("PutCard failed: {}", get_string(err)?);
            }
            Ok(())
        }
    }

    /// Remove a [`Card`] from the system.
    pub fn delete_card(&mut self, number: u32) -> Result<()> {
        unsafe {
            let err = uhppote_sys::DeleteCard(&mut *self.u, self.id, number);
            if !err.is_null() {
                bail!("DeleteCard failed: {}", get_string(err)?);
            }
            Ok(())
        }
    }

    /// Get the event index.
    pub fn get_event_index(&mut self) -> Result<u32> {
        let n = Box::into_raw(Box::new(0));
        unsafe {
            let err = uhppote_sys::GetEventIndex(&mut *self.u, n, self.id);
            if !err.is_null() {
                bail!("GetEventIndex failed: {}", get_string(err)?);
            }
            Ok(*n)
        }
    }

    /// Set the event index.
    pub fn set_event_index(&mut self, index: u32) -> Result<()> {
        unsafe {
            let err = uhppote_sys::SetEventIndex(&mut *self.u, self.id, index);
            if !err.is_null() {
                bail!("SetEventIndex failed: {}", get_string(err)?);
            }
            Ok(())
        }
    }

    /// Get an [`Event`] by its index.
    pub fn get_event(&mut self, index: u32) -> Result<Event> {
        let mut event: MaybeUninit<uhppote_sys::Event> = MaybeUninit::uninit();
        unsafe {
            let err = uhppote_sys::GetEvent(&mut *self.u, event.as_mut_ptr(), self.id, index);
            if !err.is_null() {
                bail!("GetEvent failed: {}", get_string(err)?);
            }
            let e = event.assume_init();
            Ok(Event {
                timestamp: NaiveDateTime::parse_from_str(
                    get_string(e.timestamp)?.as_str(),
                    "%Y-%m-%d %H:%M:%S",
                )?,
                index: e.index,
                event_type: e.eventType.into(),
                granted: e.granted != 0,
                door: e.door,
                direction: e.direction.into(),
                card: e.card,
                reason: e.reason.into(),
            })
        }
    }

    /// Enable/disable recording of special events
    pub fn record_special_events(&mut self, enabled: bool) -> Result<()> {
        unsafe {
            let err = uhppote_sys::RecordSpecialEvents(&mut *self.u, self.id, enabled as u8);
            if !err.is_null() {
                bail!("RecordSpecialEvents failed: {}", get_string(err)?);
            }
            Ok(())
        }
    }

    /// Get a [`TimeProfile`] by its index.
    pub fn get_time_profile(&mut self, id: u8) -> Result<TimeProfile> {
        let mut profile: MaybeUninit<uhppote_sys::TimeProfile> = MaybeUninit::uninit();
        unsafe {
            let err = uhppote_sys::GetTimeProfile(&mut *self.u, profile.as_mut_ptr(), self.id, id);
            if !err.is_null() {
                bail!("GetTimeProfile failed: {}", get_string(err)?);
            }
            let p = profile.assume_init();
            Ok(TimeProfile {
                id: p.ID,
                linked: p.linked,
                from: NaiveDate::parse_from_str(get_string(p.from)?.as_str(), "%Y-%m-%d")?,
                to: NaiveDate::parse_from_str(get_string(p.to)?.as_str(), "%Y-%m-%d")?,
                monday: p.monday != 0,
                tuesday: p.tuesday != 0,
                wednesday: p.wednesday != 0,
                thursday: p.thursday != 0,
                friday: p.friday != 0,
                saturday: p.saturday != 0,
                sunday: p.sunday != 0,
                segment1_start: NaiveTime::parse_from_str(
                    get_string(p.segment1start)?.as_str(),
                    "%H:%M:%S",
                )?,
                segment1_end: NaiveTime::parse_from_str(
                    get_string(p.segment1end)?.as_str(),
                    "%H:%M:%S",
                )?,
                segment2_start: NaiveTime::parse_from_str(
                    get_string(p.segment2start)?.as_str(),
                    "%H:%M:%S",
                )?,
                segment2_end: NaiveTime::parse_from_str(
                    get_string(p.segment2end)?.as_str(),
                    "%H:%M:%S",
                )?,
                segment3_start: NaiveTime::parse_from_str(
                    get_string(p.segment3start)?.as_str(),
                    "%H:%M:%S",
                )?,
                segment3_end: NaiveTime::parse_from_str(
                    get_string(p.segment3end)?.as_str(),
                    "%H:%M:%S",
                )?,
            })
        }
    }

    /// Set a [`TimeProfile`] by its index. Note that profile ID has to be larger than 1.
    pub fn set_time_profile(&mut self, profile: TimeProfile) -> Result<()> {
        if profile.id <= 1 {
            bail!("Invalid profile ID. Must be greater than 1");
        }

        let mut p = uhppote_sys::TimeProfile {
            ID: profile.id,
            linked: profile.linked,
            from: CString::new(profile.from.format("%Y-%m-%d").to_string())?.into_raw(),
            to: CString::new(profile.to.format("%Y-%m-%d").to_string())?.into_raw(),
            monday: profile.monday as u8,
            tuesday: profile.tuesday as u8,
            wednesday: profile.wednesday as u8,
            thursday: profile.thursday as u8,
            friday: profile.friday as u8,
            saturday: profile.saturday as u8,
            sunday: profile.sunday as u8,
            segment1start: CString::new(profile.segment1_start.format("%H:%M").to_string())?
                .into_raw(),
            segment1end: CString::new(profile.segment1_end.format("%H:%M").to_string())?.into_raw(),
            segment2start: CString::new(profile.segment2_start.format("%H:%M").to_string())?
                .into_raw(),
            segment2end: CString::new(profile.segment2_end.format("%H:%M").to_string())?.into_raw(),
            segment3start: CString::new(profile.segment3_start.format("%H:%M").to_string())?
                .into_raw(),
            segment3end: CString::new(profile.segment3_end.format("%H:%M").to_string())?.into_raw(),
        };
        unsafe {
            let err = uhppote_sys::SetTimeProfile(&mut *self.u, self.id, &mut p);
            if !err.is_null() {
                bail!("SetTimeProfile failed: {}", get_string(err)?);
            }
            Ok(())
        }
    }

    /// Clear all [`TimeProfile`]s.
    pub fn clear_time_profiles(&mut self) -> Result<()> {
        unsafe {
            let err = uhppote_sys::ClearTimeProfiles(&mut *self.u, self.id);
            if !err.is_null() {
                bail!("ClearTimeProfiles failed: {}", get_string(err)?);
            }
            Ok(())
        }
    }

    /// Add a [`Task`] to the system.
    pub fn add_task(&mut self, task: Task) -> Result<()> {
        let mut t = uhppote_sys::Task {
            task: task.task as u8,
            door: task.door,
            from: CString::new(task.from.format("%Y-%m-%d").to_string())?.into_raw(),
            to: CString::new(task.to.format("%Y-%m-%d").to_string())?.into_raw(),
            monday: task.monday as u8,
            tuesday: task.tuesday as u8,
            wednesday: task.wednesday as u8,
            thursday: task.thursday as u8,
            friday: task.friday as u8,
            saturday: task.saturday as u8,
            sunday: task.sunday as u8,
            at: CString::new(task.at.format("%Y-%m-%d").to_string())?.into_raw(),
            cards: task.cards,
        };

        unsafe {
            let err = uhppote_sys::AddTask(&mut *self.u, self.id, &mut t);
            if !err.is_null() {
                bail!("AddTask failed: {}", get_string(err)?);
            }
            Ok(())
        }
    }

    /// Refresh the task list.
    pub fn refresh_task_list(&mut self) -> Result<()> {
        unsafe {
            let err = uhppote_sys::RefreshTaskList(&mut *self.u, self.id);
            if !err.is_null() {
                bail!("RefreshTasklist failed: {}", get_string(err)?);
            }
            Ok(())
        }
    }

    /// Clear the task list
    pub fn clear_task_list(&mut self) -> Result<()> {
        unsafe {
            let err = uhppote_sys::ClearTaskList(&mut *self.u, self.id);
            if !err.is_null() {
                bail!("ClearTaskList failed: {}", get_string(err)?);
            }
            Ok(())
        }
    }
}

unsafe fn get_string(ptr: *mut i8) -> Result<String> {
    Ok(CStr::from_ptr(ptr).to_str()?.to_string())
}

impl Default for Uhppoted {
    /// Creates a default instance of [`Uhppoted`].
    /// Defaults:
    ///   -`bind`: 0.0.0.0:0
    ///   - `broadcast`: 255.255.255.255:60000
    ///   - `listen`: None
    ///   - `timeout`: 5000
    ///   - `controllers`: vec![]
    ///   - `debug`: false
    fn default() -> Uhppoted {
        Uhppoted::new(
            "0.0.0.0:0".parse().unwrap(),
            "255.255.255.255:60000".parse().unwrap(),
            None,
            5000,
            vec![],
            false,
        )
        .unwrap()
    }
}

/// Defines a specific controller that can be used by the [`Uhppoted`] instance.
pub struct Controller {
    id: u32,
    address: SocketAddr,
}

/// Configuration of a [`Device`]
#[derive(Debug)]
pub struct DeviceConfig {
    pub id: u32,
    pub address: Ipv4Addr,
    pub subnet: Ipv4Addr,
    pub gateway: Ipv4Addr,
    pub mac: String,
    pub version: String,
    pub date: NaiveDate,
}

/// Status of a [`Device`]
#[derive(Debug)]
pub struct Status {
    pub id: u32,
    pub sysdatetime: NaiveDateTime,
    pub doors: Vec<bool>,
    pub buttons: Vec<bool>,
    pub relays: u8,
    pub inputs: u8,
    pub syserror: bool,
    pub info: bool,
    pub seqno: u32,
    pub event: Option<Event>,
}

/// Event that occurred on a [`Device`]
#[derive(Debug)]
pub struct Event {
    pub timestamp: NaiveDateTime,
    pub index: u32,
    pub event_type: EventType,
    pub granted: bool,
    pub door: u8,
    pub direction: Direction,
    pub card: u32,
    pub reason: EventReason,
}

#[derive(Debug)]
pub struct DoorControl {
    pub mode: DoorControlMode,
    pub delay: u8,
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

#[derive(Debug)]
pub enum Direction {
    In = 1,
    Out = 2,
    Unknown,
}

impl From<u8> for Direction {
    fn from(direction: u8) -> Direction {
        match direction {
            1 => Direction::In,
            2 => Direction::Out,
            _ => Direction::Unknown,
        }
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

#[derive(Debug)]
pub struct Card {
    pub number: u32,
    pub from: String,
    pub to: String,
    pub doors: Vec<u8>,
}

#[derive(Debug)]
pub struct TimeProfile {
    pub id: u8,
    pub linked: u8,
    pub from: NaiveDate,
    pub to: NaiveDate,
    pub monday: bool,
    pub tuesday: bool,
    pub wednesday: bool,
    pub thursday: bool,
    pub friday: bool,
    pub saturday: bool,
    pub sunday: bool,
    pub segment1_start: NaiveTime,
    pub segment1_end: NaiveTime,
    pub segment2_start: NaiveTime,
    pub segment2_end: NaiveTime,
    pub segment3_start: NaiveTime,
    pub segment3_end: NaiveTime,
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
    pub cards: u8,
}
