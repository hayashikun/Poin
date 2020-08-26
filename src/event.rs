use glium;
use glium::backend::glutin::glutin::{EventsLoop, EventsLoopProxy};
use glium::glutin::Event;
use glium::{glutin, Surface};
use std::sync::mpsc::Receiver;
use std::time::Duration;

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

    pub fn enqueue(&self) {}

    pub fn start(
        &mut self,
        display: &glium::Display,
        ref mut ui_dispatcher: crate::view::UiDispatcher,
    ) {
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

            let mut ui_actions = Vec::new();
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
                        _ => (),
                    },
                    glutin::Event::Awakened => {
                        for p in self.rx.recv_timeout(Duration::from_millis(500)).iter() {
                            let a = UIAction::MoveTo {
                                x: p[0],
                                y: p[1],
                                normalized: true,
                            };
                            ui_actions.push(a);
                        }
                    }
                    _ => (),
                }
                match Action::new(event.clone()) {
                    Some(Action::UI { action: a }) => ui_actions.push(a),
                    _ => (),
                }
            }
            ui_dispatcher.dispatch(ui_actions);

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

enum Action {
    UI { action: UIAction },
}
pub enum UIAction {
    Move { x: f64, y: f64 },
    MoveTo { x: f64, y: f64, normalized: bool },
}

impl Action {
    pub fn new(event: Event) -> Option<Self> {
        fn move_action(x: f64, y: f64) -> Option<Action> {
            Some(Action::UI {
                action: UIAction::Move { x, y },
            })
        }

        let pe = match event {
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
                glutin::VirtualKeyCode::Left => move_action(-10.0, 0.0),
                glutin::VirtualKeyCode::Right => move_action(10.0, 0.0),
                glutin::VirtualKeyCode::Up => move_action(0.0, 10.0),
                glutin::VirtualKeyCode::Down => move_action(0.0, -10.0),
                _ => None,
            },
            _ => None,
        };
        return pe;
    }
}
