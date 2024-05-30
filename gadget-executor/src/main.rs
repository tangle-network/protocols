use std::error;
use std::time::Duration;
use gadget_common::prelude::*;
use gadget_common::tangle_runtime::api::runtime_types;
use gadget_common::tangle_subxt::subxt;
use shell_sdk::prelude::*;
use protocol::*;
use async_process::{Command, Stdio};
use futures_lite::{io::BufReader, prelude::*};
use gadget_io::tokio;
use crate::protocol::macros::*;

pub mod protocol;

// generate_protocol!(
//     "Gadget-Executor-Protocol",
//     GadgetExecutorProtocol,
//     types::GadgetExecutorExtraParams,
//     executor::generate_protocol_from,
//     executor::create_next_job,
//     types::JobType::GadgetExecutorPhaseOne(_),
//     roles::RoleType::Tss(roles::tss::ThresholdSignatureRoleType::ZcashFrostEd25519)
//         | roles::RoleType::Tss(roles::tss::ThresholdSignatureRoleType::ZcashFrostEd448)
//         | roles::RoleType::Tss(roles::tss::ThresholdSignatureRoleType::ZcashFrostP256)
//         | roles::RoleType::Tss(roles::tss::ThresholdSignatureRoleType::ZcashFrostP384)
//         | roles::RoleType::Tss(roles::tss::ThresholdSignatureRoleType::ZcashFrostSecp256k1)
//         | roles::RoleType::Tss(roles::tss::ThresholdSignatureRoleType::ZcashFrostRistretto255)
// );
//
// generate_setup_and_run_command!(GadgetExecutorProtocol);
//
// async fn keystore() -> InMemoryBackend {
//     InMemoryBackend::default()
// }
//
// shell_sdk::generate_shell_binary!(
//     setup_node,
//     keystore,
//     2,
//     // roles::RoleType::Tss(roles::tss::ThresholdSignatureRoleType::ZcashFrostEd25519),
//     // roles::RoleType::Tss(roles::tss::ThresholdSignatureRoleType::ZcashFrostEd448),
//     // roles::RoleType::Tss(roles::tss::ThresholdSignatureRoleType::ZcashFrostP256),
//     // roles::RoleType::Tss(roles::tss::ThresholdSignatureRoleType::ZcashFrostP384),
//     // roles::RoleType::Tss(roles::tss::ThresholdSignatureRoleType::ZcashFrostSecp256k1),
//     // roles::RoleType::Tss(roles::tss::ThresholdSignatureRoleType::ZcashFrostRistretto255)
// );

// Main for testing
#[tokio::main]
async fn main() {
    let output = run_discrete_command!("dir");
    println!("{output}");
    let output = run_discrete_command!("ls");
    println!("{output}");
    let output = run_discrete_command!("echo TESTING MAIN");
    println!("{output}");
    let output = run_discrete_command!("ping", "-n", "2", "google.com");
    println!("{output}");
    return;
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn run_async() -> Result<(), Box<dyn error::Error>> {
        let mut reader = run_continuous_command!("ping", "google.com");
        let (tx, rx) = tokio::sync::oneshot::channel::<()>();

        let read_block = async move {
            while let Some(line) = reader.next().await {
                let result = line.unwrap_or("ERROR IN CONTINUOUS CHILD".to_string());
                println!("{result}");
                if result.contains("seq=5") {
                    break;
                }
            }
            let _ = tx.send(());
            while let Some(line) = reader.next().await {
                let result = line.unwrap_or("ERROR IN CONTINUOUS CHILD".to_string());
                println!("{result}");
                if result.contains("seq=10") {
                    break;
                }
            }
            return;
        };

        tokio::select! {
        _ = read_block => { },
        _ = rx => {
            println!("RUNNING DISCRETE COMMAND IN ADDITION..");
            let output = run_discrete_command!("echo TESTING MAIN");
            println!("{output}");
        }
    }

        Ok(())
    }

    #[tokio::test]
    async fn test_async_monitoring() {
        run_async().await.unwrap();
    }

    #[test]
    #[cfg(target_family = "unix")]
    fn test_process_command() {
        let output = run_discrete_command!("dir");
        println!("{output}");
        let output = run_discrete_command!("ls");
        println!("{output}");
        let output = run_discrete_command!("ls", "-al");
        println!("{output}");
        let output = run_discrete_command!("echo TESTING MAIN");
        println!("{output}");
        let output = run_discrete_command!("ping", "-c", "2", "google.com");
        println!("{output}");
    }

    #[test]
    #[cfg(target_family = "windows")]
    fn test_process_command() {
        let output = run_discrete_command!("dir");
        println!("{output}");
        let output = run_discrete_command!("dir", "/AD");
        println!("{output}");
        let output = run_discrete_command!("echo TESTING MAIN");
        println!("{output}");
        let output = run_discrete_command!("ping", "-n", "2", "google.com");
        println!("{output}");
    }
}
