use std::collections::HashMap;
use itertools::Itertools;
use sysinfo::{Pid, System};
use serde::{Serialize, Deserialize};
use crate::protocol::utils::*;
use crate::run_command;
use crate::process::types::{GadgetProcess, Status};

/// Manager for gadget-executor process. The process are recorded to be controlled by their PID.
/// This Manager can be reconstructed from a file to recover a gadget-executor.
#[derive(Serialize, Deserialize, Debug)]
struct GadgetProcessManager {
    /// Hashmap that contains all the children spawned by this Manager
    children: HashMap<String, GadgetProcess>,
}

impl GadgetProcessManager {
    pub(crate) fn new() -> GadgetProcessManager {
        GadgetProcessManager{
            children: HashMap::new(),
        }
    }

    pub(crate) fn from_json(bytes: Vec<u8>) -> Result<GadgetProcessManager, Box<dyn std::error::Error>> {
        serde_json::from_str(bytes.into())?

        // TODO: Should 'check-in' with all processes to see what their statuses are


    }

    pub(crate) fn to_json(&self) -> Vec<u8> {
        serde_json::to_string(self)?.into_bytes()
    }

    pub(crate) async fn run(&self, command: &str) {
        let _ = run_command!(command);
    }

    pub(crate) async fn remove_dead(&mut self) -> Vec<GadgetProcess> {
        let dead_processes = Vec::new();
        let s = System::new_all();

        // TODO: If dead children are `supposed` to be running, we should start them up again instead of just removing them

        // Remove any processes that are no longer running
        self.children.retain(|_, c| s.processes().keys().iter().contains(*c.pid));

        dead_processes
    }

    /// Checks the status of a running service by Service, PID, or Name.
    pub(crate) async fn status(&self, service: String) -> Status {



    }

}