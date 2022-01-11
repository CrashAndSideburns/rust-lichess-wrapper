use crate::lichess::title::Title;
use crate::lichess::user::LightUser;

use serde::{Deserialize, Deserializer};

/// Lists of the top 10 users in all variants.
/// Derived from [lila.user.Perfs.Leaderboards][1].
/// [1]: <https://github.com/ornicar/lila/blob/master/modules/user/src/main/Perfs.scala>
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Top10s {
    pub bullet: [Top10; 10],
    pub blitz: [Top10; 10],
    pub rapid: [Top10; 10],
    pub classical: [Top10; 10],
    pub ultra_bullet: [Top10; 10],
    pub chess960: [Top10; 10],
    pub crazyhouse: [Top10; 10],
    pub antichess: [Top10; 10],
    pub atomic: [Top10; 10],
    pub horde: [Top10; 10],
    pub king_of_the_hill: [Top10; 10],
    pub racing_kings: [Top10; 10],
    pub three_check: [Top10; 10],
}

/// A user in a Top 10 list, with their performance in the relevant variant.
/// Derived from [lila.user.Perfs.Leaderboards][1].
/// [1]: <https://github.com/ornicar/lila/blob/master/modules/user/src/main/Perfs.scala>
#[derive(Debug)]
pub struct Top10 {
    pub user: LightUser,
    pub online: bool,
    pub perf: Top10Performance,
}

// Gross manual implementation of Deserialize to work around the fact that
// serde's alias attribute does not work if the struct containing it is
// flattened. Ideally this would all be unnecessary and I could just flatten
// LightUser and move on with my life, but since the Top10s endpoint calls what
// all other LightUser endpoints call "name" "username" instead, this is
// necessary.
impl<'de> Deserialize<'de> for Top10 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize, Debug)]
        struct Temp {
            username: String,
            id: String,
            #[serde(default)]
            title: Option<Title>,
            #[serde(default)]
            patron: bool,
            #[serde(default)]
            online: bool,
            #[serde(deserialize_with = "deserialize")]
            perfs: Top10Performance,
        }

        let temp = Temp::deserialize(deserializer)?;

        let user = LightUser {
            name: temp.username,
            id: temp.id,
            title: temp.title,
            patron: temp.patron,
        };

        Ok(Top10 {
            user,
            online: temp.online,
            perf: temp.perfs,
        })
    }
}

/// Representation of the performance of a user in a Top 10 list.
#[derive(Deserialize, Debug)]
pub struct Top10Performance {
    pub rating: i32,
    pub progress: i32,
}

// This is kind of a hack. The object returned by the Top 10 API endpoint has
// a perfs field with a single perf nested inside, corresponding to the user's
// performance in the variant in which they are in the top 10. Since this field
// is always of the same type, and the performance's variant is encoded in the
// name of the list which the Top10 is in, we remove the outer struct on
// deserialization.
fn deserialize<'de, D>(deserializer: D) -> Result<Top10Performance, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize, Debug)]
    struct WrappedTop10Performance {
        #[serde(
            alias = "blitz",
            alias = "rapid",
            alias = "classical",
            alias = "ultraBullet",
            alias = "chess960",
            alias = "crazyhouse",
            alias = "antichess",
            alias = "atomic",
            alias = "horde",
            alias = "kingOfTheHill",
            alias = "racingKings",
            alias = "threeCheck"
        )]
        bullet: Top10Performance,
    }

    let perf = WrappedTop10Performance::deserialize(deserializer)?;
    Ok(perf.bullet)
}
