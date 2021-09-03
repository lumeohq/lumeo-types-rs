use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::Resolution;

#[skip_serializing_none]
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct GridProperties {
    pub rows: u32,
    pub columns: u32,
    pub resolution: Option<Resolution>,
}
