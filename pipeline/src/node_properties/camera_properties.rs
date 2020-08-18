use crate::resolution::Resolution;

pub trait CameraProperties {
    type Runtime: CameraRuntime;

    fn resolution(&self) -> Option<&Resolution>;
    fn set_resolution(&mut self, resolution: Option<Resolution>);
    fn framerate(&self) -> Option<u32>;
    fn set_framerate(&mut self, framerate: Option<u32>);
    fn source(&self) -> &str;
    fn set_source(&mut self, source: String);
    fn runtime(&self) -> Option<&Self::Runtime>;
    fn runtime_mut(&mut self) -> Option<&mut Self::Runtime>;
}

pub trait CameraRuntime {
    fn uri(&self) -> &url::Url;
    fn set_uri(&mut self, url: url::Url);
}

macro_rules! impl_camera_props {
    ($camera_type:ty, $runtime: ty) => {
        impl crate::CameraProperties for $camera_type {
            type Runtime = $runtime;

            fn resolution(&self) -> Option<&Resolution> {
                self.resolution.as_ref()
            }

            fn set_resolution(&mut self, resolution: Option<Resolution>) {
                self.resolution = resolution;
            }

            fn framerate(&self) -> Option<u32> {
                self.framerate
            }

            fn set_framerate(&mut self, framerate: Option<u32>) {
                self.framerate = framerate;
            }

            fn source(&self) -> &str {
                &self.source
            }

            fn set_source(&mut self, source: String) {
                self.source = source;
            }

            fn runtime(&self) -> Option<&$runtime> {
                self.runtime.as_ref()
            }

            fn runtime_mut(&mut self) -> Option<&mut $runtime> {
                self.runtime.as_mut()
            }
        }

        impl crate::CameraRuntime for $runtime {
            fn uri(&self) -> &url::Url {
                &self.uri
            }

            fn set_uri(&mut self, uri: url::Url) {
                self.uri = uri
            }
        }
    };
}

#[cfg(test)]
mod test {
    use crate::{CameraProperties, CameraRuntime, Resolution};
    use crate::{UsbCameraProperties, UsbCameraRuntime};
    use std::str::FromStr;
    use url::Url;

    // A simple unit test to ensure our trait works as expected, i-e specific Camera properties
    // types can be used easily/nicely in a generic context.
    #[test]
    fn generic_camera_api() {
        let usb_camera = UsbCameraProperties {
            source: String::from("USB"),
            runtime: Some(UsbCameraRuntime {
                uri: Url::from_str("file:///whatever").unwrap(),
            }),
            resolution: Some(Resolution {
                width: 640,
                height: 480,
            }),
            framerate: Some(30),
        };

        check_camera_props(&usb_camera);
    }

    fn check_camera_props<C>(camera: &C)
    where
        C: CameraProperties,
    {
        assert_eq!(camera.source(), "USB");
        assert_eq!(camera.framerate().unwrap(), 30);
        assert_eq!(
            *camera.resolution().unwrap(),
            Resolution {
                width: 640,
                height: 480,
            },
        );
        assert_eq!(
            camera.runtime().unwrap().uri(),
            &Url::from_str("file:///whatever").unwrap()
        );
    }
}
