use crate::protocol::utils::*;
use futures_lite::prelude::*;
use gadget_common::prelude::*;
use gadget_io::tokio;
use protocol::types::roles;
use protocol::*;
use shell_sdk::prelude::*;
use std::time::Duration;

pub mod protocol;

generate_protocol!(
    "Gadget-Executor-Protocol",
    GadgetExecutorProtocol,
    types::GadgetExecutorExtraParams,
    executor::generate_protocol_from,
    executor::create_next_job,
    types::JobType::GadgetExecutorPhaseOne(_),
    // TODO: To be replaced - Changes to the concept of `roles`
    roles::RoleType::executor(roles::executor::CommandExecutor::Docker),
        | roles::RoleType::executor(roles::executor::CommandExecutor::Ping)
);

generate_setup_and_run_command!(GadgetExecutorProtocol);

async fn keystore() -> InMemoryBackend {
    InMemoryBackend::default()
}

shell_sdk::generate_shell_binary!(
    setup_node, keystore,
    2,
    // roles::RoleType::executor(roles::executor::CommandExecutor::Docker),
    // roles::RoleType::executor(roles::executor::CommandExecutor::Ping)
);

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::sleep;

    #[tokio::test]
    #[cfg(target_family = "unix")]
    async fn test_process_command() {
        // Test Discrete Command that returns immediately
        let mut output = run_command!("ls");
        while let Some(line) = output.next_line().await.unwrap() {
            println!("{}", line);
        }

        // Test Continuous Command that does not return immediately
        let mut output = run_command!("ping -c 3 google.com");
        while let Some(line) = output.next_line().await.unwrap() {
            println!("{}", line);
        }

        // Test a Continuous Command with intermittent Discrete commands
        let continuous = async move {
            let mut output = run_command!("ping -c 5 google.com");
            while let Some(line) = output.next_line().await.unwrap() {
                println!("{}", line);
            }
        };
        let discrete = async move {
            for _ in 0..3 {
                sleep(Duration::from_millis(1000)).await;
                let mut output = run_command!("ls");
                while let Some(line) = output.next_line().await.unwrap() {
                    println!("{}", line);
                }
            }
        };
        let fut_vec: Vec<Pin<Box<dyn Future<Output = ()>>>> =
            vec![Box::pin(continuous), Box::pin(discrete)];
        let _ = futures::future::join_all(fut_vec).await;
    }

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
