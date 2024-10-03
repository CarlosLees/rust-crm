use std::fs;

use anyhow::Result;
fn main() -> Result<()> {
    fs::create_dir_all("src/pb")?;
    let config = tonic_build::configure();
    config.out_dir("src/pb").compile_protos(
        &[
            "../protos/user_stats/messages.proto",
            "../protos/user_stats/rpc.proto",
        ],
        &["../protos/user_stats"],
    )?;
    Ok(())
}
