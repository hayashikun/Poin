#[macro_use]
extern crate conrod_core;

use std::sync::mpsc;

use glium::backend::glutin::glutin::dpi::LogicalPosition;

mod display;
mod event;
mod proto;
mod qoin;
mod view;

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel();
    let mut event_handler = event::EventHandler::new(rx);
    let proxy = event_handler.event_loop.create_proxy();

    tokio::spawn(async {
        let result = qoin::hand_tracking::connect(proxy, tx).await;
        println!("{:?}", result);
    });

    let display = display::display(&event_handler.event_loop);
    display
        .gl_window()
        .set_position(LogicalPosition { x: 0.0, y: 0.0 });
    let size = display.gl_window().window().get_outer_size();
    let view = view::View::new(size.unwrap().width, size.unwrap().height);
    event_handler.start(&display, view);
}
