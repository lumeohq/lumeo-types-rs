use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrackProperties {
    pub tracker: Tracker,
    #[serde(default)]
    pub profile: TrackerProfile,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Tracker {
    /// Discriminative Correlation Filter (DCF) tracker uses a correlation filter-based
    /// online discriminative learning algorithm, coupled with a Hungarian algorithm for
    /// data association in multi-object tracking.
    Dcf,

    /// Kanade Lucas Tomasi (KLT) algorithm
    Klt,

    /// The Intersection of Union (IOU) tracker uses the intersection of the detector’s
    /// bounding boxes across frames to determine the object’s unique ID.
    Iou,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrackerProfile {
    /// Loads the default tracker configuration
    Default,

    /// If available for this tracker type, loads a configuration tuned for stationary objects
    Stationary,
}

impl Default for TrackerProfile {
    fn default() -> Self {
        TrackerProfile::Default
    }
}
