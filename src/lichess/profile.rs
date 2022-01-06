use serde::{ Serialize, Deserialize };

/// Representation of a Lichess user profile.
/// Derived from [lila.user.Profile][1].
/// [1]: <https://github.com/ornicar/lila/blob/master/modules/user/src/main/Profile.scala>
#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase", default)]
pub struct Profile {
    country: Option<String>,
    location: Option<String>,
    bio: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    fide_rating: Option<i32>,
    uscf_rating: Option<i32>,
    ecf_rating: Option<i32>,
    links: Option<String>
}
