use crate::messages::GetTimeProfileResponse;
use anyhow::Result;
use chrono::{NaiveDate, NaiveTime};

#[derive(Debug)]
pub struct TimeProfile {
    pub id: u8,
    pub linked_profile_id: u8,
    pub from: NaiveDate,
    pub to: NaiveDate,
    pub monday: bool,
    pub tuesday: bool,
    pub wednesday: bool,
    pub thursday: bool,
    pub friday: bool,
    pub saturday: bool,
    pub sunday: bool,
    pub segments: [TimeProfileSegment; 3],
}

impl TryFrom<GetTimeProfileResponse> for TimeProfile {
    type Error = anyhow::Error;
    fn try_from(response: GetTimeProfileResponse) -> Result<Self> {
        let mut segments = [TimeProfileSegment {
            start: NaiveTime::from_hms(0, 0, 0),
            end: NaiveTime::from_hms(0, 0, 0),
        }; 3];

        segments[0] = TimeProfileSegment {
            start: response.segment1_start.try_into()?,
            end: response.segment1_end.try_into()?,
        };
        segments[1] = TimeProfileSegment {
            start: response.segment2_start.try_into()?,
            end: response.segment2_end.try_into()?,
        };
        segments[2] = TimeProfileSegment {
            start: response.segment3_start.try_into()?,
            end: response.segment3_end.try_into()?,
        };

        Ok(TimeProfile {
            id: response.profile_id,
            linked_profile_id: response.linked_profile_id,
            from: response.from.try_into()?,
            to: response.to.try_into()?,
            monday: response.monday,
            tuesday: response.tuesday,
            wednesday: response.wednesday,
            thursday: response.thursday,
            friday: response.friday,
            saturday: response.saturday,
            sunday: response.sunday,
            segments,
        })
    }
}

#[derive(Debug, Copy, Clone)]
pub struct TimeProfileSegment {
    pub start: NaiveTime,
    pub end: NaiveTime,
}
