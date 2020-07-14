use serde::Serialize;
use url::Url;

// Ditch all manual Serialize + Deserialize code after changing the properties to specific types.
// This includes the use of `serialize_with` attribute.

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct StreamRtspOutProperties {
    pub uri: Url,
    #[serde(serialize_with = "super::serialize_required")]
    pub udp_port: u16,
}

impl<'de> serde::de::Deserialize<'de> for StreamRtspOutProperties {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let props = <std::collections::HashMap<String, String>>::deserialize(deserializer)?;
        Ok(StreamRtspOutProperties {
            uri: super::get_required(&props, "uri")?,
            udp_port: super::get_required(&props, "udp_port")?,
        })
    }
}
