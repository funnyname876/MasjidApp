use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DonationIntention {
    Lillah,
    Sadaqah,
    Zakat,
}
impl Default for DonationIntention {
    fn default() -> Self {
        Self::Sadaqah
    }
}
impl Display for DonationIntention {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            DonationIntention::Lillah => "Lillah".to_owned(),
            DonationIntention::Sadaqah => "Sadaqah".to_owned(),
            DonationIntention::Zakat => "Zakat".to_owned(),
        };
        write!(f, "{}", str)
    }
}

impl FromStr for DonationIntention {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "lillah" => Ok(Self::Lillah),
            "sadaqah" => Ok(Self::Sadaqah),
            "zakat" => Ok(Self::Zakat),
            _ => Err(()),
        }
    }
}
