use serde::{Serialize, Deserialize};


const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq  )]
pub struct About {
    pub version: String,
}

impl About {
    pub fn new() -> About {
        About{ version: VERSION.to_string() }
    }
}
