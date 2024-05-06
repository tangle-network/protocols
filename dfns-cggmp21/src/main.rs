use crate::protocols::key_refresh::DfnsCGGMP21KeyRefreshExtraParams;
use crate::protocols::key_rotate::DfnsCGGMP21KeyRotateExtraParams;
use crate::protocols::keygen::DfnsCGGMP21KeygenExtraParams;
use crate::protocols::sign::DfnsCGGMP21SigningExtraParams;
use shell_sdk::prelude::*;

pub mod constants;
pub mod error;
pub mod protocols;

generate_protocol!(
    "DFNS-Keygen-Protocol",
    DfnsKeygenProtocol,
    DfnsCGGMP21KeygenExtraParams,
    protocols::keygen::generate_protocol_from,
    protocols::keygen::create_next_job,
    jobs::JobType::DKGTSSPhaseOne(_),
    roles::RoleType::Tss(roles::tss::ThresholdSignatureRoleType::DfnsCGGMP21Secp256k1)
        | roles::RoleType::Tss(roles::tss::ThresholdSignatureRoleType::DfnsCGGMP21Secp256r1)
        | roles::RoleType::Tss(roles::tss::ThresholdSignatureRoleType::DfnsCGGMP21Stark)
);
generate_protocol!(
    "DFNS-Signing-Protocol",
    DfnsSigningProtocol,
    DfnsCGGMP21SigningExtraParams,
    protocols::sign::generate_protocol_from,
    protocols::sign::create_next_job,
    jobs::JobType::DKGTSSPhaseTwo(_),
    roles::RoleType::Tss(roles::tss::ThresholdSignatureRoleType::DfnsCGGMP21Secp256k1)
        | roles::RoleType::Tss(roles::tss::ThresholdSignatureRoleType::DfnsCGGMP21Secp256r1)
        | roles::RoleType::Tss(roles::tss::ThresholdSignatureRoleType::DfnsCGGMP21Stark)
);
generate_protocol!(
    "DFNS-Refresh-Protocol",
    DfnsKeyRefreshProtocol,
    DfnsCGGMP21KeyRefreshExtraParams,
    protocols::key_refresh::generate_protocol_from,
    protocols::key_refresh::create_next_job,
    jobs::JobType::DKGTSSPhaseThree(_),
    roles::RoleType::Tss(roles::tss::ThresholdSignatureRoleType::DfnsCGGMP21Secp256k1)
        | roles::RoleType::Tss(roles::tss::ThresholdSignatureRoleType::DfnsCGGMP21Secp256r1)
        | roles::RoleType::Tss(roles::tss::ThresholdSignatureRoleType::DfnsCGGMP21Stark)
);
generate_protocol!(
    "DFNS-Rotate-Protocol",
    DfnsKeyRotateProtocol,
    DfnsCGGMP21KeyRotateExtraParams,
    protocols::key_rotate::generate_protocol_from,
    protocols::key_rotate::create_next_job,
    jobs::JobType::DKGTSSPhaseFour(_),
    roles::RoleType::Tss(roles::tss::ThresholdSignatureRoleType::DfnsCGGMP21Secp256k1)
        | roles::RoleType::Tss(roles::tss::ThresholdSignatureRoleType::DfnsCGGMP21Secp256r1)
        | roles::RoleType::Tss(roles::tss::ThresholdSignatureRoleType::DfnsCGGMP21Stark)
);

generate_setup_and_run_command!(
    DfnsKeygenProtocol,
    DfnsSigningProtocol,
    DfnsKeyRefreshProtocol,
    DfnsKeyRotateProtocol
);

#[cfg(test)]
mod secp256k1 {
    test_utils::generate_signing_and_keygen_tss_tests!(
        2,
        3,
        4,
        ThresholdSignatureRoleType::DfnsCGGMP21Secp256k1
    );
}

#[cfg(test)]
mod secp256r1 {
    test_utils::generate_signing_and_keygen_tss_tests!(
        2,
        3,
        4,
        ThresholdSignatureRoleType::DfnsCGGMP21Secp256r1
    );
}

#[cfg(test)]
mod stark {
    test_utils::generate_signing_and_keygen_tss_tests!(
        2,
        3,
        4,
        ThresholdSignatureRoleType::DfnsCGGMP21Stark
    );
}

async fn keystore() -> InMemoryBackend {
    InMemoryBackend::default()
}

shell_sdk::generate_shell_binary!(
    setup_node,
    keystore,
    4,
    RoleType::Tss(roles::tss::ThresholdSignatureRoleType::DfnsCGGMP21Secp256k1),
    RoleType::Tss(roles::tss::ThresholdSignatureRoleType::DfnsCGGMP21Secp256r1),
    RoleType::Tss(roles::tss::ThresholdSignatureRoleType::DfnsCGGMP21Stark)
);

