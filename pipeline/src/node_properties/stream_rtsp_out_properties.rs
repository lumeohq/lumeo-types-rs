use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StreamRtspOutProperties {
    #[serde(flatten)]
    pub runtime: StreamRtspOutRuntime,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StreamRtspOutRuntime {
    /// RTSP stream URI.
    ///
    /// This field is set to `Some` by `lumeod`.
    // TODO: this should be set by API server, more details here:
    //       https://app.clubhouse.io/lumeo/story/940/set-correct-stream-url-for-pipeline-streams-created-for-webrtc-nodes
    pub uri: Option<Url>,

    /// UDP port for `udpsink` element.
    ///
    /// This field is set to `Some` by `lumeod`
    ///
    /// Since pipeline can have multiple RTSP output streams we need to
    /// distribute ports at `lumeod` level.
    pub udp_port: Option<u16>,
}
