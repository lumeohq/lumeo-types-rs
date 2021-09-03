#![warn(
    rust_2018_idioms,
    unused_qualifications,
    clippy::cloned_instead_of_copied,
    clippy::dbg_macro,
    clippy::str_to_string,
    clippy::todo,
    clippy::unreadable_literal,
    clippy::unseparated_literal_suffix,
    clippy::wildcard_imports
)]

use serde::{Deserialize, Serialize};

pub mod deployment;
pub mod gateway;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Event {
    Gateway(gateway::Event),
    Deployment(deployment::Event),
}
