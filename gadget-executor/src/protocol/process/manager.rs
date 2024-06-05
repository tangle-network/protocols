use std::collections::HashMap;
use std::error::Error;
// use std::str::from_utf8;
use crate::process::types::{GadgetProcess, Status};
use crate::protocol::utils::*;
use crate::{craft_child_process, run_command, OS_COMMAND};
use itertools::Itertools;
use nix::libc::pid_t;
use nix::sys::signal;
use nix::sys::signal::Signal;
use procfs::process::Process;
use sysinfo::{Pid, System};

/// Manager for gadget-executor process. The processes are recorded to be controlled by their Service name.
/// This Manager can be reconstructed from a file to recover a gadget-executor.
pub struct GadgetProcessManager {
    /// Hashmap that contains all the children spawned by this Manager. Keys are the names of each Service.
    children: HashMap<String, GadgetProcess>,
}

impl GadgetProcessManager {
    pub(crate) fn new() -> GadgetProcessManager {
        GadgetProcessManager {
            children: HashMap::new(),
        }
    }

    // /// Store the state of the current processes
    // pub(crate) fn load_state(bytes: Vec<u8>) -> Result<GadgetProcessManager, Box<dyn Error>> {
    //     Ok(serde_json::from_str(from_utf8(&bytes)?)?)
    //     // TODO: Should `check-in` with all processes to see what their statuses are
    // }
    //
    // /// Load the state of previously running processes to recover gadget-executor
    // pub(crate) fn save_state(&self) -> Result<Vec<u8>, Box<dyn Error>> {
    //     Ok(serde_json::to_string(self)?.into_bytes())
    // }

    /// Runs the given command and stores it using the identifier as the key. Returns the identifier used
    pub(crate) async fn run(
        &mut self,
        identifier: String,
        command: &str,
    ) -> Result<String, Box<dyn Error>> {
        let gadget_process = run_command!(command)?;
        self.children.insert(identifier.clone(), gadget_process);
        Ok(identifier)
    }

    /// Removes processes that are no longer running from the manager. Returns a Vector of the names of processes removed
    pub(crate) async fn remove_dead(&mut self) -> Result<Vec<String>, Box<dyn Error>> {
        let mut dead_processes = Vec::new();
        let mut to_remove = Vec::new();
        let s = System::new_all();

        // Find dead processes and gather them for return
        let mut running_processes = s.processes().keys().into_iter();
        for (key, value) in self.children.iter() {
            // Move on if the PID exists and has the correct name
            let current_pid = value.pid;
            if running_processes.contains(&Pid::from_u32(current_pid)) {
                continue;
            }

            // If a different process has taken the expected process' place, we want to restart
            // the intended process without killing the existing one
            if Process::new(current_pid as i32).unwrap().status()?.name == value.process_name {
                // Kill the running process
                let _ = signal::kill(
                    nix::unistd::Pid::from_raw(current_pid as pid_t),
                    Signal::SIGTERM,
                );
                to_remove.push(key.clone());
            }
        }
        self.children.retain(|k, _| !to_remove.contains(k));

        // TODO: If dead children are `supposed` to be running, we should start them up again instead of just removing them

        Ok(dead_processes)
    }

    pub(crate) async fn restart_dead(&mut self) -> Result<Vec<GadgetProcess>, Box<dyn Error>> {
        let mut restarted_processes = Vec::new();
        let mut to_remove = Vec::new();
        // Find dead processes and restart them
        for (key, value) in self.children.iter_mut() {
            match value.status() {
                Ok(status) => {
                    match status {
                        Status::Active | Status::Sleeping => {
                            // TODO: Metrics + Logs for these living processes
                            // Check if this process is still running what is expected
                        }
                        Status::Inactive | Status::Dead | Status::Unknown => {}
                    }
                }
                Err(_err) => {
                    // TODO: Log error
                }
            }
            restarted_processes.push(value.restart_process().await?);
            to_remove.push(key.clone());
        }
        self.children.retain(|k, _| !to_remove.contains(k));

        Ok(restarted_processes)
    }
}
