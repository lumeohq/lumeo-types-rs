//! Combine multiple streams, and interleave the frames to create one output stream.
//! Also resizes all frames to specified output resolution.

use crate::Resolution;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct CombineProperties {
    pub resolution: Option<Resolution>,
    pub num_streams: u32,
}
