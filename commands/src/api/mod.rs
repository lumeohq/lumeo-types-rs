use std::fmt::{Display, Formatter};
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

pub mod deployment;
pub mod webrtc;

/// Error types
#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to deserialize command")]
    DeserializeCommand,
}

/// API request packet type
#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    /// Request body
    pub body: Body,
    /// Response correlation string
    pub respond_to: Option<String>,
}

/// API request body
#[derive(Debug, Serialize, Deserialize)]
pub enum Body {
    /// Start pipeline deployment
    StartDeployment(deployment::StartDeployment),
    /// Stop deployment
    StopDeployment(deployment::StopDeployment),
    /// Discover cameras
    Discover,
    /// WebRTC subcommands collection
    WebRtc(webrtc::Request),
}

impl FromStr for Request {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        serde_json::from_str(s).map_err(|_| Error::DeserializeCommand)
    }
}

impl Request {
    /// Create new notification
    pub fn notification(body: Body) -> Self {
        Self {
            body,
            respond_to: None,
        }
    }

    /// Create new request
    pub fn request(body: Body) -> Self {
        Self {
            body,
            respond_to: Some(format!("resp/{}", Uuid::new_v4())),
        }
    }
}

impl Display for Request {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let s = serde_json::to_string(self).map_err(|_| std::fmt::Error)?;
        f.write_str(s.as_str())
    }
}
