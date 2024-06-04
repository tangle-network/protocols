use std::collections::HashMap;
use std::error::Error;
use std::str::from_utf8;
use itertools::Itertools;
use nix::sys::signal;
use nix::sys::signal::Signal;
use procfs::process::Process;
use sysinfo::{Pid, System};
use serde::{Serialize, Deserialize};
use crate::protocol::utils::*;
use crate::run_command;
use crate::craft_child_process;
use crate::process::types::{GadgetProcess, Status};

/// Manager for gadget-executor process. The process are recorded to be controlled by their Service name.
/// This Manager can be reconstructed from a file to recover a gadget-executor.
#[derive(Serialize, Deserialize, Debug)]
struct GadgetProcessManager {
    /// Hashmap that contains all the children spawned by this Manager. Keys are the names of each Service.
    children: HashMap<String, (GadgetProcess, Lines<BufReader<tokio::process::ChildStdout>>)>,
}

impl GadgetProcessManager {
    pub(crate) fn new() -> GadgetProcessManager {
        GadgetProcessManager{
            children: HashMap::new(),
        }
    }

    pub(crate) fn from_json(bytes: Vec<u8>) -> Result<GadgetProcessManager, Box<dyn Error>> {
        Ok(serde_json::from_str(from_utf8(&bytes)?)?)
        // TODO: Should `check-in` with all processes to see what their statuses are
    }

    pub(crate) fn to_json(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(serde_json::to_string(self)?.into_bytes())
    }

    pub(crate) async fn run(&self, command: &str) {
        let _gadget_process = run_command!(command);
    }

    /// Removes processes that are no longer running from the manager. Returns a Vector of the processes removed
    pub(crate) async fn remove_dead(&mut self) -> Vec<GadgetProcess> {
        let mut dead_processes = Vec::new();
        let mut to_remove = Vec::new();
        let s = System::new_all();

        // Find dead processes and gather them for return
        let mut running_processes = s.processes().keys().into_iter();
        for (key, value) in &self.children {
            // Move on if the PID exists and has the correct name
            if value.0.pid.is_some() {
                let current_pid = value.0.pid.unwrap();
                if (running_processes.contains(&Pid::from_u32(current_pid))) {
                    continue;
                }

                // If a different process has taken the expected process' place, we want to restart
                // the intended process without killing the existing one
                if Process::new(current_pid as i32).unwrap().status()?.name == value.0.process_name {
                    to_remove.push(key.clone());
                    // Kill the running process
                    let _ = signal::kill(nix::unistd::Pid::from_u32(current_pid), Signal::SIGTERM);

                }
            }
            dead_processes.push(value.0.clone());
        }

        // TODO: If dead children are `supposed` to be running, we should start them up again instead of just removing them

        // Removes the dead processes
        let _ = to_remove.iter().map(|s| self.children.remove(s));

        dead_processes
    }

    pub(crate) async fn restart_dead(&mut self) -> Result<Vec<GadgetProcess>, Box<dyn Error>> {
        let mut restarted_processes = Vec::new();
        // Find dead processes and restart them
        for (_key, value) in &self.children {
            match self.status(value.0.pid) {
                Ok(status) => {
                    match status {
                        Status::Active
                            | Status::Sleeping => {
                            // TODO: Metrics + Logs for these living processes
                        }
                        Status::Inactive
                            | Status::Dead
                            | Status::Unknown => {
                            restarted_processes.push(self.restart_process(value.0.pid).await);
                        }
                    }
                }
                Err(_err) => {
                    // TODO: Log error
                    restarted_processes.push(self.restart_process(value.0.pid).await);
                }
            }
            // if !(s.processes().keys().into_iter().contains(&Pid::from_u32(value.pid))) {
            //     dead_processes.push(value.clone());
            //     to_remove.push(key.clone());
            // }
        }

        Ok(restarted_processes)
    }

    pub(crate) async fn restart_process(&mut self, pid: u32) -> GadgetProcess {

    }

    /// Checks the status of a running service by PID
    pub(crate) fn status(&self, service: u32) -> Result<Status, Box<dyn Error>> {
        let status = Process::new(service as i32)?.status()?;
        Ok(Status::try_from(status.state)?)
    }


}