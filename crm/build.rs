use std::fs;

use anyhow::Result;
fn main() -> Result<()> {
    fs::create_dir_all("src/pb")?;
    let config = tonic_build::configure();
    config
        .out_dir("src/pb")
        .compile_protos(&["../protos/crm/crm.proto"], &["../protos/crm"])?;
    Ok(())
}
