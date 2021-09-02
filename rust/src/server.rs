use hello_world::hello_service_server::{HelloService, HelloServiceServer};
use hello_world::{HelloRequest, HelloResponse};
use helloworld_tonic::hello_world;
use tonic::{transport::Server, Request, Response, Status};

#[derive(Debug, Default)]
pub struct Greeter {}

#[tonic::async_trait]
impl HelloService for Greeter {
    async fn hello(
        &self,
        request: Request<HelloRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<HelloResponse>, Status> {
        // Return an instance of type HelloReply
        println!("Got a request: {:?}", request);
        let message = request.into_inner();
        let reply = hello_world::HelloResponse {
            greeting: format!("Hello {} {}!", message.first_name, message.last_name),
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:8080".parse()?;
    let greeter = Greeter::default();
    println!("Server starting @ {} with service {:#?}", addr, greeter);

    Server::builder()
        .add_service(HelloServiceServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
