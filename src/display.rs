use glium;
use glium::backend::glutin::glutin::dpi::LogicalSize;
use glium::glutin::os::macos::WindowBuilderExt;
use glium::glutin::EventsLoop;

pub fn display(event_loop: &EventsLoop) -> glium::Display {
    let window = glium::glutin::WindowBuilder::new()
        .with_title("Poin")
        .with_dimensions(LogicalSize::new(50.0, 50.0))
        .with_always_on_top(true)
        .with_transparency(true)
        .with_titlebar_hidden(true)
        .with_titlebar_buttons_hidden(true);
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);
    glium::Display::new(window, context, &event_loop).unwrap()
}
