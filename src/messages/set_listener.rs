use super::{Request, RequestResponseType, Response, HEADER};
use bincode::{Decode, Encode};
use std::net::Ipv4Addr;

#[derive(Encode, Request)]
pub struct SetListenerRequest {
    header: u8,
    message_type: u8,
    _unused: u16,
    device_id: u32,
    ip_address: Ipv4Addr,
    port: u16,
}

impl SetListenerRequest {
    pub fn new(device_id: u32, ip_address: Ipv4Addr, port: u16) -> Self {
        Self {
            header: HEADER,
            message_type: RequestResponseType::SetListener.into(),
            _unused: 0,
            device_id,
            ip_address,
            port,
        }
    }
}

#[test]
fn set_listener_request_to_bytes() {
    let expected = [
        0x17, 0x90, 0x00, 0x00, 0x2D, 0x55, 0x39, 0x19, 0xc0, 0xa8, 0x01, 0x64, 0x40, 0x9c, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ];

    let r = SetListenerRequest::new(423187757, Ipv4Addr::new(192, 168, 1, 100), 40000);

    let actual = r.to_bytes();
    assert_eq!(expected, actual);
}

#[derive(Decode, Response, Debug)]
pub struct SetListenerResponse {
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
    fn set_listener_request_to_bytes() {
        let expected = [
            0x17, 0x90, 0x00, 0x00, 0x2D, 0x55, 0x39, 0x19, 0xc0, 0xa8, 0x01, 0x64, 0x40, 0x9c,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        let r = SetListenerRequest::new(423187757, Ipv4Addr::new(192, 168, 1, 100), 40000);

        let actual = r.to_bytes();
        assert_eq!(expected, actual);
    }

    #[test]
    fn set_listener_response_from_bytes() {
        let bytes: [u8; 64] = [
            0x17, 0x90, 0x00, 0x00, 0x2d, 0x55, 0x39, 0x19, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        let r = SetListenerResponse::from_bytes(&bytes).unwrap();
        assert_eq!(r.message_type, RequestResponseType::SetListener.into());
        assert!(r.success);
    }
}
