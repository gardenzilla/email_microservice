use protos::email::email_server;
use protos::email::*;
use std::{path::PathBuf, sync::Mutex};
use storaget::*;
use tonic::{transport::Server, Request, Response, Status};

pub mod check;
mod email;
pub mod prelude;

struct EmailService;

impl EmailService {
    fn new() -> Self {
        EmailService
    }
}

#[tonic::async_trait]
impl email_server::Email for EmailService {
    async fn send_email(&self, request: Request<EmailRequest>) -> Result<Response<()>, Status> {
        todo!()
    }
}

#[tokio::main]
async fn main() -> prelude::ServiceResult<()> {
    let email_service = EmailService::new();

    let addr = "[::1]:50053".parse().unwrap();

    Server::builder()
        .add_service(email_server::EmailServer::new(email_service))
        .serve(addr)
        .await
        .expect("Error while staring server"); // Todo implement ? from<?>

    Ok(())
}
