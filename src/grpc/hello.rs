use hello::greeter_client::GreeterClient;
use hello::HelloRequest;

pub mod hello {
    tonic::include_proto!("qoin");
}

#[tokio::main]
pub async fn connect() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;
    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });
    let response = client.say_hello(request).await?;
    println!("RESPONSE={:?}", response);
    Ok(())
}
