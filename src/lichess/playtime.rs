use serde::{Deserialize, Serialize};

/// Represention of a Lichess user's playtime.
/// Derived from [lila.user.User.PlayTime][1].
/// [1]: <https://github.com/ornicar/lila/blob/master/modules/user/src/main/User.scala>
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlayTime {
    pub total: i32,
    pub tv: i32,
}
