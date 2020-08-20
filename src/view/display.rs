use glium;
use glium::glutin::os::macos::WindowBuilderExt;
use glium::glutin::EventsLoop;

pub fn display(width: u32, height: u32, event_loop: &EventsLoop) -> glium::Display {
    let window = glium::glutin::WindowBuilder::new()
        .with_dimensions((width, height).into())
        .with_always_on_top(true)
        .with_title("Poin")
        //.with_transparency(true)
        //.with_titlebar_hidden(true)
        .with_titlebar_buttons_hidden(true);
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);
    glium::Display::new(window, context, &event_loop).unwrap()
}
