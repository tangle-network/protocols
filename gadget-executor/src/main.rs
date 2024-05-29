use gadget_common::prelude::*;
use gadget_common::tangle_runtime::api::runtime_types;
use gadget_common::tangle_subxt::subxt;
use shell_sdk::prelude::*;
use crate::protocol::executor::*;

pub mod protocol;

// generate_protocol!(
//     "Gadget-Executor-Protocol",
//     GadgetExecutorProtocol,
//     GadgetExecutorExtraParams,
//     crate::protocol::executor::generate_protocol_from,
//     crate::protocol::executor::create_next_job,
//     jobs::JobType::DKGTSSPhaseOne(_),
//     // roles::RoleType::Tss(roles::tss::ThresholdSignatureRoleType::ZcashFrostEd25519)
//     //     | roles::RoleType::Tss(roles::tss::ThresholdSignatureRoleType::ZcashFrostEd448)
//     //     | roles::RoleType::Tss(roles::tss::ThresholdSignatureRoleType::ZcashFrostP256)
//     //     | roles::RoleType::Tss(roles::tss::ThresholdSignatureRoleType::ZcashFrostP384)
//     //     | roles::RoleType::Tss(roles::tss::ThresholdSignatureRoleType::ZcashFrostSecp256k1)
//     //     | roles::RoleType::Tss(roles::tss::ThresholdSignatureRoleType::ZcashFrostRistretto255)
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

fn main() {
    let output = crate::protocol::executor::run_shell_command!("dir");
    println!("{output}");
    let output = crate::protocol::executor::run_shell_command!("ls");
    println!("{output}");
    let output = crate::protocol::executor::run_shell_command!("echo TESTING MAIN");
    println!("{output}");
    let output = crate::protocol::executor::run_shell_command!("ping", "-n", "2", "google.com");
    println!("{output}");
    // let output = crate::protocol::executor::run_shell_command!("docker run hello-world");
    // println!("{output:?}");
    return;
}

#[cfg(test)]
mod tests{
    #[test]
    fn test_process_command() {
        let output = crate::protocol::executor::run_shell_command!("echo Hello World!");
        println!("{output}");
        let output = crate::protocol::executor::run_shell_command!("ls", "-al");
        println!("{output}");
        let output = crate::protocol::executor::run_shell_command!("ls");
        println!("{output}");
    }
}