use chrono::Utc;
use chrono::{Date, NaiveDate};

use serde::Deserialize;
use serde::Deserializer;

#[derive(Debug)]
pub struct RatingHistory {
    pub bullet: Vec<RatingHistoryRecord>,
    pub blitz: Vec<RatingHistoryRecord>,
    pub rapid: Vec<RatingHistoryRecord>,
    pub classical: Vec<RatingHistoryRecord>,
    pub correspondence: Vec<RatingHistoryRecord>,
    pub chess960: Vec<RatingHistoryRecord>,
    pub king_of_the_hill: Vec<RatingHistoryRecord>,
    pub three_check: Vec<RatingHistoryRecord>,
    pub antichess: Vec<RatingHistoryRecord>,
    pub atomic: Vec<RatingHistoryRecord>,
    pub horde: Vec<RatingHistoryRecord>,
    pub racing_kings: Vec<RatingHistoryRecord>,
    pub crazyhouse: Vec<RatingHistoryRecord>,
    pub puzzles: Vec<RatingHistoryRecord>,
    pub ultra_bullet: Vec<RatingHistoryRecord>,
}

impl<'de> Deserialize<'de> for RatingHistory {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct NamedHistory {
            name: String,
            points: Vec<RatingHistoryRecord>,
        }
        let mut arr = Vec::<NamedHistory>::deserialize(deserializer)?;
        arr.reverse();
        Ok(RatingHistory {
            bullet: arr.pop().unwrap().points,
            blitz: arr.pop().unwrap().points,
            rapid: arr.pop().unwrap().points,
            classical: arr.pop().unwrap().points,
            correspondence: arr.pop().unwrap().points,
            chess960: arr.pop().unwrap().points,
            king_of_the_hill: arr.pop().unwrap().points,
            three_check: arr.pop().unwrap().points,
            antichess: arr.pop().unwrap().points,
            atomic: arr.pop().unwrap().points,
            horde: arr.pop().unwrap().points,
            racing_kings: arr.pop().unwrap().points,
            crazyhouse: arr.pop().unwrap().points,
            puzzles: arr.pop().unwrap().points,
            ultra_bullet: arr.pop().unwrap().points,
        })
    }
}

#[derive(Debug)]
pub struct RatingHistoryRecord {
    pub rating: i32,
    pub date: Date<Utc>,
}

impl<'de> Deserialize<'de> for RatingHistoryRecord {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let arr = <[i32; 4]>::deserialize(deserializer)?;
        let date = NaiveDate::from_ymd(
            arr[0],
            (arr[1] + 1).try_into().unwrap(),
            arr[2].try_into().unwrap(),
        );
        Ok(RatingHistoryRecord {
            rating: arr[3],
            date: Date::from_utc(date, Utc),
        })
    }
}
