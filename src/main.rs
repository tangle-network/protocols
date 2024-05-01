use gadget_common::prelude::*;
use gadget_common::ExecutableJob;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    JobBuilder::default()
        .catch(async move { unreachable!("This should not be called") })
        .build()
        .execute()
        .await?;

    Ok(())
}
