use std::collections::HashMap;
use std::error::Error;
use std::str::from_utf8;
// use std::str::from_utf8;
use crate::process::types::{GadgetProcess, Status};
use crate::protocol::utils::*;
use crate::{craft_child_process, run_command, OS_COMMAND};
use itertools::Itertools;
use nix::libc::pid_t;
use nix::sys::signal;
use nix::sys::signal::Signal;
use procfs::process::Process;
use serde::{Deserialize, Serialize};
use sysinfo::{Pid, System};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

/// Manager for gadget-executor process. The processes are recorded to be controlled by their Service name.
/// This Manager can be reconstructed from a file to recover a gadget-executor.
#[derive(Serialize, Deserialize, Debug)]
pub struct GadgetProcessManager {
    /// Hashmap that contains all the children spawned by this Manager. Keys are the names of each Service.
    pub children: HashMap<String, GadgetProcess>,
}

impl GadgetProcessManager {
    pub(crate) fn new() -> GadgetProcessManager {
        GadgetProcessManager {
            children: HashMap::new(),
        }
    }

    /// Load the state of previously running processes to recover gadget-executor
    pub(crate) async fn new_from_saved(file: &str) -> Result<GadgetProcessManager, Box<dyn Error>> {
        let file = std::fs::File::open(file).unwrap();
        let mut new_manager: GadgetProcessManager = serde_json::from_reader(file).unwrap();

        // Restarts processes that were previously running
        new_manager.restart_dead().await?;

        Ok(new_manager)
    }

    /// Store the state of the current processes
    pub(crate) async fn save_state(&self) -> Result<String, Box<dyn Error>> {
        let serialized_data = serde_json::to_string(self)?;
        let mut file = File::create("./savestate.json").await?;
        file.write_all(serialized_data.clone().as_bytes()).await?;
        Ok(serialized_data)
    }

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

    /// Finds all dead processes that still exist in map and starts them again. This function
    /// is used to restart all processes after loading a Manager from a file.
    pub(crate) async fn restart_dead(&mut self) -> Result<(), Box<dyn Error>> {
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
                            // If it is still correctly running, we just move along
                            continue;
                        }
                        Status::Inactive | Status::Dead | Status::Unknown => {
                            // Dead, should be restarted
                        }
                    }
                }
                Err(err) => {
                    // TODO: Log error
                    // Error generally means it died and no longer exists - restart it
                    println!(
                        "LOG : {} yielded {} while attempting to restart dead processes",
                        key.clone(),
                        err
                    );
                }
            }
            restarted_processes.push((key.clone(), value.restart_process().await?));
            to_remove.push(key.clone());
        }
        self.children.retain(|k, _| !to_remove.contains(k));
        for (service, restarted) in restarted_processes {
            self.children.insert(service.clone(), restarted);
        }

        Ok(())
    }
}
