use crate::protocol::utils::*;
use futures_lite::prelude::*;
use gadget_common::prelude::*;
use gadget_io::tokio;
use protocol::types::roles;
use protocol::*;
use shell_sdk::prelude::*;

pub mod protocol;

// TODO: Gadget Setup following V2 - yet to be implemented

#[tokio::main]
async fn main() {
    return;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::executor::run_executor;
    use crate::protocol::process::manager::GadgetProcessManager;
    use tokio::fs::File;
    use tokio::io::AsyncReadExt;

    #[tokio::test]
    async fn test_executor() {
        let the_file = r#"{
            "commands": [
                ["ping_google", "ping -c 5 google.com"],
                ["ping_local", "ping -c 5 localhost"],
                ["sequence_test", ["ping -c 5 localhost", "ls", "clear"]]
            ]
        }"#;
        run_executor(the_file).await;
    }

    #[tokio::test]
    async fn test_loading() {
        // let the_file = r#"{
        //     "commands": [
        //         ["ping_google", "ping -c 5 google.com"],
        //         ["ping_local", "ping -c 5 localhost"],
        //         ["sequence_test", ["ping -c 5 localhost", "ls", "clear"]]
        //     ]
        // }"#;

        let new_manager = GadgetProcessManager::new_from_saved("./savestate.json")
            .await
            .unwrap();

        for (service, child) in new_manager.children {
            println!(
                "Child {} has:\n\t Name: {}\t PID: {}\t Output:{:?}",
                service, child.process_name, child.pid, child.output
            )
        }
    }
}
