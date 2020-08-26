use hello::greeter_client::GreeterClient;
use hello::HelloRequest;

pub mod hello {
    tonic::include_proto!("qoin");
}

pub async fn connect() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });
    let response = client.say_hello(request).await?;
    println!("RESPONSE={:?}", response);

    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });
    let response = client.hello_stream(request).await?;
    let mut inbound = response.into_inner();
    while let Some(message) = inbound.message().await? {
        println!("MESSAGE = {:?}", message);
    }
    Ok(())
}
