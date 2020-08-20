use glium;
use glium::backend::glutin::glutin::EventsLoop;
use glium::Surface;

pub struct EventHandler {
    pub event_loop: EventsLoop,
}

impl EventHandler {
    pub fn new() -> Self {
        EventHandler {
            event_loop: glium::glutin::EventsLoop::new(),
        }
    }

    pub fn start(&mut self, display: &glium::Display, ref mut ui_manager: super::UiDispatcher) {
        let mut renderer = conrod_glium::Renderer::new(display).unwrap();
        let image_map = conrod_core::image::Map::<glium::texture::Texture2d>::new();

        let mut events = Vec::new();
        'render: loop {
            events.clear();
            self.event_loop.poll_events(|event| {
                events.push(event);
            });

            if events.is_empty() {
                self.event_loop.run_forever(|event| {
                    events.push(event);
                    glium::glutin::ControlFlow::Break
                })
            }

            for event in events.drain(..) {
                match event.clone() {
                    glium::glutin::Event::WindowEvent { event, .. } => match event {
                        glium::glutin::WindowEvent::KeyboardInput {
                            input:
                                glium::glutin::KeyboardInput {
                                    virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
                                    ..
                                },
                            ..
                        } => break 'render,
                        _ => (),
                    },
                    _ => (),
                };
            }

            ui_manager.dispatch();

            if let Some(primitives) = ui_manager.ui.draw_if_changed() {
                renderer.fill(display, primitives, &image_map);
                let mut target = display.draw();
                target.clear_color(0., 0., 0., 0.);
                renderer.draw(display, &mut target, &image_map).unwrap();
                target.finish().unwrap();
            }
        }
    }
}
