use serde::{Serialize, Deserialize};
use data_encoding::BASE64;


const VERSION: &str = env!("CARGO_PKG_VERSION");
const CONTACT: &str = "aHR0cHM6Ly9kaXNjb3JkLmdnL3V6cTg5R1ByVWc=";

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq  )]
pub struct About {
    pub version: String,
    pub contact: String,
}

impl About {
    pub fn new() -> About {
        let input: Vec<u8> = CONTACT.into();
        let binding = BASE64.decode(&input).unwrap();
        let s = String::from_utf8_lossy(&binding);
        About{
            version: VERSION.to_string(),
            contact: s.into_owned()
        }
    }
}
