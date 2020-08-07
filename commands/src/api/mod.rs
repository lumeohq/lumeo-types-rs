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

/// API message type
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Message {
    /// Request
    Request(Request),
    /// Notification
    Notification(Notification),
}

/// Request type
///
/// Expect response from remote end
#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    /// Request body
    pub body: Body,
    /// Response correlation string
    pub respond_to: String,
}

/// Notification
///
/// Fire and forget packet
#[derive(Debug, Serialize, Deserialize)]
pub struct Notification {
    /// Notification body
    pub body: Body,
}

/// API message body payloads
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

impl Request {
    pub fn new(body: Body) -> Self {
        Request {
            body,
            respond_to: format!("resp/{}", Uuid::new_v4()),
        }
    }
}
