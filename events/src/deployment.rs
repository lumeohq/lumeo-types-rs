use serde::{Deserialize, Serialize};

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
}
