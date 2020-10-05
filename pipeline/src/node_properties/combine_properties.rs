//! Combine multiple streams, and interleave the frames to create one output stream.
//! Also resizes all frames to specified output resolution.

use crate::Resolution;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct CombineProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<Resolution>,
    pub num_streams: u32,
}
