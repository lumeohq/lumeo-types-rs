use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod camera;
pub mod deployment;
pub mod snapshot;
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
    /// Camera commands
    Camera(camera::Request),
    /// WebRTC subcommands collection
    WebRtc(webrtc::Request),
    /// Snapshot commands
    Snapshot(snapshot::Request),
}
