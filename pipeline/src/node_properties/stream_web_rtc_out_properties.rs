use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StreamWebRtcOutProperties {
    #[serde(flatten)]
    pub runtime: StreamWebRtcOutRuntime,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StreamWebRtcOutRuntime {
    /// RTSP stream URI.
    ///
    /// This field is set to `Some` by `lumeod`.
    ///
    /// WebRTC is essentially the same thing as RTSP for lumeo-gst because
    /// conversion RTSP->WebRTC is made with a separate service.
    // TODO: this should be set by API server, more details here:
    //       https://app.clubhouse.io/lumeo/story/940/set-correct-stream-url-for-pipeline-streams-created-for-webrtc-nodes
    pub uri: Option<Url>,

    /// UDP port for `udpsink` element.
    ///
    /// This field is set to `Some` by `lumeod`.
    ///
    /// Since pipeline can have multiple RTSP output streams we need to
    /// distribute ports at `lumeod` level.
    pub udp_port: Option<u16>,

    /// Stream ID.
    ///
    /// This field is set to `Some` by API server.
    ///
    /// Stream ID is used by `lumeod` to add a WebRTC endpoint to webrtcstreamer service.
    pub stream_id: Option<String>,
}

impl_stream_props!(StreamWebRtcOutProperties, StreamWebRtcOutRuntime, "webrtc");
