#[macro_use]
extern crate conrod_core;

mod view;

fn main() {
    let mut event_handler = view::EventHandler::new();
    let display = view::display(&event_handler.event_loop);
    let size = display.gl_window().window().get_outer_size();
    let ui_manager = view::UiDispatcher::new(size.unwrap().width, size.unwrap().height);
    event_handler.start(&display, ui_manager);
}
