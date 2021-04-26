pub use gstreamer::{CoreError, LibraryError, ResourceError, StreamError};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use zvariant::derive::Type;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
    pub deployment_id: Uuid,
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
    NodeLog(NodeLogs),
}

#[derive(Serialize, Deserialize, Type, Debug, Clone)]
pub enum GstError {
    Core(CoreError),
    Library(LibraryError),
    Resource(ResourceError),
    Stream(StreamError),
}

#[derive(Serialize, Deserialize, Type, Debug, Clone)]
pub struct NodeLog {
    // ID of the source node. Enable it when we've means to set it.
    //pub source: String,
    pub msg: String,
}

#[derive(Serialize, Deserialize, Type, Debug, Clone)]
pub struct NodeLogs {
    pub logs: Vec<NodeLog>,
}

impl NodeLogs {
    pub fn bytes_len(&self) -> usize {
        self.logs.iter().map(|log| log.msg.len()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_logs_len() {
        let logs = NodeLogs {
            logs: vec![
                NodeLog {
                    msg: "hi".to_string(),
                },
                NodeLog {
                    msg: "there".to_string(),
                },
            ],
        };
        assert_eq!(logs.bytes_len(), 7);
    }
}
