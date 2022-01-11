use serde::{Deserialize, Serialize};
use std::default::Default;

/// Representation of a Lichess user's performances.
/// Derived from [lila.user.Perfs][1].
/// [1]: <https://github.com/ornicar/lila/blob/master/modules/user/src/main/Perfs.scala>
#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase", default)]
pub struct Performances {
    chess960: Performance,
    atomic: Performance,
    racing_kings: Performance,
    ultra_bullet: Performance,
    blitz: Performance,
    king_of_the_hill: Performance,
    bullet: Performance,
    correspondence: Performance,
    horde: Performance,
    puzzle: Performance,
    classical: Performance,
    rapid: Performance,
    storm: StormPerformance,
}

/// Representation of a user's performance in any variant except Puzzle Storm.
/// Derived from [lila.rating.Perf][1].
/// [1]: <https://github.com/ornicar/lila/blob/master/modules/rating/src/main/Perf.scala>
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Performance {
    games: u64,
    rating: u64,
    rd: u64,
    prog: i64,
    #[serde(default)]
    prov: bool,
}

impl Default for Performance {
    fn default() -> Self {
        Performance {
            games: 0,
            rating: 1500,
            rd: 500,
            prog: 0,
            prov: true,
        }
    }
}

/// Representation of a user's performance in the Puzzle Storm variant.
/// Derived from [lila.rating.Perf.Storm][1].
/// [1]: <https://github.com/ornicar/lila/blob/master/modules/rating/src/main/Perf.scala>
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StormPerformance {
    runs: u64,
    score: u64,
}

impl Default for StormPerformance {
    fn default() -> Self {
        StormPerformance { runs: 0, score: 0 }
    }
}
