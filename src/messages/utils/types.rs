use super::bcd;
use anyhow::Result;
use chrono::{Datelike, NaiveDate, NaiveDateTime, NaiveTime, Timelike};
use std::fmt::Display;

#[derive(bincode::Decode, PartialEq, Eq, Debug)]
pub struct MacAddress {
    pub addr: (u8, u8, u8, u8, u8, u8),
}

impl MacAddress {
    pub fn new(addr: (u8, u8, u8, u8, u8, u8)) -> Self {
        MacAddress { addr }
    }
}

impl Display for MacAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            self.addr.0, self.addr.1, self.addr.2, self.addr.3, self.addr.4, self.addr.5
        )
    }
}

#[derive(bincode::Decode, PartialEq, Eq, Debug)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

#[derive(bincode::Decode, bincode::Encode, PartialEq, Eq, Debug, Default)]
pub struct DateBCD {
    date: (u8, u8, u8, u8), // Y, Y, M, D
}

impl DateBCD {
    pub fn new(year: u16, month: u8, day: u8) -> Self {
        let bcd = bcd::encode(format!("{:02}{:02}{:02}", year, month, day));
        DateBCD {
            date: (bcd[0], bcd[1], bcd[2], bcd[3]),
        }
    }
}

impl TryInto<NaiveDate> for DateBCD {
    type Error = anyhow::Error;
    fn try_into(self) -> Result<NaiveDate> {
        Ok(NaiveDate::parse_from_str(&self.to_string(), "%Y-%m-%d")?)
    }
}

#[test]
fn test_date_bcd_into_naive_date() {
    let date = NaiveDate::from_ymd(2019, 1, 1);
    let bcd = DateBCD::new(2019, 1, 1);
    let converted: NaiveDate = bcd.try_into().unwrap();
    assert_eq!(converted, date);
}

impl TryFrom<NaiveDate> for DateBCD {
    type Error = anyhow::Error;
    fn try_from(date: NaiveDate) -> Result<Self> {
        Ok(DateBCD::new(
            date.year() as u16,
            date.month() as u8,
            date.day() as u8,
        ))
    }
}

#[test]
fn test_date_bcd_from_naive_date() {
    let date = NaiveDate::from_ymd(2019, 1, 1);
    let bcd = DateBCD::new(2019, 1, 1);
    let converted = DateBCD::try_from(date).unwrap();
    assert_eq!(converted, bcd);
}

impl Display for DateBCD {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{:02x}{:02x}-{:02x}-{:02x}",
            self.date.0, self.date.1, self.date.2, self.date.3
        )
    }
}

#[test]
fn test_date_bcd_display() {
    let date = DateBCD::new(2019, 1, 1);
    assert_eq!(date.to_string(), "2019-01-01");
}

// TimeWithoutSecondsBCD

#[derive(bincode::Decode, bincode::Encode, PartialEq, Eq, Debug, Default)]
pub struct TimeWithoutSecondsBCD {
    pub hour: u8,
    pub minute: u8,
}

impl TimeWithoutSecondsBCD {
    pub fn new(hour: u8, minute: u8) -> Self {
        TimeWithoutSecondsBCD {
            hour: bcd::encode(format!("{:02}", hour))[0],
            minute: bcd::encode(format!("{:02}", minute))[0],
        }
    }
}

impl TryFrom<NaiveTime> for TimeWithoutSecondsBCD {
    type Error = anyhow::Error;
    fn try_from(time: NaiveTime) -> Result<Self> {
        Ok(TimeWithoutSecondsBCD::new(
            time.hour() as u8,
            time.minute() as u8,
        ))
    }
}

#[test]
fn test_time_without_seconds_bcd_from_naive_time() {
    let time = NaiveTime::from_hms(8, 12, 0);
    let bcd = TimeWithoutSecondsBCD::new(8, 12);
    let converted = TimeWithoutSecondsBCD::try_from(time).unwrap();
    assert_eq!(converted, bcd);
}

impl TryInto<NaiveTime> for TimeWithoutSecondsBCD {
    type Error = anyhow::Error;
    fn try_into(self) -> Result<NaiveTime> {
        Ok(NaiveTime::parse_from_str(&self.to_string(), "%H:%M")?)
    }
}
#[test]
fn test_time_without_seconds_bcd_into_naive_time() {
    let time = NaiveTime::from_hms(8, 12, 0);
    let bcd = TimeWithoutSecondsBCD::new(8, 12);
    let converted: NaiveTime = bcd.try_into().unwrap();
    assert_eq!(converted, time);
}

impl Display for TimeWithoutSecondsBCD {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:02x}:{:02x}", self.hour, self.minute,)
    }
}

#[test]
fn test_time_without_seconds_bcd_display() {
    let time = TimeWithoutSecondsBCD::new(8, 12);
    assert_eq!(time.to_string(), "08:12");
}

// TimeWithSecondsBCD

