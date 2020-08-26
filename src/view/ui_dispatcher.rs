use std::borrow::BorrowMut;

use conrod_core::{widget, Colorable, Point, Positionable, Ui, UiBuilder, Widget};

use crate::event::UIAction;

widget_ids! {
    pub struct WidgetIds {
        circle,
        rect,
    }
}

struct UiState {
    pub init: bool,
    pub circle_center: Point,
    pub window_width: f64,
    pub window_height: f64,
}

pub struct UiDispatcher {
    pub ui: Ui,
    widget_ids: WidgetIds,
    state: UiState,
}

impl UiDispatcher {
    pub fn new(width: f64, height: f64) -> Self {
        let mut ui = UiBuilder::new([width, height]).build();
        let ids = WidgetIds::new(ui.widget_id_generator());
        UiDispatcher {
            ui,
            widget_ids: ids,
            state: UiState {
                init: false,
                circle_center: [0.0, 0.0],
                window_width: width,
                window_height: height,
            },
        }
    }

    pub fn dispatch(&mut self, actions: Vec<UIAction>) {
        let mut cell = self.ui.set_widgets();
        for a in actions {
            match a {
                UIAction::Move { x, y } => {
                    self.state.circle_center[0] += x;
                    self.state.circle_center[1] += y;
                }
                UIAction::MoveTo {
                    mut x,
                    mut y,
                    normalized,
                } => {
                    if normalized {
                        // x, y: 0-1
                        x = (x - 0.5) * 1.2 * self.state.window_width;
                        y = (0.5 - y) * 1.2 * self.state.window_height;
                    }
                    self.state.circle_center[0] = x;
                    self.state.circle_center[1] = y;
                }
            }
        }
        let circle = widget::Circle::fill(if self.state.init { 20.0 } else { 0.0 })
            .xy(self.state.circle_center)
            .color(conrod_core::color::GREEN);
        circle.set(self.widget_ids.circle, cell.borrow_mut());
        if !self.state.init
            && (self.state.circle_center[0] != 0.0 || self.state.circle_center[1] != 0.0)
        {
            self.state.init = true;
        }
    }
}
