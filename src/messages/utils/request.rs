use std::fmt::Debug;

use anyhow::Result;
pub trait Request {
    fn to_bytes(&self) -> [u8; 64];
    fn to_bytes_impl(&self) -> [u8; 64]
    where
        Self: std::marker::Sized,
        Self: bincode::Encode,
    {
        let options = bincode::config::standard()
            .with_fixed_int_encoding()
            .with_little_endian();

        let mut result = [0u8; 64];
        bincode::encode_into_slice(self, &mut result, options).unwrap();

        result
    }
    fn get_id(&self) -> u32;
}

pub trait Response {
    fn from_bytes(bytes: &[u8; 64]) -> Result<Self>
    where
        Self: std::marker::Sized,
        Self: Debug;
    fn from_bytes_impl(bytes: &[u8; 64]) -> Result<Self>
    where
        Self: std::marker::Sized,
        Self: bincode::Decode,
        Self: Debug,
    {
        let options = bincode::config::standard()
            .with_fixed_int_encoding()
            .with_little_endian();

        match bincode::decode_from_slice(bytes, options) {
            Ok((res, _)) => Ok(res),
            Err(e) => Err(anyhow::Error::from(e)),
        }
    }
}
