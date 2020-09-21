use std::sync::mpsc::Receiver;
use std::time::Duration;

use glium;
use glium::backend::glutin::glutin::EventsLoop;
use glium::glutin::dpi::LogicalPosition;
use glium::{glutin, Surface};

pub struct EventHandler {
    pub event_loop: EventsLoop,
    pub rx: Receiver<[f64; 2]>,
}

impl EventHandler {
    pub fn new(rx: Receiver<[f64; 2]>) -> Self {
        EventHandler {
            event_loop: glutin::EventsLoop::new(),
            rx,
        }
    }

    pub fn start(&mut self, display: &glium::Display, ref mut view: crate::view::View) {
        let mut renderer = conrod_glium::Renderer::new(display).unwrap();
        let image_map = conrod_core::image::Map::<glium::texture::Texture2d>::new();
        let monitor_size = display.gl_window().get_current_monitor().get_dimensions();
        let dpi_factor = display.gl_window().get_current_monitor().get_hidpi_factor();
        let x_factor = monitor_size.width / dpi_factor;
        let y_factor = monitor_size.height / dpi_factor;

        let move_to_window = |x: f64, y: f64| {
            display.gl_window().set_position(LogicalPosition { x, y });
        };
        let move_window = |dx: f64, dy: f64| {
            let p = display.gl_window().get_position().unwrap();
            move_to_window(p.x + dx, p.y + dy);
        };

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

            for event in events.drain(..) {
                match event.clone() {
                    glutin::Event::WindowEvent {
                        event:
                            glutin::WindowEvent::KeyboardInput {
                                input:
                                    glutin::KeyboardInput {
                                        virtual_keycode: Some(virtual_keycode),
                                        ..
                                    },
                                ..
                            },
                        ..
                    } => match virtual_keycode {
                        glutin::VirtualKeyCode::Escape => break 'render,
                        glutin::VirtualKeyCode::Left => move_window(-10.0, 0.0),
                        glutin::VirtualKeyCode::Right => move_window(10.0, 0.0),
                        glutin::VirtualKeyCode::Up => move_window(0.0, -10.0),
                        glutin::VirtualKeyCode::Down => move_window(0.0, 10.0),
                        _ => (),
                    },
                    glutin::Event::Awakened => {
                        for p in self.rx.recv_timeout(Duration::from_millis(500)).iter() {
                            move_to_window(p[0] * x_factor, p[1] * y_factor)
                        }
                    }
                    _ => (),
                }
            }
            view.update();

            if let Some(primitives) = view.ui.draw_if_changed() {
                renderer.fill(display, primitives, &image_map);
                let mut target = display.draw();
                target.clear_color(0., 0., 0., 0.);
                renderer.draw(display, &mut target, &image_map).unwrap();
                target.finish().unwrap();
            }
        }
    }
}
