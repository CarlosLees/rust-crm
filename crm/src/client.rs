use anyhow::Result;
use crm::pb::{user_service_client::UserServiceClient, CreateUserRequest};
use tonic::Request;

#[tokio::main]
async fn main() -> Result<()> {
    let mut clinet = UserServiceClient::connect("http://[::1]:50051").await?;

    let request = Request::new(CreateUserRequest {
        name: "carlos".to_string(),
        email: "carlos@163.com".to_string(),
    });

    let response = clinet.create_user(request).await?;
    println!("response: {:?}", response.into_inner());
    Ok(())
}
