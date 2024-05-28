use gadget_common::prelude::*;

// /// Message corresponding to any iteration `i` of commands being run
// #[derive(Clone, Serialize, Deserialize)]
// #[serde(bound = "")]
// pub struct Msg {
//     pub msg: Vec<u8>,
//     pub msg_number: u8,
// }

// #[derive(Clone)]
// pub struct GadgetExecutorExtraParams {
//     pub i: u16,
//     // pub t: u16,
//     // pub n: u16,
//     pub job_id: u64,
//     // pub role_type: roles::RoleType,
//     pub user_id_to_account_id_mapping: Arc<HashMap<UserID, ecdsa::Public>>,
// }

// pub async fn create_next_job<C: ClientWithApi, N: Network, KBE: KeystoreBackend>(
//     config: &crate::GadgetExecutorProtocol<C, N, KBE>,
//     job: JobInitMetadata,
//     _work_manager: &ProtocolWorkManager<WorkManager>,
// ) -> Result<GadgetExecutorExtraParams, gadget_common::Error> {
//     let job_id = job.job_id;
//     // let role_type = job.job_type.get_role_type();
//     //
//     // // We can safely make this assumption because we are only creating jobs for phase one
//     // let jobs::JobType::DKGTSSPhaseOne(p1_job) = job.job_type else {
//     //     panic!("Should be valid type")
//     // };
//     //
//     let participants = job.participants_role_ids;
//     // let threshold = p1_job.threshold;
//
//     // let user_id_to_account_id_mapping = Arc::new(
//     //     participants
//     //         .clone()
//     //         .into_iter()
//     //         .enumerate()
//     //         .map(|r| (r.0 as UserID, r.1))
//     //         .collect(),
//     // );
//
//     let id = config.key_store.pair().public();
//
//     let params = GadgetExecutorExtraParams {
//         i: participants
//             .iter()
//             .position(|p| p == &id)
//             .expect("Should exist") as u16,
//         // t: threshold as u16,
//         // n: participants.len() as u16,
//         // role_type,
//         job_id,
//         user_id_to_account_id_mapping,
//     };
//
//     Ok(params)
// }

#[macro_export]
macro_rules! run_shell_command {
    ($cmd:expr) => {{
        let output = std::process::Command::new("sh")
            .arg("-c")
            .arg($cmd)
            .output()
            .expect(&format!("Failed to execute: {:?}", $cmd));

        if !output.status.success() {
            eprintln!("Command failed with status: {}", output.status);
            std::process::exit(output.status.code().unwrap_or(-1));
        }

        std::str::from_utf8(&output.stdout).unwrap().trim().to_owned()
    }};
    ($cmd:expr, $($args:expr),*) => {{
        let mut command = std::process::Command::new($cmd);
        $(
            command.arg($args);
        )*
        let output = command
            .output()
            .expect(&format!("Failed to execute: {} {:?}", $cmd, &[$($args),*]));

        if !output.status.success() {
            eprintln!("Command failed with status: {}", output.status);
            std::process::exit(output.status.code().unwrap_or(-1));
        }

        std::str::from_utf8(&output.stdout).unwrap().trim().to_owned()
    }};
}

pub(crate) use crate::run_shell_command;

