#![doc(html_root_url = "https://docs.rs/uhppote-rs/1/")]
//! uhppote-rs is a safe Rust library for access control systems based on the UHPPOTE UT0311-L0x
//! TCP/IP Wiegand access control boards. This library is based on the
//! [uhppoted](https://github.com/uhppoted/uhppoted) project.
//!
//! Most interactions with the system happen through the [`Device`] type.
//!
//! Example:
//! ```no_run
//! use uhppote_rs::Uhppote;
//! let uhppoted = Uhppoted::default();
//! let device = uhppoted.get_device(423196779).unwrap();
//! let status = device.get_status().unwrap();
//! ```
mod messages;
mod types;
use anyhow::bail;
use anyhow::Result;
use chrono::Datelike;
pub use chrono::NaiveDate;
pub use chrono::NaiveDateTime;
pub use chrono::NaiveTime;
use messages::types::DateBCD;
use messages::*;
use std::fmt::Debug;
use std::net::Ipv4Addr;
use std::net::SocketAddr;
use std::net::UdpSocket;
use std::time::Duration;
pub use types::*;

const UHPPOTE_PORT: u16 = 60000;
#[derive(Debug)]
pub struct Uhppoted {
    bind_address: SocketAddr,
    broadcast_address: Ipv4Addr,
    timeout: Duration,
}

impl Uhppoted {
    /// Create a new Uhppote struct
    ///
    /// Example:
    /// ```no_run
    /// use uhppote_rs::Uhppoted;
    /// let uhppoted = UUhppoted::new(
    ///     "0.0.0.0:0".parse().unwrap(),
    ///     "255.255.255.255".parse().unwrap(),
    ///     Duration::new(5, 0),
    ///     Vec::new(),
    ///     false,
    /// )
    pub fn new(bind: SocketAddr, broadcast: Ipv4Addr, timeout: Duration) -> Uhppoted {
        Uhppoted {
            bind_address: bind,
            broadcast_address: broadcast,
            timeout,
        }
    }

    /// Get all the available [`DeviceConfig`]s on the local network. This broadcasts a discovery message
    /// and waits [`Uhppoted::timeout`] for responses.
    pub fn get_device_configs(&self) -> Result<Vec<DeviceConfig>> {
        let request = GetConfigRequest::new(0);
        let response: Vec<GetConfigResponse> = broadcast_and_receive(request, self)?;
        let r = response
            .into_iter()
            .map(|r| r.try_into().unwrap())
            .collect();
        Ok(r)
    }

    /// Get all the available [`Device`]s on the local network. This broadcasts a discovery message
    /// and waits [`Uhppoted::timeout`] for responses.
    pub fn get_devices(&self) -> Result<Vec<Device>> {
        let request = GetConfigRequest::new(0);
        let response: Vec<GetConfigResponse> = broadcast_and_receive(request, self)?;
        let r = response
            .into_iter()
            .map(|r| Device::new(self, r.device_id, Some(r.ip_address)))
            .collect();
        Ok(r)
    }

    /// Get a [`Device`] by its device ID. This does not check if the device actually exists, but
    /// merely represents a device to interact with.
    ///
    /// When `ip_address` is specified, communication will happen directly with the device. Otherwise,
    /// communication to the device will happen via local network broadcast.
    ///
    /// Specify an `ip_address` when the device is not on the local network.
    pub fn get_device(&self, id: u32, ip_address: Option<Ipv4Addr>) -> Device {
        Device::new(self, id, ip_address)
    }

    /// Listen for incoming [`Status`] messages from the UHPPOTE system on a specific `address`.
    /// Example:
    /// ```no_run
    /// use uhppote_rs::Uhppoted;
    /// let uhppoted = Uhppoted::default();
    /// let device = uhppoted.get_device(423196779);
    ///
    /// let listener_address: SocketAddr = "192.168.0.10:12345".parse().unwrap();
    ///
    /// device.set_listener(listener_address).unwrap();
    /// uhppoted.listen(listener_address, |status| {
    ///     println!("{:?}", status);
    /// });
    /// ```
    pub fn listen(&self, address: SocketAddr, handler: fn(Status)) -> Result<()> {
        let socket = UdpSocket::bind(&address)?;
        socket.set_broadcast(true)?;
        socket.set_read_timeout(None)?;
        loop {
            let mut buf = [0u8; 64];
            socket.recv(&mut buf)?;
            match buf[1].try_into()? {
                RequestResponseType::Status => {
                    let response = GetStatusResponse::from_bytes(&buf)?;
                    handler(response.try_into()?);
                }
                response_type => bail!("Can't listen for {:?}", response_type),
            }
        }
    }
}

