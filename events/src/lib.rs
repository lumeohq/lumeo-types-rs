use serde::{Deserialize, Serialize};

pub mod deployment;
pub mod gateway;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Event {
    Gateway(gateway::Event),
    Deployment(deployment::Event),
}
