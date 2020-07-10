use url::Url;

#[derive(Debug, Clone, PartialEq)]
pub struct StreamWebRtcOutProperties {
    pub uri: Url,
    pub udp_port: u16,
}

// We can ditch this manual Deserialize impl. after changing the properties to specific types.
impl<'de> serde::de::Deserialize<'de> for StreamWebRtcOutProperties {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let props = <std::collections::HashMap<String, String>>::deserialize(deserializer)?;
        Ok(StreamWebRtcOutProperties {
            uri: super::get_required(&props, "uri")?,
            udp_port: super::get_required(&props, "udp_port")?,
        })
    }
}
