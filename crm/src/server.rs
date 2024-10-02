use anyhow::Result;
use crm::pb::{
    user_service_server::{UserService, UserServiceServer},
    CreateUserRequest, GetUserRequest, User,
};
use tonic::{async_trait, transport::Server, Request, Response, Status};

#[derive(Default)]
struct UserServer;

#[async_trait]
impl UserService for UserServer {
    async fn get_user(&self, request: Request<GetUserRequest>) -> Result<Response<User>, Status> {
        let input = request.into_inner();
        println!("get user:{:?}", input);
        Ok(Response::new(User::default()))
    }

    async fn create_user(
        &self,
        request: tonic::Request<CreateUserRequest>,
    ) -> Result<Response<User>, Status> {
        let input = request.into_inner();
        println!("create user:{:?}", input);
        Ok(Response::new(User::new(1, "carlos", "carlos@123.com")))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "[::1]:50051".parse().unwrap();
    let user_server = UserServer::default();
    println!("user server listening on {}", addr);

    Server::builder()
        .add_service(UserServiceServer::new(user_server))
        .serve(addr)
        .await?;
    Ok(())
}
