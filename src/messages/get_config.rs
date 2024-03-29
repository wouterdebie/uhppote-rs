use super::{
    utils::types::{MacAddress, Version},
    Request, RequestResponseType, Response, HEADER,
};
use crate::messages::utils::types::DateBCD;
use bincode::{Decode, Encode};
use std::net::Ipv4Addr;

#[derive(Encode, Request)]
pub struct GetConfigRequest {
    header: u8,
    message_type: u8,
    _unused: u16,
    device_id: u32,
}

impl GetConfigRequest {
    pub fn new(device_id: u32) -> Self {
        GetConfigRequest {
            header: HEADER,
            message_type: RequestResponseType::GetConfig.into(),
            _unused: 0,
            device_id,
        }
    }
}

#[derive(Decode, Response, Debug)]
pub struct GetConfigResponse {
    pub header: u8,
    pub message_type: u8,
    _unused: u16,
    pub device_id: u32,
    pub ip_address: Ipv4Addr,
    pub subnet: Ipv4Addr,
    pub gateway: Ipv4Addr,
    pub mac: MacAddress,
    pub version: Version,
    pub date: DateBCD,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_config_request_to_bytes() {
        let expected = [
            0x17, 0x94, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        let r = GetConfigRequest::new(0);

        let actual = r.to_bytes();
        assert_eq!(expected, actual);
    }

    #[test]
    fn get_config_response_from_bytes() {
        let bytes: [u8; 64] = [
            0x17, 0x94, 0x00, 0x00, 0x2d, 0x55, 0x39, 0x19, 0xc0, 0xa8, 0x00, 0x00, 0xff, 0xff,
            0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x66, 0x19, 0x39, 0x55, 0x2d, 0x08, 0x92,
            0x20, 0x18, 0x08, 0x16, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        let r = GetConfigResponse::from_bytes(&bytes).unwrap();
        assert_eq!(r.message_type, RequestResponseType::GetConfig.into());
        assert_eq!(r.device_id, 423187757);
        assert_eq!(r.ip_address, Ipv4Addr::new(192, 168, 0, 0));
        assert_eq!(r.subnet, Ipv4Addr::new(255, 255, 255, 0));
        assert_eq!(r.gateway, Ipv4Addr::new(0, 0, 0, 0));
        assert_eq!(r.mac.to_string(), "00:66:19:39:55:2d".to_string());
        assert_eq!(r.version.to_string(), "8.146".to_string());
        assert_eq!(r.date.to_string(), "2018-08-16".to_string());
    }
}
