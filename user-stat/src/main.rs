use anyhow::Result;
use tonic::transport::Server;
use user_stat::create_service;

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "[::1]:50051".parse().unwrap();
    println!("user server listening on {}", addr);

    let service = create_service();
    Server::builder().add_service(service).serve(addr).await?;
    Ok(())
}
