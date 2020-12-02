use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "status", rename_all = "lowercase")]
pub enum Event {
    Online { version: String },
    Offline,
}
