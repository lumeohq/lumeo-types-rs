use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
    pub cameras: Vec<Camera>,
}

/// `Camera` type represents different camera types: IP, USB, CSI cameras
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Camera {
    /// URI to access the camera.
    ///
    /// - USB or CSI camera: local file name (for example `file:///dev/video0`).
    /// - IP camera: ONVIF device management service URL (for example `http://192.168.1.2/device`);
    pub uri: String,

    /// Status: `online`, `offline`
    pub status: Option<String>,

    /// Name
    pub name: Option<String>,

    /// Manufacturer
    pub manufacturer: Option<String>,

    /// Model
    pub model: Option<String>,

    /// Connection type:
    /// - `local` for USB and CSI cameras
    /// - `remote` for IP cameras
    pub conn_type: Option<String>,

    /// Physical interface: `usb`, `csi`, `ethernet`
    pub interface: Option<String>,

    /// RTSP stream address.
    /// It is valid for `remote` cameras only.
    /// Example: `rtsp://192.168.1.2:554/my_stream`.
    pub rtsp_uri: Option<String>,

    /// Local IP address for IP cameras
    pub ip_local: Option<String>,

    /// MAC address for IP cameras
    pub mac_address: Option<String>,

    /// List of camera capabilities
    pub capabilities: Vec<Capability>,
}

/// Configuration parameters that are supported by this specific camera.
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Capability {
    /// Capability name
    pub name: String,

    /// Width in pixels
    pub width: i32,

    /// Height in pixels
    pub height: i32,

    /// List of supported framerates
    pub framerates: Vec<Fraction>,

    /// Video format, "MJPG", "YUYV", etc.
    pub format: Option<String>,
}

/// Type representing rational number.
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Fraction {
    pub numer: i32,
    pub denom: i32,
}