impl Default for Uhppoted {
    /// Creates a default instance of [`Uhppoted`].
    /// Defaults:
    ///   - `bind`: 0.0.0.0:0
    ///   - `broadcast`: 255.255.255.255
    ///   - `listen`: None
    ///   - `timeout`: Duration::new(5, 0)
    ///   - `controllers`: None
    fn default() -> Uhppoted {
        Uhppoted::new(
            "0.0.0.0:0".parse().unwrap(),
            "255.255.255.255".parse().unwrap(),
            Duration::new(5, 0),
        )
    }
}

#[derive(Debug)]
pub struct Device<'a> {
    u: &'a Uhppoted,
    id: u32,
    ip_address: Option<Ipv4Addr>,
}

impl<'a> Device<'a> {
    /// Create a new [`Device`] from an [`Uhppoted`] and a device ID.
    fn new(u: &'a Uhppoted, id: u32, ip_address: Option<Ipv4Addr>) -> Device<'a> {
        Device { u, id, ip_address }
    }

    /// Add a [`Card`] to the [`Device`].
    pub fn add_card(&self, card: Card) -> Result<()> {
        let request = PutCardRequest::new(
            self.id,
            card.number,
            card.from.try_into()?,
            card.to.try_into()?,
            card.doors[0],
            card.doors[1],
            card.doors[2],
            card.doors[3],
        );
        let response: PutCardResponse = send_and_receive(request, self)?;
        if response.success {
            Ok(())
        } else {
            bail!("PutCard failed")
        }
    }

    /// Add a [`Task`] to the system.
    pub fn add_task(&self, task: Task) -> Result<()> {
        let request = AddTaskRequest::new(
            self.id,
            DateBCD::new(
                task.from.year() as u16,
                task.from.month() as u8,
                task.from.day() as u8,
            ),
            DateBCD::new(
                task.to.year() as u16,
                task.to.month() as u8,
                task.to.day() as u8,
            ),
            task.monday,
            task.tuesday,
            task.wednesday,
            task.thursday,
            task.friday,
            task.saturday,
            task.sunday,
            task.at.try_into()?,
            task.door,
            task.task as u8,
            task.more_cards,
        );

        let response: AddTaskResponse = send_and_receive(request, self)?;

        if response.success {
            Ok(())
        } else {
            bail!("AddTask failed")
        }
    }

    /// Remove all [`Card`]s from the [`Device`].
    pub fn clear_cards(&self) -> Result<()> {
        let magic_word = 0x55aaaa55;
        let request = DeleteCardsRequest::new(self.id, magic_word);
        let response: DeleteCardsResponse = send_and_receive(request, self)?;
        if response.success {
            Ok(())
        } else {
            bail!("DeleteCard failed")
        }
    }

    /// Remove all [`Task`]s from the [`Device`].
    pub fn clear_tasks(&self) -> Result<()> {
        let request = ClearTaskListRequest::new(self.id, 0x55aaaa55);
        let response: ClearTaskListResponse = send_and_receive(request, self)?;
        if response.success {
            Ok(())
        } else {
            bail!("ClearTaskList failed")
        }
    }

    /// Remove all [`TimeProfile`]s from the [`Device`].
    pub fn clear_time_profiles(&self) -> Result<()> {
        let magic_word = 0x55aaaa55;
        let request = ClearTimeProfilesRequest::new(self.id, magic_word);
        let response: ClearTimeProfilesResponse = send_and_receive(request, self)?;
        if response.magic_word == magic_word {
            Ok(())
        } else {
            bail!("ClearTimeProfiles failed")
        }
    }

    /// Remove a [`Card`] from the [`Device`].
    pub fn delete_card(&self, number: u32) -> Result<()> {
        let request = DeleteCardRequest::new(self.id, number);
        let response: DeleteCardResponse = send_and_receive(request, self)?;
        if response.success {
            Ok(())
        } else {
            bail!("DeleteCard failed")
        }
    }

    /// Get a specific [`Card`] by its ID.
    pub fn get_card_by_id(&self, id: u32) -> Result<Card> {
        let request = GetCardByIDRequest::new(self.id, id);
        let response: GetCardByIDResponse = send_and_receive(request, self)?;
        response.try_into()
    }

    /// Get a specific [`Card`] by its index.
    pub fn get_card_by_index(&self, index: u32) -> Result<Card> {
        let request = GetCardByIndexRequest::new(self.id, index);
        let response: GetCardByIndexResponse = send_and_receive(request, self)?;
        response.try_into()
    }

    /// Get the number of [`Card`]s from the [`Device`].
    pub fn get_cards(&self) -> Result<u32> {
        let request = GetCardsRequest::new(self.id);
        let response: GetCardsResponse = send_and_receive(request, self)?;
        Ok(response.records)
    }

    /// Get a [`DeviceConfig`] for a the [`Device`].
    pub fn get_config(&self) -> Result<DeviceConfig> {
        let request = GetConfigRequest::new(self.id);
        let response: GetConfigResponse = send_and_receive(request, self)?;
        response.try_into()
    }

    /// Get a [`DoorControl`] for a specific door.
    /// Note that doors are addressed 1-4, not 0-3.
    pub fn get_door_control(&self, door: u8) -> Result<DoorControl> {
        let request = GetDoorControlStateRequest::new(self.id, door);
        let response: GetDoorControlStateResponse = send_and_receive(request, self)?;
        Ok(response.into())
    }

    /// Get an [`Event`] by its index.
    pub fn get_event(&self, index: u32) -> Result<Event> {
        let request = GetEventRequest::new(self.id, index);
        let response: GetEventResponse = send_and_receive(request, self)?;
        response.try_into()
    }

    /// Get the event index the [`Device`]
    pub fn get_event_index(&self) -> Result<u32> {
        let request = GetEventIndexRequest::new(self.id);
        let response: GetEventIndexResponse = send_and_receive(request, self)?;
        Ok(response.index)
    }

    /// Get what listener (IP:PORT) is set on the [`Device`]. This is where the the [`Device`]
    /// will send [`Status`] messages to over UDP.
    pub fn get_listener(&self) -> Result<SocketAddr> {
        let request = GetListenerRequest::new(self.id);
        let response: GetListenerResponse = send_and_receive(request, self)?;
        Ok(SocketAddr::from((response.ip_address, response.port)))
    }

    /// Get the [`Status`] of the [`Device`].
    pub fn get_status(&self) -> Result<Status> {
        let request = GetStatusRequest::new(self.id);
        let response: GetStatusResponse = send_and_receive(request, self)?;
        let status: Status = response.try_into()?;
        Ok(status)
    }

    /// Get the current time of the [`Device`].
    pub fn get_time(&self) -> Result<NaiveDateTime> {
        let request = GetTimeRequest::new(self.id);
        let response: GetTimeResponse = send_and_receive(request, self)?;
        response.datetime.try_into()
    }

    /// Get the [`TimeProfile`] by ID.
    pub fn get_time_profile(&self, profile_id: u8) -> Result<TimeProfile> {
        let request = GetTimeProfileRequest::new(self.id, profile_id);
        let response: GetTimeProfileResponse = send_and_receive(request, self)?;
        response.try_into()
    }

    /// Open a door.
    /// Note that doors are addressed 1-4, not 0-3.
    pub fn open_door(&self, door: u8) -> Result<()> {
        let request = OpenDoorRequest::new(self.id, door);
        let response: OpenDoorResponse = send_and_receive(request, self)?;
        if response.success {
            Ok(())
        } else {
            bail!("OpenDoor failed")
        }
    }

    /// Refresh the task list of the [`Device`].
    pub fn refresh_task_list(&self) -> Result<()> {
        let request = RefreshTaskListRequest::new(self.id, 0x55aaaa55);
        let response: RefreshTaskListResponse = send_and_receive(request, self)?;
        if response.success {
            Ok(())
        } else {
            bail!("RefreshTaskList failed")
        }
    }

    /// Set the [`DoorControl`] for a specific door.
    /// Note that the delay is in seconds and can maximally be 255.
    pub fn set_door_control_state(&self, door: u8, state: DoorControl) -> Result<DoorControl> {
        let request = SetDoorControlStateRequest::new(
            self.id,
            door,
            state.mode as u8,
            state.delay.as_secs() as u8,
        );
        let response: SetDoorControlStateResponse = send_and_receive(request, self)?;
        Ok(response.into())
    }

    /// Set the event index the [`Device`] will use.
    pub fn set_event_index(&self, index: u32) -> Result<()> {
        let request = SetEventIndexRequest::new(self.id, index, 0x55aaaa55);
        let response: SetEventIndexResponse = send_and_receive(request, self)?;
        if response.success {
            Ok(())
        } else {
            bail!("SetEventIndex failed")
        }
    }

    /// Set the listener (IP:PORT) the [`Device`] will use to send [`Status`] messages to over UDP.
    pub fn set_listener(&self, address: Ipv4Addr, port: u16) -> Result<()> {
        let request = SetListenerRequest::new(self.id, address, port);
        let response: SetListenerResponse = send_and_receive(request, self)?;
        if response.success {
            Ok(())
        } else {
            bail!("SetListener failed")
        }
    }

    /// Set IP address, subnet mask and gateway for the [`Device`].
    pub fn set_network_config(
        &self,
        address: Ipv4Addr,
        subnet: Ipv4Addr,
        gateway: Ipv4Addr,
    ) -> Result<()> {
        let request = SetAddressRequest::new(self.id, address, subnet, gateway, 0x55aaaa55);

        send(request, self)
    }

    /// Enable the recording of special events.
    pub fn enable_record_special_events(&self, enable: bool) -> Result<()> {
        let request = SetRecordSpecialEventsRequest::new(self.id, enable);
        let response: SetRecordSpecialEventsResponse = send_and_receive(request, self)?;
        if response.success {
            Ok(())
        } else {
            bail!("SetRecordSpecialEvents failed")
        }
    }

    /// Set the local time of the [`Device`].
    pub fn set_time(&self, datetime: NaiveDateTime) -> Result<NaiveDateTime> {
        let request = SetTimeRequest::new(self.id, datetime.try_into()?);
        let response: SetTimeResponse = send_and_receive(request, self)?;
        response.datetime.try_into()
    }

    /// Add or update new [`TimeProfile`] to the [`Device`].
    pub fn add_or_update_time_profile(&self, profile: TimeProfile) -> Result<()> {
        let request = SetTimeProfileRequest::new(
            self.id,
            profile.id,
            profile.from.try_into()?,
            profile.to.try_into()?,
            profile.monday,
            profile.tuesday,
            profile.wednesday,
            profile.thursday,
            profile.friday,
            profile.saturday,
            profile.sunday,
            profile.segments[0].start.try_into()?,
            profile.segments[0].end.try_into()?,
            profile.segments[1].start.try_into()?,
            profile.segments[1].end.try_into()?,
            profile.segments[2].start.try_into()?,
            profile.segments[2].end.try_into()?,
            profile.linked_profile_id,
        );
        let response: SetTimeProfileResponse = send_and_receive(request, self)?;
        if response.success {
            Ok(())
        } else {
            bail!("SetTimeProfile failed")
        }
    }
}

/// Send a [`Request`] and receive a [`Response`].
fn send_and_receive<T: messages::Request, S: messages::Response + Debug>(
    request: T,
    d: &Device,
) -> Result<S> {
    let socket = setup_socket(d.u)?;
    let addr = get_address(d);
    socket.send_to(
        &request.to_bytes(),
        &SocketAddr::new(addr.into(), UHPPOTE_PORT),
    )?;

    // Receive the response
    let mut buf = [0u8; 64];
    socket.recv(&mut buf)?;

    S::from_bytes(&buf)
}

/// Send a [`Request`] to the [`Device`], but don't expect a response.
fn send<T: messages::Request>(request: T, d: &Device) -> Result<()> {
    let socket = setup_socket(d.u)?;
    let addr = get_address(d);
    socket.send_to(
        &request.to_bytes(),
        &SocketAddr::new(addr.into(), UHPPOTE_PORT),
    )?;
    Ok(())
}

/// Get the IP address of the [`Device`]. If None, use the broadcast address from [`Uhppoted`]
fn get_address(d: &Device) -> Ipv4Addr {
    match d.ip_address {
        Some(ip) => ip,
        None => d.u.broadcast_address,
    }
}

/// Setup a socket with correct timeouts.
fn setup_socket(u: &Uhppoted) -> Result<UdpSocket, anyhow::Error> {
    let socket = UdpSocket::bind(u.bind_address)?;
    socket.set_write_timeout(Some(Duration::new(1, 0)))?;
    socket.set_read_timeout(Some(u.timeout))?;
    socket.set_broadcast(true)?;
    Ok(socket)
}

/// Broadcast a [`Request`] to all [`Device`]s.
fn broadcast_and_receive<T: messages::Request, S: messages::Response + Debug>(
    request: T,
    u: &Uhppoted,
) -> Result<Vec<S>> {
    let socket = setup_socket(u)?;

    let to_addr = SocketAddr::new(u.broadcast_address.into(), UHPPOTE_PORT);

    socket.send_to(&request.to_bytes(), to_addr)?;
    let mut buf = [0u8; 64];

    let mut ret = Vec::new();

    while let Ok((_, _)) = socket.recv_from(&mut buf) {
        ret.push(S::from_bytes(&buf)?);
    }

    Ok(ret)
}
