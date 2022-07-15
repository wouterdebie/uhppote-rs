use chrono::NaiveDate;

use crate::messages::{GetCardByIndexResponse, GetCardByIDResponse};
use anyhow::Result;

#[derive(Debug)]
pub struct Card {
    pub number: u32,
    pub from: NaiveDate,
    pub to: NaiveDate,
    pub doors: Vec<u8>,
}

impl TryFrom<GetCardByIndexResponse> for Card {
    type Error = anyhow::Error;
    fn try_from(response: GetCardByIndexResponse) -> Result<Card> {
        Ok(Card {
            number: response.card_number,
            from: response.from.try_into()?,
            to: response.to.try_into()?,
            doors: vec![
                response.door_1,
                response.door_2,
                response.door_3,
                response.door_4,
            ],
        })
    }
}

impl TryFrom<GetCardByIDResponse> for Card {
    type Error = anyhow::Error;
    fn try_from(response: GetCardByIDResponse) -> Result<Card> {
        Ok(Card {
            number: response.card_number,
            from: response.from.try_into()?,
            to: response.to.try_into()?,
            doors: vec![
                response.door_1,
                response.door_2,
                response.door_3,
                response.door_4,
            ],
        })
    }
}
