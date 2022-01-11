use serde::{Deserialize, Serialize};

/// Representation of a Lichess user's title.
/// Derived from [lila.user.Title][1].
/// [1]: <https://github.com/ornicar/lila/blob/master/modules/user/src/main/Title.scala>
#[derive(Serialize, Deserialize, Debug)]
pub enum Title {
    GM,
    WGM,
    IM,
    WIM,
    FM,
    WFM,
    CM,
    WCM,
    NM,
    WNM,
    LM,
    BOT,
}
