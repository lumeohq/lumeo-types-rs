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
#![allow(clippy::suspicious_else_formatting)]

use std::{collections::BTreeMap, fs, process};

use anyhow::Context;
use clap::Clap;
use lumeo_pipeline::Pipeline;
use serde_json::Value as JsonValue;

#[derive(Clap)]
struct Options {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    /// Checks whether a JSON pipeline definition can be deserialized correctly (reads from stdin)
    CheckPipeline { pipeline_file: String, configuration_file: Option<String> },
}

fn main() {
    let opt = Options::parse();
    match opt.subcmd {
        SubCommand::CheckPipeline { pipeline_file, configuration_file } => {
            if let Err(e) = check_pipeline(&pipeline_file, configuration_file.as_deref()) {
                eprintln!("{}", e);
                process::exit(1);
            }
        }
    }
}

fn check_pipeline(pipeline_file: &str, configuration_file: Option<&str>) -> anyhow::Result<()> {
    type Configuration = BTreeMap<String, serde_json::Map<String, JsonValue>>;

    let mut pipeline_json = fs::read_to_string(pipeline_file)?;

    if let Some(file) = configuration_file {
        let configuration_json = fs::read_to_string(file)?;
        let configuration = match serde_json::from_str::<Configuration>(&configuration_json) {
            Ok(config) => config,
            Err(e) => {
                eprint!("Invalid pipeline configuration: ");
                return Err(e.into());
            }
        };

        update_pipeline_def(&mut pipeline_json, &configuration).context("configuring pipeline")?;
    }

    match serde_json::from_str::<Pipeline>(&pipeline_json) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprint!("Invalid pipeline definition: ");
            Err(e.into())
        }
    }
}

fn update_pipeline_def(
    pipeline_json: &mut String,
    configuration: &BTreeMap<String, serde_json::Map<String, JsonValue>>,
) -> anyhow::Result<()> {
    let mut pipeline_nodes: Vec<JsonValue> = serde_json::from_str(pipeline_json)?;

    for node in &mut pipeline_nodes {
        let node_id = node["id"].as_str().context("id is not a string")?.to_owned();
        let props_obj =
            node["properties"].as_object_mut().context("properties is not an object")?;

        if let Some(config_props) = configuration.get(&node_id) {
            props_obj.extend(config_props.clone());
        }
    }

    *pipeline_json = serde_json::to_string(&pipeline_nodes)?;

    Ok(())
}
