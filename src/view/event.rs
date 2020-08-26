use glium;
use glium::backend::glutin::glutin::EventsLoop;
use glium::glutin::Event;
use glium::{glutin, Surface};

pub struct EventHandler {
    pub event_loop: EventsLoop,
}

impl EventHandler {
    pub fn new() -> Self {
        EventHandler {
            event_loop: glutin::EventsLoop::new(),
        }
    }

    pub fn start(&mut self, display: &glium::Display, ref mut ui_dispatcher: super::UiDispatcher) {
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
                    glutin::ControlFlow::Break
                })
            }

            let mut poin_events = Vec::new();
            for event in events.drain(..) {
                match PoinEvent::new(event.clone()) {
                    Some(e) => poin_events.push(e),
                    _ => (),
                }
                match event.clone() {
                    glutin::Event::WindowEvent { event, .. } => match event {
                        glutin::WindowEvent::KeyboardInput {
                            input:
                                glutin::KeyboardInput {
                                    virtual_keycode: Some(glutin::VirtualKeyCode::Escape),
                                    ..
                                },
                            ..
                        } => break 'render,
                        _ => (),
                    },
                    _ => (),
                };
            }

            ui_dispatcher.dispatch(poin_events);

            if let Some(primitives) = ui_dispatcher.ui.draw_if_changed() {
                renderer.fill(display, primitives, &image_map);
                let mut target = display.draw();
                target.clear_color(0., 0., 0., 0.);
                renderer.draw(display, &mut target, &image_map).unwrap();
                target.finish().unwrap();
            }
        }
    }
}

pub enum PoinEvent {
    Move([f64; 2]),
    MoveTo([f64; 2]),
}

impl PoinEvent {
    pub fn new(event: Event) -> Option<Self> {
        match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::KeyboardInput { input, .. } => match input {
                    glutin::KeyboardInput {
                        virtual_keycode, ..
                    } => match virtual_keycode {
                        Some(glutin::VirtualKeyCode::Left) => {
                            return Some(PoinEvent::Move([-10.0, 0.0]))
                        }
                        Some(glutin::VirtualKeyCode::Right) => {
                            return Some(PoinEvent::Move([10.0, 0.0]))
                        }
                        Some(glutin::VirtualKeyCode::Up) => {
                            return Some(PoinEvent::Move([0.0, 10.0]))
                        }
                        Some(glutin::VirtualKeyCode::Down) => {
                            return Some(PoinEvent::Move([0.0, -10.0]))
                        }
                        _ => (),
                    },
                },
                _ => (),
            },
            _ => (),
        };
        return None;
    }
}
