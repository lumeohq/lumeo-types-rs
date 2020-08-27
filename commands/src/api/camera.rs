pub use lumeo_events::camera::Camera;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use url::Url;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Request {
    /// Discover cameras. Returned data can be incomplete if camera
    /// requires authorization. For authorized access use `GetWithAuth`.
    Discover,

    /// Get camera information
    GetWithAuth {
        url: Url,
        credentials: Option<Credentials>,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DiscoverResponse(pub Vec<Camera>);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetWithAuthResponse(pub Result<Camera, GetWithAuthError>);

#[derive(Serialize, Deserialize, Debug, Clone, Error)]
pub enum GetWithAuthError {
    #[error("Camera is offline")]
    CameraOffline,

    #[error("Invalid credentials")]
    InvalidCredentials,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Credentials {
    /// Camera user name
    pub username: String,

    /// Plaintext password
    pub password: String,
}
