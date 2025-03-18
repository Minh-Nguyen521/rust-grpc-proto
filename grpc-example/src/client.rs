use hello::greeter_client::GreeterClient;
use hello::HelloRequest;
use hello::GoodByeRequest;

// we need this to know where to find the generated code
pub mod hello {
    tonic::include_proto!("hello");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    // First request - SayHello
    let hello_request = tonic::Request::new(HelloRequest {
        name: "Rust gRPC".into(),
    });
    let hello_response = client.say_hello(hello_request).await?;
    println!("HELLO RESPONSE = {:?}", hello_response);

    // Second request - SayGoodbye
    let goodbye_request = tonic::Request::new(GoodByeRequest {
        name: "Rust gRPC".into(),
    });
    let goodbye_response = client.say_goodbye(goodbye_request).await?;
    println!("GOODBYE RESPONSE = {:?}", goodbye_response);

    Ok(())
} 