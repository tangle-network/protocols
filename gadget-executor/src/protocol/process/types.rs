use serde::{Deserialize, Serialize};
use sysinfo::Pid;
use tokio::io::{BufReader, Lines};
use crate::protocol::process::manager::*;

/// A Process spawned by gadget-executor, running a given service
#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct GadgetProcess {
    /// The command executed
    // TODO: Add support for multiple commands with a vector
    pub(crate) command: String,
    /// The name of the process itself
    pub(crate) process_name: String,
    /// Process ID
    pub(crate) pid: Option<u32>,
    /// History of output from process for reviewing/tracking progress
    pub(crate) output: Vec<String>,
}

impl GadgetProcess {
    pub fn new(command: String, process_name: String, pid: Option<u32>, output: Vec<String>) -> GadgetProcess {
        GadgetProcess {
            command,
            process_name,
            pid,
            output,
        }
    }
}

pub(crate) enum Status {
    /// Process is running or able to run
    Active,
    /// Stopped process
    Inactive,
    /// Sleeping process, either waiting for resources or a signal
    Sleeping,
    /// Zombie process
    Dead,
    /// Unknown or invalid status
    Unknown,
}

impl From<String> for Status {
    fn from(value: String) -> Status {
        match value.as_str() {
            "R" => { Status::Active },
            "S" | "D" => { Status::Sleeping },
            "T" => { Status::Inactive },
            "Z" => { Status::Dead },
            _ => { Status::Unknown },
        }
    }
}

impl TryFrom<String> for Status {
    type Error = Box<dyn std::error::Error>;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "R" => { Ok(Status::Active) },
            "S" | "D" => { Ok(Status::Sleeping) },
            "T" => { Ok(Status::Inactive) },
            "Z" => { Ok(Status::Dead) },
            _ => { Err(format!("Status Error: {} is an invalid status", value)) },
        }
    }
}

pub trait ProcessIdentifier {
    fn service(&self) -> String;
}
