use std::borrow::BorrowMut;

use crate::view::PoinEvent;
use conrod_core::{widget, Colorable, Point, Positionable, Ui, UiBuilder, Widget};

widget_ids! {
    pub struct WidgetIds {
        circle,
        rect,
    }
}

struct UiState {
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
                _ => (),
            }
        }
        let circle = widget::Circle::fill(50.)
            .xy(self.state.circle_center)
            .color(conrod_core::color::BLUE)
            .depth(0.0);
        circle.set(self.widget_ids.circle, cell.borrow_mut());

        let mut rect_center = self.state.circle_center.clone();
        rect_center[0] += 40.0;
        let rect = widget::Rectangle::fill([50.0, 50.0])
            .xy(rect_center)
            .color(conrod_core::color::RED)
            .depth(0.5);
        rect.set(self.widget_ids.rect, cell.borrow_mut());
    }
}
