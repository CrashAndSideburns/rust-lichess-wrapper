use serde::{Deserialize, Serialize};

/// Represention of a Lichess user's overall game stats.
/// Derived from [lila.user.Count][1].
/// [1]: <https://github.com/ornicar/lila/blob/master/modules/user/src/main/Count.scala>
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Count {
    pub all: i32,
    pub rated: i32,
    pub ai: i32,
    pub draw: i32,
    pub draw_h: i32,
    pub loss: i32,
    pub loss_h: i32,
    pub win: i32,
    pub win_h: i32,
    pub bookmark: i32,
    pub playing: i32,
    pub import: i32,
    pub me: i32,
}
