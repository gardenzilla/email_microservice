use protos::email::email_server;
use protos::email::*;
use std::sync::mpsc::*;
use std::sync::Mutex;
use std::thread::JoinHandle;
use tonic::{transport::Server, Request, Response, Status};

mod email;
pub mod prelude;

pub struct Email {
    to: String,
    subject: String,
    body: String,
}

impl Email {
    fn new(to: String, subject: String, body: String) -> Self {
        Self { to, subject, body }
    }
}

struct EmailService {
    sender: Mutex<Sender<Email>>,
    _handle: JoinHandle<()>,
}

impl EmailService {
    fn new() -> Self {
        let (tx, rx) = channel::<Email>();
        let handle = std::thread::spawn(move || {
            for email in rx {
                match email.try_send() {
                    _ => (),
                }
            }
        });
        EmailService {
            sender: Mutex::new(tx),
            _handle: handle,
        }
    }
    fn send_email(&self, email: Email) -> prelude::EmailResult<()> {
        match self
            .sender
            .lock()
            .expect("Could not lock sender")
            .send(email)
        {
            Ok(_) => Ok(()),
            Err(_) => Err(prelude::EmailError::internal_error("Email service is down")),
        }
    }
}

#[tonic::async_trait]
impl email_server::Email for EmailService {
    async fn send_email(&self, request: Request<EmailRequest>) -> Result<Response<()>, Status> {
        let req = request.into_inner();
        self.send_email(Email::new(req.to, req.subject, req.body))?;
        Ok(Response::new(()))
    }
}

#[tokio::main]
async fn main() -> prelude::EmailResult<()> {
    let email_service = EmailService::new();

    let addr = "[::1]:50053".parse().unwrap();

    Server::builder()
        .add_service(email_server::EmailServer::new(email_service))
        .serve(addr)
        .await
        .expect("Error while staring server"); // Todo implement ? from<?>

    Ok(())
}
