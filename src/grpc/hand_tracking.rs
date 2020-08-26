use qoin_grpc::hand_tracking_client::HandTrackingClient;
use qoin_grpc::HandTrackingRequest;

pub mod qoin_grpc {
    tonic::include_proto!("qoin");
}

pub async fn connect() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = HandTrackingClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(HandTrackingRequest {});
    let response = client.hand_tracking_stream(request).await?;
    println!("{:?}", response);
    let mut inbound = response.into_inner();
    while let Some(message) = inbound.message().await? {
        if let Some(landmark_list) = message.landmark_list {
            let x_sum: f64 = landmark_list.landmark.iter().map(|l| l.x as f64).sum();
            let y_sum: f64 = landmark_list.landmark.iter().map(|l| l.y as f64).sum();
            let x = x_sum / landmark_list.landmark.len() as f64;
            let y = y_sum / landmark_list.landmark.len() as f64;
            println!("{:?}", (x, y));
        }
    }
    Ok(())
}
