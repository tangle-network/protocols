use crate::process::types::GadgetInstructionData;
use crate::protocol::process::manager::GadgetProcessManager;
use crate::protocol::process::types::CommandOrSequence;

pub async fn run_executor(instructions: &str) {
    // TODO: New commands should be received real-time from connection - will follow new V2 structure
    let mut manager = GadgetProcessManager::new();
    let instructions: GadgetInstructionData =
        serde_json::from_str(instructions).expect("JSON was not well-formatted");

    // Execute all the required commands
    for to_execute in instructions.commands {
        let name = to_execute.name;
        let commands = to_execute.command;
        match commands {
            CommandOrSequence::Command(cmd) => {
                println!("Executing single command {:?}: {:?}", name, cmd);
                match manager.run(name, &cmd).await {
                    Ok(_identifier) => {
                        // Logs/Metrics
                    }
                    Err(_error) => {
                        // Error logging
                    }
                }
            }
            CommandOrSequence::Sequence(cmds) => {
                println!("Executing sequence of commands {:?}: {:?}", name, cmds);
                // TODO: Utilizing the output of commands for new commands
            }
        }
    }
}
