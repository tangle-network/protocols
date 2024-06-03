use gadget_common::gadget::message::UserID;
use gadget_common::tangle_runtime::api::runtime_types;
use serde::{Deserialize, Serialize};
use sp_core::ecdsa;
use std::collections::HashMap;
use std::sync::Arc;

/// Message corresponding to any iteration `i` of commands being run
#[derive(Clone, Serialize, Deserialize)]
#[serde(bound = "")]
pub struct Msg {
    pub msg: Vec<u8>,
    pub msg_number: u8,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GadgetExecutorPackage {
    command: String,
    execution_id: u8,
    output: String,
    // pub participants : runtime_types :: bounded_collections :: bounded_vec :: BoundedVec < _0 > ,
    // pub threshold : :: core :: primitive :: u8 ,
    // pub permitted_caller : :: core :: option :: Option < _0 > ,
    // pub role_type : runtime_types :: tangle_primitives :: roles :: tss :: ThresholdSignatureRoleType ,
    // pub hd_wallet : :: core :: primitive :: bool ,
    // # [codec (skip)]
    // pub __ignore : :: core :: marker :: PhantomData < _1 >,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum JobType<_0, _1, _2, _3> {
    GadgetExecutorPhaseOne(
        runtime_types::tangle_primitives::jobs::tss::DKGTSSPhaseOneJobType<_0, _1>,
    ),
    GadgetExecutorPhaseTwo(
        runtime_types::tangle_primitives::jobs::tss::DKGTSSPhaseTwoJobType<_2, _3>,
    ),
}

#[derive(Clone)]
pub struct GadgetExecutorExtraParams {
    pub i: u16,
    // pub t: u16,
    // pub n: u16,
    pub job_id: u64,
    // pub role_type: roles::RoleType,
    pub user_id_to_account_id_mapping: Arc<HashMap<UserID, ecdsa::Public>>,
}

pub mod roles {
    pub mod executor {
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub enum CommandExecutor {
            Ping,
            Docker,
            // #[codec(index = 0)]
            // DfnsCGGMP21Secp256k1,
            // #[codec(index = 1)]
            // DfnsCGGMP21Secp256r1,
            // #[codec(index = 2)]
            // DfnsCGGMP21Stark,
            // #[codec(index = 3)]
            // SilentShardDKLS23Secp256k1,
            // #[codec(index = 4)]
            // ZcashFrostP256,
            // #[codec(index = 5)]
            // ZcashFrostP384,
            // #[codec(index = 6)]
            // ZcashFrostSecp256k1,
            // #[codec(index = 7)]
            // ZcashFrostRistretto255,
            // #[codec(index = 8)]
            // ZcashFrostEd25519,
            // #[codec(index = 9)]
            // ZcashFrostEd448,
            // #[codec(index = 10)]
            // GennaroDKGBls381,
            // #[codec(index = 11)]
            // WstsV2,
        }
    }
    pub enum RoleType {
        Executor(crate::roles::executor::CommandExecutor),
    }
}
