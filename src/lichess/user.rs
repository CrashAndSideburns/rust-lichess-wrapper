use crate::lichess::count::Count;
use crate::lichess::performances::Performances;
use crate::lichess::profile::Profile;
use crate::lichess::title::Title;
use crate::lichess::playtime::PlayTime;

use serde::{Deserialize, Serialize};
use chrono::{ Utc, DateTime };
use chrono::serde::ts_milliseconds;

/// Light representation of a Lichess user.
/// Derived from [lila.common.LightUser][1].
/// [1]: <https://github.com/ornicar/lila/blob/master/modules/common/src/main/LightUser.scala>
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LightUser {
    pub name: String,
    #[serde(default)]
    pub title: Option<Title>,
    #[serde(default)]
    pub patron: bool,
    pub id: String,
}

/// Representation of a Lichess user.
/// Derived from [lila.user.User][1]. User and ExtendedUser result from
/// different serializations of the same object.
/// [1]: <https://github.com/ornicar/lila/blob/master/modules/user/src/main/User.scala>
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub username: String,
    #[serde(default)]
    pub online: bool,
    #[serde(default)]
    pub perfs: Performances,
    #[serde(with = "ts_milliseconds")]
    pub created_at: DateTime<Utc>,
    #[serde(default)]
    pub disabled: bool,
    #[serde(default)]
    pub tos_violation: bool,
    #[serde(default)]
    pub profile: Profile,
    #[serde(with = "ts_milliseconds")]
    pub seen_at: DateTime<Utc>,
    #[serde(default)]
    pub patron: bool,
    #[serde(default)]
    pub verified: bool,
    pub play_time: PlayTime,
    #[serde(default)]
    title: Option<Title>
}

/// Extended representation of a Lichess user.
/// Derived from [lila.user.User][1]. User and ExtendedUser result from
/// different serializations of the same object.
/// [1]: <https://github.com/ornicar/lila/blob/master/modules/user/src/main/User.scala>
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExtendedUser {
    pub id: String,
    pub username: String,
    #[serde(default)]
    pub online: bool,
    #[serde(default)]
    pub perfs: Performances,
    #[serde(with = "ts_milliseconds")]
    pub created_at: DateTime<Utc>,
    #[serde(default)]
    pub disabled: bool,
    #[serde(default)]
    pub tos_violation: bool,
    #[serde(default)]
    pub profile: Profile,
    #[serde(with = "ts_milliseconds")]
    pub seen_at: DateTime<Utc>,
    #[serde(default)]
    pub patron: bool,
    #[serde(default)]
    pub verified: bool,
    pub play_time: PlayTime,
    #[serde(default)]
    pub title: Option<Title>,
    pub url: String,
    #[serde(default)]
    pub playing: Option<String>,
    #[serde(default)]
    pub completion_rate: i32,
    pub count: Count,
    #[serde(default)]
    pub streaming: bool,

    // Making these field Option<bool> rather than bool is not strictly
    // justified by the API, but makes sense in the context of distinguishing
    // the values of these fields when the struct is yielded by a
    // non-authenticated client, in which case neither true nor false makes
    // sense.
    #[serde(default = "follow_field_default")]
    pub followable: Option<bool>,
    #[serde(default = "follow_field_default")]
    pub following: Option<bool>,
    #[serde(default = "follow_field_default")]
    pub blocking: Option<bool>,
    #[serde(default = "follow_field_default")]
    pub follows_you: Option<bool>
}

// This is something of an ugly hack. Option<bool>::default() is None, but when
// deserializing Users we need it to be Some(false), as when the field is
// absent the value ought to default to false like all other booleans, and only
// be overwritten with None when the caller is a non-authenticated client.
fn follow_field_default() -> Option<bool> {
    Some(false)
}