#[derive(bincode::Decode, bincode::Encode, PartialEq, Eq, Debug, Default)]
pub struct TimeWithSecondsBCD {
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

impl TimeWithSecondsBCD {
    pub fn new(hour: u8, minute: u8, second: u8) -> Self {
        TimeWithSecondsBCD {
            hour: bcd::encode(format!("{:02}", hour))[0],
            minute: bcd::encode(format!("{:02}", minute))[0],
            second: bcd::encode(format!("{:02}", second))[0],
        }
    }
}

impl TryFrom<NaiveTime> for TimeWithSecondsBCD {
    type Error = anyhow::Error;
    fn try_from(time: NaiveTime) -> Result<Self> {
        Ok(TimeWithSecondsBCD::new(
            time.hour() as u8,
            time.minute() as u8,
            time.second() as u8,
        ))
    }
}

#[test]
fn test_time_with_seconds_bcd_from_naive_time() {
    let time = NaiveTime::from_hms(8, 12, 13);
    let bcd = TimeWithSecondsBCD::new(8, 12, 13);
    let converted = TimeWithSecondsBCD::try_from(time).unwrap();
    assert_eq!(converted, bcd);
}

impl TryInto<NaiveTime> for TimeWithSecondsBCD {
    type Error = anyhow::Error;
    fn try_into(self) -> Result<NaiveTime> {
        Ok(NaiveTime::parse_from_str(&self.to_string(), "%H:%M:%S")?)
    }
}

#[test]
fn test_time_with_seconds_bcd_into_naive_time() {
    let time = NaiveTime::from_hms(8, 12, 13);
    let bcd = TimeWithSecondsBCD::new(8, 12, 13);
    let converted: NaiveTime = bcd.try_into().unwrap();
    assert_eq!(converted, time);
}

impl Display for TimeWithSecondsBCD {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{:02x}:{:02x}:{:02x}",
            self.hour, self.minute, self.second
        )
    }
}

#[test]
fn test_time_with_seconds_bcd_display() {
    let time = TimeWithSecondsBCD::new(8, 12, 13);
    assert_eq!(time.to_string(), "08:12:13");
}

#[derive(bincode::Decode, bincode::Encode, PartialEq, Eq, Debug, Default)]
pub struct DateTime {
    pub date: DateBCD,
    pub time: TimeWithSecondsBCD,
}

impl DateTime {
    pub fn new(year: u16, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> Self {
        DateTime {
            date: DateBCD::new(year, month, day),
            time: TimeWithSecondsBCD::new(hour, minute, second),
        }
    }
}

impl TryFrom<NaiveDateTime> for DateTime {
    type Error = anyhow::Error;
    fn try_from(datetime: NaiveDateTime) -> Result<Self> {
        Ok(DateTime::new(
            datetime.year() as u16,
            datetime.month() as u8,
            datetime.day() as u8,
            datetime.hour() as u8,
            datetime.minute() as u8,
            datetime.second() as u8,
        ))
    }
}

#[test]
fn test_date_time_bcd_from_naive_time() {
    let date_time =
        NaiveDateTime::parse_from_str("2019-08-01 08:31:22", "%Y-%m-%d %H:%M:%S").unwrap();
    let bcd = DateTime::new(2019, 8, 1, 8, 31, 22);
    let converted = DateTime::try_from(date_time).unwrap();
    assert_eq!(converted, bcd);
}

impl TryInto<NaiveDateTime> for DateTime {
    type Error = anyhow::Error;
    fn try_into(self) -> Result<NaiveDateTime> {
        Ok(NaiveDateTime::parse_from_str(
            &self.to_string(),
            "%Y-%m-%d %H:%M:%S",
        )?)
    }
}

#[test]
fn test_date_time_bcd_into_naive_date_time() {
    let date_time =
        NaiveDateTime::parse_from_str("2019-08-01 08:31:22", "%Y-%m-%d %H:%M:%S").unwrap();
    let bcd = DateTime::new(2019, 8, 1, 8, 31, 22);
    let converted: NaiveDateTime = bcd.try_into().unwrap();
    assert_eq!(converted, date_time);
}

impl Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}", self.date, self.time)
    }
}

#[test]
fn test_date_time_bcd_display() {
    let date_time = DateTime::new(2019, 8, 1, 8, 31, 22);
    assert_eq!(date_time.to_string(), "2019-08-01 08:31:22");
}

#[derive(bincode::Decode, bincode::Encode, PartialEq, Eq, Debug, Default)]
pub struct DateShortBCD {
    bcd: (u8, u8, u8),
}

impl DateShortBCD {
    pub fn new(year: u16, month: u8, day: u8) -> Self {
        let bcd = bcd::encode(format!("{:02}{:02}{:02}", year % 100, month, day));
        DateShortBCD {
            bcd: (bcd[0], bcd[1], bcd[2]),
        }
    }
}

impl TryInto<NaiveDate> for DateShortBCD {
    type Error = anyhow::Error;
    fn try_into(self) -> Result<NaiveDate> {
        Ok(NaiveDate::parse_from_str(&self.to_string(), "%y%m%d")?)
    }
}

#[test]
fn test_date_short_bcd_into_naive_date() {
    let date = NaiveDate::from_ymd(2019, 1, 1);
    let bcd = DateShortBCD::new(2019, 1, 1);
    let converted: NaiveDate = bcd.try_into().unwrap();
    assert_eq!(converted, date);
}

impl Display for DateShortBCD {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:02x}{:02x}{:02x}", self.bcd.0, self.bcd.1, self.bcd.2)
    }
}

#[test]
fn test_date_short_bcd_display() {
    let date_time = DateShortBCD::new(2019, 8, 1);
    assert_eq!(date_time.to_string(), "190801");
}
