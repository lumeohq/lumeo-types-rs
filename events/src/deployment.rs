use serde::{Deserialize, Serialize};
use zvariant::derive::Type;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
    pub deployment_id: String,
    pub event: DeploymentEventKind,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DeploymentEventKind {
    Started,
    StartFailed,
    Stopped,
    StopFailed,
    ExitedUnexpectedly,
    GstError(GstError),
}

#[derive(Serialize, Deserialize, Type, Debug, Clone)]
pub enum GstErrorDomain {
    Core = 1,
    Library = 2,
    Resource = 3,
    Stream = 4,
}

#[derive(Serialize, Deserialize, Type, Debug, Clone)]
pub struct GstError {
    pub domain: GstErrorDomain,
    // FIXME: We probaly could do better than this and use an enum here.
    pub code: i32,
    pub description: String,
}
