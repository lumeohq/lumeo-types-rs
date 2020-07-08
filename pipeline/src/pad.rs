use std::collections::HashMap;
use std::str::FromStr;

use serde::de::{Deserialize, Deserializer, MapAccess, Visitor};
use serde::de::{Error, Unexpected};

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct SinkPad {
    // FIXME: this should be a Node ref
    pub node: String,
    pub name: String,
}

impl FromStr for SinkPad {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split('.').collect::<Vec<_>>()[..] {
            [node, name] => Ok(SinkPad {
                node: node.to_string(),
                name: name.to_string(),
            }),
            _ => Err("Bad pad format".to_string()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourcePad {
    pub name: String,
    pub sinks: Vec<SinkPad>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SourcePads(HashMap<String, SourcePad>);

impl SourcePads {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn add(&mut self, pad: SourcePad) {
        self.0.insert(pad.name.clone(), pad);
    }

    pub fn get(&self, name: &str) -> Option<&SourcePad> {
        self.0.get(name)
    }

    pub fn all(&self) -> Vec<&SourcePad> {
        self.0.values().collect()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

// FIXME: Manual Deserialize is complicated. We should change the serialized YAML format so we don't
// need to do this and just use the derive macro.
impl<'de> Deserialize<'de> for SourcePads {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let visitor = SourcePadsVisitor;

        deserializer.deserialize_map(visitor)
    }
}

struct SourcePadsVisitor;

impl<'de> Visitor<'de> for SourcePadsVisitor {
    type Value = SourcePads;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a node's source pad")
    }

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut pads = SourcePads::new();
        while let Some(name) = map.next_key::<String>()? {
            let sinks = map.next_value::<Vec<String>>()?;
            let mut pad = SourcePad {
                name,
                sinks: vec![],
            };
            for sink in sinks {
                let sink_pad = SinkPad::from_str(&sink).map_err(|_| {
                    Error::invalid_value(Unexpected::Str(&sink), &"NODE_ID.SINK_PAD_NAME")
                })?;
                pad.sinks.push(sink_pad);
            }
            pads.add(pad);
        }

        Ok(pads)
    }
}
