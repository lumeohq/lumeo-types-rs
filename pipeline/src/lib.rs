pub mod node;
pub mod node_properties;
pub mod pad;
pub mod pipeline;
pub mod resolution;

pub use node::*;
pub use node_properties::*;
pub use pad::*;
pub use pipeline::*;
pub use resolution::*;

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use url::Url;

    use crate::{EncodeProperties, StreamRtspOutProperties, UsbCameraProperties};
    use crate::{NodeProperties, Pipeline, Resolution};

    #[test]
    fn pipeline_nodes_de() {
        let yaml = r#"---
- type: usb_camera
  id: camera1
  properties:
    uri: file:///dev/video0
    framerate: '15'
    resolution: 720x480
  wires:
    video:
      - encode1.input
    snapshot: []
- type: encode
  id: encode1
  properties:
    codec: 'h264'
    max_bitrate: '1500000'
    quality: '10'
    fps: '15'
  wires:
    output:
      - stream_rtsp_out1.input
- type: stream_rtsp_out
  id: stream_rtsp_out1
  properties:
    uri: rtsp://127.0.0.1:5555/mycamera
    udp_port: '5800'
  wires: {}"#;

        let pipeline: Pipeline = serde_yaml::from_str(yaml).unwrap();

        // Check usb_camera node
        let node = pipeline.node_by_id("camera1").unwrap();
        assert_eq!(node.id(), "camera1");
        assert_eq!(
            node.properties(),
            &NodeProperties::UsbCamera(UsbCameraProperties {
                uri: url::Url::from_str("file:///dev/video0").unwrap(),
                framerate: Some(15),
                resolution: Some(Resolution {
                    width: 720,
                    height: 480
                })
            })
        );
        let src_pad = node.source_pads().get("snapshot").unwrap();
        assert!(src_pad.sinks.is_empty());
        let src_pad = node.source_pads().get("video").unwrap();
        assert_eq!(src_pad.sinks, &["encode1.input".parse().unwrap()]);

        // Check encode1 node
        let node = pipeline.node_by_id("encode1").unwrap();
        assert_eq!(node.id(), "encode1");
        assert_eq!(
            node.properties(),
            &NodeProperties::Encode(EncodeProperties {
                codec: "h264".to_string(),
                max_bitrate: Some(1_500_000),
                bitrate: None,
                quality: Some(10),
                fps: Some(15)
            }),
        );
        let src_pad = node.source_pads().get("output").unwrap();
        assert_eq!(src_pad.sinks, &["stream_rtsp_out1.input".parse().unwrap()]);

        // and finally check the stream_rtsp_out node
        let node = pipeline.node_by_id("stream_rtsp_out1").unwrap();
        assert_eq!(node.id(), "stream_rtsp_out1");
        assert_eq!(
            node.properties(),
            &NodeProperties::StreamRtspOut(StreamRtspOutProperties {
                uri: Url::from_str("rtsp://127.0.0.1:5555/mycamera").unwrap(),
                udp_port: 5800
            })
        );
        assert!(node.source_pads().is_empty());
    }
}
