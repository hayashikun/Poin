#[macro_use]
extern crate conrod_core;

mod view;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn main() {
    let mut event_handler = view::EventHandler::new();
    let display = view::display(WIDTH, HEIGHT, &event_handler.event_loop);
    let ui_manager = view::UiDispatcher::new(WIDTH as f64, HEIGHT as f64);
    event_handler.start(&display, ui_manager);
}
