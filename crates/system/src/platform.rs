use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

#[derive(Clone, Hash, PartialEq, Eq)]
pub enum Platform {
    Ctr,
    Hac,
    Cafe,
}

impl Platform {
    pub const ALL: [Platform; 3] = [Platform::Ctr, Platform::Hac, Platform::Cafe];
}

impl FromStr for Platform {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.eq_ignore_ascii_case("ctr") {
            Ok(Platform::Ctr)
        } else if s.eq_ignore_ascii_case("hac") {
            Ok(Platform::Hac)
        } else if s.eq_ignore_ascii_case("cafe") {
            Ok(Platform::Cafe)
        } else {
            Err(format!("unknown platform: {s}"))
        }
    }
}

impl Display for Platform {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let name = match self {
            Platform::Ctr => "ctr",
            Platform::Hac => "hac",
            Platform::Cafe => "cafe",
        };
        write!(f, "{name}")
    }
}
