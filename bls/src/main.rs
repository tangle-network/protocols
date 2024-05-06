use shell_sdk::prelude::*;

pub mod constants;
pub mod protocol;

use protocol::{keygen::BlsKeygenAdditionalParams, signing::BlsSigningAdditionalParams};

generate_protocol!(
    "BLS-Keygen-Protocol",
    BlsKeygenProtocol,
    BlsKeygenAdditionalParams,
    protocol::keygen::generate_protocol_from,
    protocol::keygen::create_next_job,
    jobs::JobType::DKGTSSPhaseOne(_),
    roles::RoleType::Tss(roles::tss::ThresholdSignatureRoleType::GennaroDKGBls381)
);

generate_protocol!(
    "BLS-Signing-Protocol",
    BlsSigningProtocol,
    BlsSigningAdditionalParams,
    protocol::signing::generate_protocol_from,
    protocol::signing::create_next_job,
    jobs::JobType::DKGTSSPhaseTwo(_),
    roles::RoleType::Tss(roles::tss::ThresholdSignatureRoleType::GennaroDKGBls381)
);

generate_setup_and_run_command!(BlsKeygenProtocol, BlsSigningProtocol);

#[cfg(test)]
test_utils::generate_signing_and_keygen_tss_tests!(
    2,
    3,
    2,
    ThresholdSignatureRoleType::GennaroDKGBls381
);

async fn keystore() -> InMemoryBackend {
    InMemoryBackend::default()
}

shell_sdk::generate_shell_binary!(
    setup_node,
    keystore,
    2,
    RoleType::Tss(roles::tss::ThresholdSignatureRoleType::GennaroDKGBls381)
);
