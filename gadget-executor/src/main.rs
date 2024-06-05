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

    #[tokio::test]
    async fn test_executor() {
        let the_file = r#"{
            "commands": [
                ["ping_google", "ping -c 5 google.com"],
                ["ping_local", ["ping -c 5 localhost", "ls", "clear"]]
            ]
        }"#;
        run_executor(the_file).await;
    }
}
