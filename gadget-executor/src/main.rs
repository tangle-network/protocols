use crate::protocol::utils::*;
use futures_lite::prelude::*;
use gadget_common::prelude::*;
use gadget_io::tokio;
use protocol::types::roles;
use protocol::*;
use shell_sdk::prelude::*;
use std::time::Duration;
use sysinfo::System;

pub mod protocol;

// TODO: Gadget Setup following V2 - yet to be implemented

use std::{
    process::exit,
    thread::sleep,
};

use nix::{
    sys::wait::waitpid,
    unistd::{fork, ForkResult},
};

#[tokio::main]
async fn main() {
    // Test Continuous Command that does not return immediately
    let s = System::new_all();
    let pid = sysinfo::get_current_pid().unwrap();
    if let Some(process) = s.process(pid) {
        println!("This Process' PID: {} Process Name: {}", pid, process.name());
    }
    let mut output = run_command!("ping -c 30 localhost");
    while let Some(line) = output.next_line().await.unwrap() {
        println!("{}", line);
    }
    tokio::time::sleep(Duration::from_secs(30)).await;
    return;
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::sleep;


    #[tokio::test]
    async fn test_orphan() {
        let s = System::new_all();
        for (pid, process) in s.processes() {
            if process.name().contains("protocol") {
                println!("{} {}", pid, process.name());
            }
        }

        let mut output = run_command!("ping -c 30 localhost");

        for (pid, process) in s.processes() {
            if process.name().contains("protocol") {
                println!("{} {}", pid, process.name());
            }
        }

        while let Some(line) = output.next_line().await.unwrap() {
            println!("{}", line);
        }

        // unsafe {
        //     match fork().expect("Failed to fork process") {
        //         ForkResult::Parent { child } => {
        //             println!("Try to kill me to check if the target process will be killed");
        //
        //             // Do not forget to wait for the fork in order to prevent it from becoming a zombie!!!
        //             waitpid(Some(child), None).unwrap();
        //
        //             sleep(Duration::from_secs(45)).await;
        //         }
        //
        //         ForkResult::Child => {
        //             // This allows the child to outlive the parent
        //             let _new_pid = nix::unistd::setsid();
        //
        //             sleep(Duration::from_secs(30)).await;
        //
        //             let mut output = run_command!("ping -c 5 localhost");
        //             while let Some(line) = output.next_line().await.unwrap() {
        //                 println!("{}", line);
        //             }
        //             exit(0);
        //         }
        //     }
        // }
    }

    // #[tokio::test]
    // #[cfg(target_family = "unix")]
    // async fn test_process_command() {
    //     // Test Discrete Command that returns immediately
    //     let mut output = run_command!("ls");
    //     while let Some(line) = output.next_line().await.unwrap() {
    //         println!("{}", line);
    //     }
    //
    //     // Test Continuous Command that does not return immediately
    //     let mut output = run_command!("ping -c 3 google.com");
    //     while let Some(line) = output.next_line().await.unwrap() {
    //         println!("{}", line);
    //     }
    //
    //     // Test a Continuous Command with intermittent Discrete commands
    //     let continuous = async move {
    //         let mut output = run_command!("ping -c 5 google.com");
    //         while let Some(line) = output.next_line().await.unwrap() {
    //             println!("{}", line);
    //         }
    //     };
    //     let discrete = async move {
    //         for _ in 0..3 {
    //             sleep(Duration::from_millis(1000)).await;
    //             let mut output = run_command!("ls");
    //             while let Some(line) = output.next_line().await.unwrap() {
    //                 println!("{}", line);
    //             }
    //         }
    //     };
    //     let fut_vec: Vec<Pin<Box<dyn Future<Output = ()>>>> =
    //         vec![Box::pin(continuous), Box::pin(discrete)];
    //     let _ = futures::future::join_all(fut_vec).await;
    // }

    #[tokio::test]
    #[cfg(target_family = "windows")]
    async fn test_process_command() {
        // Test Discrete Command that returns immediately
        let mut output = run_command!("dir");
        while let Some(line) = output.next_line().await.unwrap() {
            println!("{}", line);
        }

        // Test Continuous Command that does not return immediately
        let mut output = run_command!("ping -n 3 google.com");
        while let Some(line) = output.next_line().await.unwrap() {
            println!("{}", line);
        }

        // Test a Continuous Command with intermittent Discrete commands
        let continuous = async move {
            let mut output = run_command!("ping -n 5 google.com");
            while let Some(line) = output.next_line().await.unwrap() {
                println!("{}", line);
            }
        };
        let discrete = async move {
            for _ in 0..3 {
                sleep(Duration::from_millis(1000)).await;
                let mut output = run_command!("dir");
                while let Some(line) = output.next_line().await.unwrap() {
                    println!("{}", line);
                }
            }
        };
        let fut_vec: Vec<Pin<Box<dyn Future<Output = ()>>>> =
            vec![Box::pin(continuous), Box::pin(discrete)];
        let _ = futures::future::join_all(fut_vec).await;
    }
}
