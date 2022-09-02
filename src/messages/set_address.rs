use super::{Request, RequestResponseType, HEADER};
use bincode::Encode;
use std::net::Ipv4Addr;

#[derive(Encode, Request)]
pub struct SetAddressRequest {
    header: u8,
    message_type: u8,
    _unused: u16,
    device_id: u32,
    ip_address: Ipv4Addr,
    subnet: Ipv4Addr,
    gateway: Ipv4Addr,
    magic_word: u32,
}

impl SetAddressRequest {
    pub fn new(
        device_id: u32,
        ip_address: Ipv4Addr,
        subnet: Ipv4Addr,
        gateway: Ipv4Addr,
        magic_word: u32,
    ) -> Self {
        Self {
            header: HEADER,
            message_type: RequestResponseType::SetAddress.into(),
            _unused: 0,
            device_id,
            ip_address,
            subnet,
            gateway,
            magic_word,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn set_address_request_to_bytes() {
        let expected = [
            0x17, 0x96, 0x00, 0x00, 0x2d, 0x55, 0x39, 0x19, 0xc0, 0xa8, 0x01, 0x7d, 0xff, 0xff,
            0xff, 0x00, 0xc0, 0xa8, 0x01, 0x00, 0x55, 0xaa, 0xaa, 0x55, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        let r = SetAddressRequest::new(
            423187757,
            Ipv4Addr::new(192, 168, 1, 125),
            Ipv4Addr::new(255, 255, 255, 0),
            Ipv4Addr::new(192, 168, 1, 0),
            0x55_aa_aa_55,
        );

        let actual = r.to_bytes();
        assert_eq!(expected, actual);
    }
}
