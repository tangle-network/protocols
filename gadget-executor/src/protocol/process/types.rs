use crate::protocol::utils::*;
use crate::{craft_child_process, run_command, OS_COMMAND};
use failure::format_err;
use nix::libc::pid_t;
use nix::sys::signal;
use nix::sys::signal::Signal;
use procfs::process::Process;
use serde::{Deserialize, Serialize};
use std::error::Error;
use tokio::io::{BufReader, Lines};
pub use tokio::process::Child;

/// A Process spawned by gadget-executor, running some service or command(s)
pub(crate) struct GadgetProcess {
    /// The command executed
    pub(crate) command: String,
    /// The name of the process itself
    pub(crate) process_name: String,
    /// Process ID
    pub(crate) pid: u32,
    /// History of output from process for reviewing/tracking progress
    pub(crate) output: Vec<String>,
    /// Stream for output from child process
    pub(crate) stream: Lines<BufReader<tokio::process::ChildStdout>>,
}

impl GadgetProcess {
    pub fn new(
        command: String,
        pid: Option<u32>,
        output: Vec<String>,
        stream: Lines<BufReader<tokio::process::ChildStdout>>,
    ) -> Result<GadgetProcess, Box<dyn Error>> {
        let process_name = Process::new(pid.ok_or("PID does not exist")? as i32)?
            .status()?
            .name;
        let pid = pid.ok_or("PID does not exist")?;
        Ok(GadgetProcess {
            command,
            process_name,
            pid,
            output,
            stream,
        })
    }

    /// Restart a GadgetProcess, killing the previously running process if it exists. Returns the new GadgetProcess
    pub(crate) async fn restart_process(&mut self) -> Result<GadgetProcess, Box<dyn Error>> {
        // Kill current process running this command
        let status = Process::new(self.pid as i32)?.status()?;
        if status.name == self.process_name {
            self.kill()?;
        }
        run_command!(&self.command.clone())
    }

    /// Checks the status of this GadgetProcess
    pub(crate) fn status(&self) -> Result<Status, Box<dyn Error>> {
        let status = Process::new(self.pid as i32)?.status()?;
        Ok(Status::try_from(status.state)?)
    }

    /// Gets process name by PID
    pub(crate) fn get_name(&self) -> Result<String, Box<dyn Error>> {
        let status = Process::new(self.pid as i32)?.status()?;
        Ok(status.name)
    }

    /// Terminates the process depicted by this GadgetProcess - will fail if the PID is now being reused
    pub(crate) fn kill(&self) -> Result<(), Box<dyn Error>> {
        let running_process = Process::new(self.pid as i32)?.status()?.name;
        if running_process == self.process_name {
            Ok(signal::kill(
                nix::unistd::Pid::from_raw(self.pid as pid_t),
                Signal::SIGTERM,
            )?)
        } else {
            Err(Box::from(format_err!(
                "Expected {} and found {} running instead - process termination aborted",
                self.process_name,
                running_process
            )))
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
    /// Unknown or invalid status - if this occurs, something went wrong
    Unknown,
}

impl From<String> for Status {
    fn from(value: String) -> Status {
        match value.as_str() {
            "R" => Status::Active,
            "S" | "D" => Status::Sleeping,
            "T" => Status::Inactive,
            "Z" => Status::Dead,
            _ => Status::Unknown,
        }
    }
}

// impl TryFrom<String> for Status {
//     type Error = &'static Box<dyn Error>;
//     fn try_from(value: String) -> Result<Self, Self::Error> {
//         match value.as_str() {
//             "R" => { Ok(Status::Active) },
//             "S" | "D" => { Ok(Status::Sleeping) },
//             "T" => { Ok(Status::Inactive) },
//             "Z" => { Ok(Status::Dead) },
//             _ => { Err(format!("Status Error: {} is an invalid status", value)) },
//         }
//     }
// }

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct GadgetInstructionData {
    pub(crate) commands: Vec<CommandData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct CommandData {
    pub(crate) name: String,
    pub(crate) command: CommandOrSequence,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub(crate) enum CommandOrSequence {
    Command(String),
    Sequence(Vec<String>),
}
