use std::collections::HashMap;
use std::sync::Arc;
use gadget_common::gadget::message::UserID;
use gadget_common::tangle_runtime::api::runtime_types;
use serde::{Deserialize, Serialize};
use sp_core::ecdsa;

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