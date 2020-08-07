pub trait StreamProperties {
    /// The stream type, e.g "webrtc", "rtsp" etc
    const STREAM_TYPE: &'static str;
    type Runtime: StreamRuntime;

    fn runtime(&self) -> &Self::Runtime;
    fn runtime_mut(&mut self) -> &mut Self::Runtime;
}

pub trait StreamRuntime {
    fn uri(&self) -> &url::Url;
    fn set_uri(&mut self, url: url::Url);
    fn udp_port(&self) -> Option<u16>;
    fn set_udp_port(&mut self, port: Option<u16>);
    fn stream_id(&self) -> Option<&str>;
    fn set_stream_id(&mut self, stream_id: Option<String>);
}

macro_rules! impl_stream_props {
    ($stream_type:ty, $runtime: ty, $type_str:literal) => {
        impl crate::StreamProperties for $stream_type {
            const STREAM_TYPE: &'static str = $type_str;
            type Runtime = $runtime;

            fn runtime(&self) -> &$runtime {
                &self.runtime
            }

            fn runtime_mut(&mut self) -> &mut $runtime {
                &mut self.runtime
            }
        }

        impl crate::StreamRuntime for $runtime {
            fn uri(&self) -> &url::Url {
                &self.uri
            }

            fn set_uri(&mut self, uri: url::Url) {
                self.uri = uri
            }

            fn udp_port(&self) -> Option<u16> {
                self.udp_port
            }

            fn set_udp_port(&mut self, port: Option<u16>) {
                self.udp_port = port;
            }

            fn stream_id(&self) -> Option<&str> {
                self.stream_id.as_deref()
            }

            fn set_stream_id(&mut self, stream_id: Option<String>) {
                self.stream_id = stream_id;
            }
        }
    };
}
