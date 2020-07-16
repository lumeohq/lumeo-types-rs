use serde::{Deserialize, Serialize};

use crate::{NodeProperties, SourcePads};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Node {
    id: String,
    #[serde(flatten)]
    props: NodeProperties,
    #[serde(rename = "wires")]
    source_pads: SourcePads,
}

impl Node {
    pub fn new(id: &str, props: NodeProperties, source_pads: Option<SourcePads>) -> Self {
        Self {
            id: id.to_string(),
            props,
            source_pads: source_pads.unwrap_or_else(SourcePads::new),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn properties(&self) -> &NodeProperties {
        &self.props
    }

    pub fn source_pads(&self) -> &SourcePads {
        &self.source_pads
    }

    pub fn source_pads_mut(&mut self) -> &mut SourcePads {
        &mut self.source_pads
    }
}
