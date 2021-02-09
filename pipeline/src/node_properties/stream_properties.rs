use uuid::Uuid;

pub trait StreamProperties {
    /// The stream type, e.g "webrtc", "rtsp" etc
    const STREAM_TYPE: &'static str;
    type Runtime: StreamRuntime;

    fn runtime(&self) -> Option<&Self::Runtime>;
    fn runtime_mut(&mut self) -> Option<&mut Self::Runtime>;

    fn stream_type(&self) -> &'static str {
        Self::STREAM_TYPE
    }
}

pub trait StreamRuntime {
    fn uri(&self) -> &url::Url;
    fn set_uri(&mut self, url: url::Url);
    fn shm_path(&self) -> Option<&str>;
    fn set_shm_path(&mut self, port: Option<String>);
    fn stream_id(&self) -> Uuid;
    fn set_stream_id(&mut self, stream_id: Uuid);
}

macro_rules! impl_stream_props {
    ($stream_type:ty, $runtime: ty, $type_str:literal) => {
        impl crate::StreamProperties for $stream_type {
            const STREAM_TYPE: &'static str = $type_str;
            type Runtime = $runtime;

            fn runtime(&self) -> Option<&$runtime> {
                self.runtime.as_ref()
            }

            fn runtime_mut(&mut self) -> Option<&mut $runtime> {
                self.runtime.as_mut()
            }
        }

        impl crate::StreamRuntime for $runtime {
            fn uri(&self) -> &url::Url {
                &self.uri
            }

            fn set_uri(&mut self, uri: url::Url) {
                self.uri = uri
            }

            fn shm_path(&self) -> Option<&str> {
                self.shm_path.as_deref()
            }

            fn set_shm_path(&mut self, port: Option<String>) {
                self.shm_path = port;
            }

            fn stream_id(&self) -> uuid::Uuid {
                self.stream_id
            }

            fn set_stream_id(&mut self, stream_id: uuid::Uuid) {
                self.stream_id = stream_id;
            }
        }
    };
}
