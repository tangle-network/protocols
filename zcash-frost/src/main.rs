use zcash_frost::*;
use shell_sdk::prelude::*;
use shell_sdk::gadget_io::tokio;

shell_sdk::generate_shell_binary!(
    setup_node,
    keystore,
    2,
    roles::RoleType::Tss(roles::tss::ThresholdSignatureRoleType::ZcashFrostEd25519),
    roles::RoleType::Tss(roles::tss::ThresholdSignatureRoleType::ZcashFrostEd448),
    roles::RoleType::Tss(roles::tss::ThresholdSignatureRoleType::ZcashFrostP256),
    roles::RoleType::Tss(roles::tss::ThresholdSignatureRoleType::ZcashFrostP384),
    roles::RoleType::Tss(roles::tss::ThresholdSignatureRoleType::ZcashFrostSecp256k1),
    roles::RoleType::Tss(roles::tss::ThresholdSignatureRoleType::ZcashFrostRistretto255)
);