use std::borrow::BorrowMut;

use conrod_core::{widget, Colorable, Point, Positionable, Ui, UiBuilder, Widget};

use crate::view::PoinEvent;

widget_ids! {
    pub struct WidgetIds {
        circle,
        rect,
    }
}

struct UiState {
    pub init: bool,
    pub circle_center: Point,
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
            },
        }
    }

    pub fn dispatch(&mut self, events: Vec<PoinEvent>) {
        let mut cell = self.ui.set_widgets();
        for e in events {
            match e {
                PoinEvent::Move(xy) => {
                    self.state.circle_center[0] += xy[0];
                    self.state.circle_center[1] += xy[1];
                }
                PoinEvent::MoveTo(xy) => {
                    self.state.circle_center[0] = xy[0];
                    self.state.circle_center[1] = xy[1];
                }
            }
        }
        let circle = widget::Circle::fill(if self.state.init { 10.0 } else { 0.0 })
            .xy(self.state.circle_center)
            .color(conrod_core::color::RED);
        circle.set(self.widget_ids.circle, cell.borrow_mut());
        if !self.state.init
            && (self.state.circle_center[0] != 0.0 || self.state.circle_center[1] != 0.0)
        {
            self.state.init = true;
        }
    }
}
