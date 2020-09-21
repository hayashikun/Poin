use std::sync::mpsc::Sender;

use glium::backend::glutin::glutin::EventsLoopProxy;

use crate::proto::qoin::hand_tracking_client::HandTrackingClient;
use crate::proto::qoin::HandTrackingPullRequest;

pub async fn connect(
    proxy: EventsLoopProxy,
    tx: Sender<[f64; 2]>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = HandTrackingClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(HandTrackingPullRequest {});
    let response = client.hand_tracking_pull_stream(request).await?;
    println!("{:?}", response);
    let mut inbound = response.into_inner();
    while let Some(message) = inbound.message().await? {
        if let Some(landmark_list) = message.landmark_list {
            let x_sum: f64 = landmark_list
                .landmark
                .iter()
                .map(|l| l.x.unwrap_or(0.0) as f64)
                .sum();
            let y_sum: f64 = landmark_list
                .landmark
                .iter()
                .map(|l| l.y.unwrap_or(0.0) as f64)
                .sum();
            if x_sum == 0.0 && y_sum == 0.0 {
                continue;
            }
            let x = x_sum / landmark_list.landmark.len() as f64;
            let y = y_sum / landmark_list.landmark.len() as f64;
            tx.send([x, y]).expect("Failed to send x, y");
            proxy.wakeup().expect("Failed to wakeup");
        }
    }
    Ok(())
}
