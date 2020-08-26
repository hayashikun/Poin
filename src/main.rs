#[macro_use]
extern crate conrod_core;

mod event;
mod grpc;
mod view;

#[tokio::main]
async fn main() {
    tokio::spawn(async {
        // let result = grpc::hello::connect().await;
        let result = grpc::hand_tracking::connect().await;
        println!("{:?}", result);
    });

    let mut event_handler = event::EventHandler::new();
    let display = view::display(&event_handler.event_loop);
    let size = display.gl_window().window().get_outer_size();
    let ui_manager = view::UiDispatcher::new(size.unwrap().width, size.unwrap().height);
    event_handler.start(&display, ui_manager);
}