// pub async fn generate_protocol_from<C: ClientWithApi, N: Network, KBE: KeystoreBackend>(
//     config: &crate::GadgetExecutorProtocol<C, N, KBE>,
//     associated_block_id: <WorkManager as WorkManagerInterface>::Clock,
//     associated_retry_id: <WorkManager as WorkManagerInterface>::RetryID,
//     associated_session_id: <WorkManager as WorkManagerInterface>::SessionID,
//     associated_task_id: <WorkManager as WorkManagerInterface>::TaskID,
//     protocol_message_channel: UnboundedReceiver<GadgetProtocolMessage>,
//     additional_params: GadgetExecutorExtraParams,
// ) -> Result<BuiltExecutableJobWrapper, JobError> {
//     let key_store = config.key_store.clone();
//     let key_store2 = config.key_store.clone();
//     let protocol_output = Arc::new(gadget_io::tokio::sync::Mutex::new(None));
//     let protocol_output_clone = protocol_output.clone();
//     let pallet_tx = config.pallet_tx.clone();
//     let id = config.key_store.pair().public();
//     let logger = config.logger.clone();
//     let network = config.clone();
//
//     // let (i, t, n, mapping, role_type) = (
//     //     additional_params.i,
//     //     additional_params.t,
//     //     additional_params.n,
//     //     additional_params.user_id_to_account_id_mapping,
//     //     additional_params.role_type.clone(),
//     // );
//     let i = additional_params.i;
//     let mapping = additional_params.user_id_to_account_id_mapping;
//
//     // let role = match role_type {
//     //     roles::RoleType::Tss(role) => role,
//     //     _ => {
//     //         return Err(JobError {
//     //             reason: "Invalid role type".to_string(),
//     //         })
//     //     }
//     // };
//
//     Ok(JobBuilder::new()
//         .protocol(async move {
//             let mut rng = rand::rngs::StdRng::from_entropy();
//             // logger.info(format!(
//             //     "Starting Keygen Protocol with params: i={i}, t={t}, n={n}"
//             // ));
//
//             let (
//                 keygen_tx_to_outbound,
//                 keygen_rx_async_proto,
//                 broadcast_tx_to_outbound,
//                 broadcast_rx_from_gadget,
//             ) = channels::create_job_manager_to_async_protocol_channel_split_io::<
//                 _,
//                 _,
//                 Outgoing<Msg>,
//                 Incoming<Msg>,
//             >(
//                 protocol_message_channel,
//                 associated_block_id,
//                 associated_retry_id,
//                 associated_session_id,
//                 associated_task_id,
//                 mapping.clone(),
//                 id,
//                 network.clone(),
//                 logger.clone(),
//                 i,
//             );
//             let mut tracer = PerfProfiler::new();
//             let delivery = (keygen_rx_async_proto, keygen_tx_to_outbound);
//             let party = round_based_21::MpcParty::connected(delivery);
//
//             // TODO: Run intended commands
//
//
//
//             // let frost_key_share_package = match role {
//             //     roles::tss::ThresholdSignatureRoleType::ZcashFrostEd25519 => {
//             //         run_threshold_keygen!(
//             //             Ed25519Sha512,
//             //             &mut tracer,
//             //             i,
//             //             t,
//             //             n,
//             //             role.clone(),
//             //             &mut rng,
//             //             party
//             //         )
//             //     }
//             //     roles::tss::ThresholdSignatureRoleType::ZcashFrostEd448 => {
//             //         run_threshold_keygen!(
//             //             Ed448Shake256,
//             //             &mut tracer,
//             //             i,
//             //             t,
//             //             n,
//             //             role.clone(),
//             //             &mut rng,
//             //             party
//             //         )
//             //     }
//             //     roles::tss::ThresholdSignatureRoleType::ZcashFrostP256 => {
//             //         run_threshold_keygen!(
//             //             P256Sha256,
//             //             &mut tracer,
//             //             i,
//             //             t,
//             //             n,
//             //             role.clone(),
//             //             &mut rng,
//             //             party
//             //         )
//             //     }
//             //     roles::tss::ThresholdSignatureRoleType::ZcashFrostP384 => {
//             //         run_threshold_keygen!(
//             //             P384Sha384,
//             //             &mut tracer,
//             //             i,
//             //             t,
//             //             n,
//             //             role.clone(),
//             //             &mut rng,
//             //             party
//             //         )
//             //     }
//             //     roles::tss::ThresholdSignatureRoleType::ZcashFrostRistretto255 => {
//             //         run_threshold_keygen!(
//             //             Ristretto255Sha512,
//             //             &mut tracer,
//             //             i,
//             //             t,
//             //             n,
//             //             role.clone(),
//             //             &mut rng,
//             //             party
//             //         )
//             //     }
//             //     roles::tss::ThresholdSignatureRoleType::ZcashFrostSecp256k1 => {
//             //         run_threshold_keygen!(
//             //             Secp256K1Sha256,
//             //             &mut tracer,
//             //             i,
//             //             t,
//             //             n,
//             //             role.clone(),
//             //             &mut rng,
//             //             party
//             //         )
//             //     }
//             //     _ => unreachable!("Invalid role"),
//             // };
//             let perf_report = tracer.get_report().map_err(|err| JobError {
//                 reason: format!("Keygen protocol error: {err:?}"),
//             })?;
//             logger.trace(format!("Incomplete Keygen protocol report: {perf_report}"));
//             logger.debug("Finished AsyncProtocol - Incomplete Keygen");
//
//             // let job_result = handle_public_key_gossip(
//             //     key_store2,
//             //     &logger,
//             //     &frost_key_share_package.verifying_key,
//             //     role.clone(),
//             //     t,
//             //     i,
//             //     broadcast_tx_to_outbound,
//             //     broadcast_rx_from_gadget,
//             // )
//             //     .await?;
//
//             // *protocol_output.lock().await = Some((frost_key_share_package, job_result));
//             Ok(())
//         })
//         .post(async move {
//             // TODO: handle protocol blames
//             // Store the keys locally, as well as submitting them to the blockchain
//             // if let Some((local_key, job_result)) = protocol_output_clone.lock().await.take() {
//             //     key_store
//             //         .set_job_result(additional_params.job_id, local_key)
//             //         .await
//             //         .map_err(|err| JobError {
//             //             reason: format!("Failed to store key: {err:?}"),
//             //         })?;
//             //
//             //     pallet_tx
//             //         .submit_job_result(
//             //             additional_params.role_type,
//             //             additional_params.job_id,
//             //             job_result,
//             //         )
//             //         .await
//             //         .map_err(|err| JobError {
//             //             reason: format!("Failed to submit job result: {err:?}"),
//             //         })?;
//             // }
//
//             Ok(())
//         })
//         .build())
// }