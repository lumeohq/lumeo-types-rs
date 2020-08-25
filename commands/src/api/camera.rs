pub use lumeo_events::camera::Camera;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Request {
    /// Discover cameras. Returned data can be incomplete if camera
    /// requires authorization.
    Discover,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DiscoverResponse(pub Vec<Camera>);
