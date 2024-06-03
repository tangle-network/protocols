use serde::{Deserialize, Serialize};
use sysinfo::Pid;
use crate::protocol::process::manager::*;

/// A Process spawned by gadget-executor, running a given service
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct GadgetProcess {
    /// Which Blueprint/Service the process is running
    pub(crate) service: String,
    /// The name of the process itself
    pub(crate) process_name: String,
    /// Process ID
    pub(crate) pid: Pid,
}

pub(crate) enum Status {
    // Returns the GadgetProcess to easily access/manage
    Active(GadgetProcess),
    // Includes what service is dead (was running)
    Dead(String),
}

pub trait ProcessIdentifier {
    fn service(&self) -> String;
}
