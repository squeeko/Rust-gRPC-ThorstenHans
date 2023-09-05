use tonic::{transport::Server, Request, Response, Status};
use voting::{
    voting_server::{Voting, VotingServer},
    VotingRequest, VotingResponse,
};

pub mod voting {
    tonic::include_proto!("voting");
}

#[derive(Debug, Default)]
pub struct VotingService {}

#[tonic::async_trait]
impl Voting for VotingService {
    async fn vote(
        &self,
        request: Request<VotingRequest>,
    ) -> Result<Response<VotingResponse>, Status> {
        let r = request.into_inner();
        /* In general, Rust methods are named into_something when they consume self, avoiding clones as much as possible, and to_something when they take &self, potentially cloning some data.
         */
        match r.vote {
            0 => Ok(Response::new(voting::VotingResponse {
                confirmation: { format!("Happy for confirm that you upvoted for {}", r.url) },
            })),
            1 => Ok(Response::new(voting::VotingResponse {
                confirmation: { format!("Confirmation that you downvoted for {}", r.url) },
            })),
            _ => Err(Status::new(
                tonic::Code::OutOfRange,
                "Invalid vote provided",
            )),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = "[::1]:8080".parse().unwrap();
    let voting_service = VotingService::default();

    Server::builder()
        .add_service(VotingServer::new(voting_service))
        .serve(address)
        .await?;
    Ok(())
}
