use hello_world::hello_service_client::HelloServiceClient;
use hello_world::HelloRequest;
use helloworld_tonic::hello_world;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = HelloServiceClient::connect("http://[::1]:8080").await?;

    let request = tonic::Request::new(HelloRequest {
        first_name: "Leonardo".to_string(),
        last_name: "Razovic".to_string(),
    });

    let response = client.hello(request).await?;

    println!("RAW RESPONSE = {:?}", response);
    println!();
    println!("Greeting: '{}'", response.into_inner().greeting);

    Ok(())
}
