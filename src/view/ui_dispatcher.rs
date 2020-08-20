use std::borrow::BorrowMut;

use conrod_core::{widget, Colorable, Point, Positionable, Ui, UiBuilder, Widget};

widget_ids! {
    pub struct WidgetIds {
        circle,
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

    pub fn dispatch(&mut self) {
        let mut cell = self.ui.set_widgets();
        widget::Circle::fill(50.)
            .xy(self.state.circle_center)
            .color(conrod_core::color::RED)
            .set(self.widget_ids.circle, cell.borrow_mut());
    }
}
