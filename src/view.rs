use std::borrow::BorrowMut;

use conrod_core::{widget, Colorable, Positionable, Ui, UiBuilder, Widget};

widget_ids! {
    pub struct WidgetIds {
        circle,
    }
}

pub struct View {
    pub ui: Ui,
    widget_ids: WidgetIds,
}

impl View {
    pub fn new(width: f64, height: f64) -> Self {
        let mut ui = UiBuilder::new([width, height]).build();
        let ids = WidgetIds::new(ui.widget_id_generator());
        View {
            ui,
            widget_ids: ids,
        }
    }

    pub fn update(&mut self) {
        let mut cell = self.ui.set_widgets();
        let circle = widget::Circle::fill(20.0)
            .xy([0.0, 0.0])
            .color(conrod_core::color::GREEN);
        circle.set(self.widget_ids.circle, cell.borrow_mut());
    }
}
