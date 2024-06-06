use crate::process::types::GadgetInstructionData;
use crate::protocol::process::manager::GadgetProcessManager;
use crate::protocol::process::types::{CommandOrSequence, ProcessOutput};

pub async fn run_executor(instructions: &str) {
    // TODO: New commands should be received real-time from connection - will follow new V2 structure
    let mut manager = GadgetProcessManager::new();
    let instructions: GadgetInstructionData =
        serde_json::from_str(instructions).expect("JSON was not well-formatted");

    // Execute all the required commands
    for to_execute in instructions.commands {
        let identifier = to_execute.name;
        let commands = to_execute.command;
        match commands {
            CommandOrSequence::Command(cmd) => {
                println!("Executing single command {:?}: {:?}", identifier, cmd);
                match manager.run(identifier, &cmd).await {
                    Ok(_identifier) => {
                        // Logs/Metrics
                    }
                    Err(_error) => {
                        // Error logging
                    }
                }
            }
            CommandOrSequence::Sequence(cmds) => {
                println!(
                    "Executing sequence of commands {:?}: {:?}",
                    identifier, cmds
                );
                // TODO: Utilizing the output of commands for new commands
            }
        }
    }

    // Receive input from all running processes
    let mut ended = Vec::new();
    for (service, mut process) in &mut manager.children {
        println!("LOG : Process {}", service.clone());
        let status = process.status().unwrap();
        println!("\tSTATUS: {:?}", status);
        let output = process.read().await;
        println!("\n{} read result:\n\t {:?}\n", service, output);
        if let ProcessOutput::Exhausted(_) = output {
            println!("{} ended, removing from Manager", service);
            ended.push(service.clone());
        }
    }
    manager.children.retain(|k, _| !ended.contains(k));

    println!(
        " ENDED EXECUTOR WITH GADGET PROCESS MANAGER:\n{:?}\n",
        manager
    );
}
