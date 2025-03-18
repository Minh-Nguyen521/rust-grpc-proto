use tonic::{transport::Server, Request, Response, Status};

use hello::greeter_server::{Greeter, GreeterServer};
use hello::{HelloReply, HelloRequest, GoodByeReply, GoodByeRequest};

// we need this to know where to find the generated code
pub mod hello {
    tonic::include_proto!("hello");
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a hello request: {:?}", request);

        let reply = hello::HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };

        Ok(Response::new(reply))
    }
    
    async fn say_goodbye(
        &self,
        request: Request<GoodByeRequest>,
    ) -> Result<Response<GoodByeReply>, Status> {
        println!("Got a goodbye request: {:?}", request);
        
        let reply = hello::GoodByeReply {
            message: format!("Goodbye, {}! See you next time.", request.into_inner().name),
        };
        
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
} 