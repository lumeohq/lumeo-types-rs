use std::collections::HashMap;

use serde::de::{Deserialize, Deserializer, Error};
use serde::ser::{Serialize, SerializeSeq, Serializer};

use crate::Node;

#[derive(Default, Debug, Clone)]
pub struct Pipeline {
    nodes: HashMap<String, Node>,
}

impl Pipeline {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.id().to_string(), node);
    }

    pub fn nodes(&self) -> impl Iterator<Item = &Node> {
        self.nodes.values()
    }

    pub fn node_by_id(&self, id: &str) -> Option<&Node> {
        self.nodes.get(id)
    }
}

// Manual implemention is needed here as we want to only serialize as series of nodes.
impl Serialize for Pipeline {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.nodes.len()))?;
        for node in self.nodes.values() {
            seq.serialize_element(node)?;
        }

        seq.end()
    }
}

// Manual implemention is needed here as we need to verify source pads don't link to inexistant
// sink pads.
impl<'de> Deserialize<'de> for Pipeline {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let nodes = <Vec<Node>>::deserialize(deserializer)?;
        let mut pipeline = Pipeline::new();
        for node in nodes {
            pipeline.add_node(node);
        }

        // Ensure all sink pads are setup correctly
        for node in pipeline.nodes() {
            for src_pad in node.source_pads().all() {
                for sink in &src_pad.sinks {
                    pipeline.node_by_id(&sink.node).ok_or_else(|| {
                        Error::custom(&format!("Destination node `{}` not foud", sink.node))
                    })?;
                }
            }
        }

        Ok(pipeline)
    }
}
