use std::net::Ipv4Addr;

use chrono::NaiveDate;

use crate::messages::GetConfigResponse;

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

impl TryFrom<GetConfigResponse> for DeviceConfig {
    type Error = anyhow::Error;

    fn try_from(response: GetConfigResponse) -> Result<Self, Self::Error> {
        Ok(DeviceConfig {
            id: response.device_id,
            address: response.ip_address,
            subnet: response.subnet,
            gateway: response.gateway,
            mac: response.mac.to_string(),
            version: response.version.to_string(),
            date: response.date.try_into()?,
        })
    }
}
