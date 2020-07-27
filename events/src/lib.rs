use serde::{Deserialize, Serialize};

pub mod camera;
pub mod deployment;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Event {
    Deployment(deployment::Event),
    Camera(camera::Event),
}
