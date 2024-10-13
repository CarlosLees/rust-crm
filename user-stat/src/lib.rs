use std::pin::Pin;

mod abi;
pub mod pb;

use futures::Stream;
use pb::{
    user_stats_server::{UserStats, UserStatsServer},
    QueryRequest, RawQueryRequest, User,
};
use tonic::{async_trait, Request, Response, Status};

#[derive(Default)]
pub struct UserStatsRpc;

type ServiceResult<T> = Result<Response<T>, Status>;
type ResponseStream = Pin<Box<dyn Stream<Item = Result<User, Status>> + Send>>;

#[async_trait]
impl UserStats for UserStatsRpc {
    type QueryStream = ResponseStream;
    type RawQueryStream = ResponseStream;

    async fn query(&self, request: Request<QueryRequest>) -> ServiceResult<Self::QueryStream> {
        let req = request.into_inner();
        println!("query request:{:?}", req);
        todo!()
    }

    async fn raw_query(
        &self,
        request: Request<RawQueryRequest>,
    ) -> ServiceResult<Self::RawQueryStream> {
        let req = request.into_inner();
        println!("query row request:{:?}", req);
        todo!()
    }
}

pub fn create_service() -> UserStatsServer<UserStatsRpc> {
    let user_stats_rpc = UserStatsRpc::default();
    UserStatsServer::new(user_stats_rpc)
}