#[cfg(test)]
mod derivation {
    use tangle_primitives::jobs::{
        DKGTSSPhaseOneJobType, DKGTSSPhaseTwoJobType, JobId, JobSubmission, JobType,
    };
    use tangle_primitives::roles::{RoleType, ThresholdSignatureRoleType};
    use tangle_primitives::AccountId;
    use test_utils::mock::{id_to_sr25519_public, Jobs, RuntimeOrigin};
    use test_utils::sync::substrate_test_channel::MultiThreadedTestExternalities;

    const N: usize = 3;
    const T: usize = 2;

    #[tokio::test(flavor = "multi_thread")]
    async fn signing_with_derivation_secp256k1() {
        let threshold_sig_ty = ThresholdSignatureRoleType::DfnsCGGMP21Secp256k1;
        signing_with_derivation(threshold_sig_ty).await;
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn signing_with_derivation_secp256r1() {
        let threshold_sig_ty = ThresholdSignatureRoleType::DfnsCGGMP21Secp256r1;
        signing_with_derivation(threshold_sig_ty).await;
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn signing_with_derivation_stark() {
        let threshold_sig_ty = ThresholdSignatureRoleType::DfnsCGGMP21Stark;
        signing_with_derivation(threshold_sig_ty).await;
    }

    async fn signing_with_derivation(threshold_sig_ty: ThresholdSignatureRoleType) {
        test_utils::setup_log();

        let ext = new_test_ext::<N>().await;
        let keygen_job_id = wait_for_keygen::<N, T>(&ext, threshold_sig_ty).await;
        assert_eq!(
            wait_for_signing::<N>(&ext, keygen_job_id, threshold_sig_ty).await,
            1
        );
    }

    async fn wait_for_keygen<const N: usize, const T: usize>(
        ext: &MultiThreadedTestExternalities,
        threshold_sig_ty: ThresholdSignatureRoleType,
    ) -> JobId {
        let job_id = ext
            .execute_with_async(move || {
                let job_id = Jobs::next_job_id();
                let identities = (0..N)
                    .map(|i| id_to_sr25519_public(i as u8))
                    .map(AccountId::from)
                    .collect::<Vec<_>>();

                let submission = JobSubmission {
                    fallback: tangle_primitives::jobs::FallbackOptions::Destroy,
                    expiry: 100,
                    ttl: 100,
                    job_type: JobType::DKGTSSPhaseOne(DKGTSSPhaseOneJobType {
                        participants: identities.clone().try_into().unwrap(),
                        threshold: T as _,
                        permitted_caller: None,
                        hd_wallet: true,
                        role_type: threshold_sig_ty,
                    }),
                };

                assert!(
                    Jobs::submit_job(RuntimeOrigin::signed(identities[0].clone()), submission)
                        .is_ok()
                );

                log::info!(target: "gadget", "******* Submitted Keygen Job {job_id}");
                job_id
            })
            .await;

        test_utils::wait_for_job_completion(ext, RoleType::Tss(threshold_sig_ty), job_id).await;
        job_id
    }

    async fn wait_for_signing<const N: usize>(
        ext: &MultiThreadedTestExternalities,
        keygen_job_id: JobId,
        threshold_sig_ty: ThresholdSignatureRoleType,
    ) -> JobId {
        let job_id = ext
            .execute_with_async(move || {
                let job_id = Jobs::next_job_id();
                let identities = (0..N)
                    .map(|i| id_to_sr25519_public(i as u8))
                    .map(AccountId::from)
                    .collect::<Vec<_>>();
                let submission = JobSubmission {
                    fallback: tangle_primitives::jobs::FallbackOptions::Destroy,
                    expiry: 100,
                    ttl: 100,
                    job_type: JobType::DKGTSSPhaseTwo(DKGTSSPhaseTwoJobType {
                        phase_one_id: keygen_job_id,
                        derivation_path: Some(
                            String::from("m/44'/60'/0'/0/0")
                                .as_bytes()
                                .to_vec()
                                .try_into()
                                .unwrap(),
                        ),
                        submission: Vec::from("Hello, world!").try_into().unwrap(),
                        role_type: threshold_sig_ty,
                    }),
                };

                assert!(
                    Jobs::submit_job(RuntimeOrigin::signed(identities[0].clone()), submission)
                        .is_ok()
                );

                log::info!(target: "gadget", "******* Submitted Signing Job {job_id}");
                job_id
            })
            .await;

        test_utils::wait_for_job_completion(ext, RoleType::Tss(threshold_sig_ty), job_id).await;
        job_id
    }

    async fn new_test_ext<const N: usize>() -> MultiThreadedTestExternalities {
        test_utils::mock::new_test_ext::<N, 4, (), _, _>((), crate::setup_node).await
    }
}
