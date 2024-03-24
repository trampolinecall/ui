use std::collections::HashSet;

use crate::{
    actual_widget::{ActualWidget, ActualWidgetId, ActualWidgetIdMaker},
    event, graphics, layout,
    widgets::Widget,
};

pub struct Empty;

pub struct EmptyActualWidget {
    size: graphics::Vector2f,
}

impl<Data> Widget<Data> for Empty {
    type ActualWidget = EmptyActualWidget;

    fn to_actual_widget(self, _: &mut ActualWidgetIdMaker) -> Self::ActualWidget {
        EmptyActualWidget { size: graphics::Vector2f::new(0.0, 0.0) }
    }

    fn update_actual_widget(self, _: &mut Self::ActualWidget, _: &mut ActualWidgetIdMaker) {}
}

impl<Data> ActualWidget<Data> for EmptyActualWidget {
    fn layout(&mut self, _: &graphics::GraphicsContext, sc: layout::SizeConstraints) {
        self.size = sc.clamp_size(graphics::Vector2f::new(0.0, 0.0));
    }

    fn draw(&self, _: &graphics::GraphicsContext, _: &mut dyn graphics::RenderTarget, _: graphics::Vector2f, _: &HashSet<ActualWidgetId>) {}

    fn find_hover(&self, _: graphics::Vector2f, _: graphics::Vector2f) -> Box<(dyn Iterator<Item = (ActualWidgetId, bool)>)> {
        Box::new(std::iter::empty())
    }

    fn size(&self) -> graphics::Vector2f {
        self.size
    }

    fn send_targeted_event(&mut self, _: graphics::Vector2f, _: &mut Data, _: ActualWidgetId, _: event::TargetedEvent) {}
    fn targeted_event(&mut self, _: graphics::Vector2f, _: &mut Data, _: event::TargetedEvent) {}
    fn general_event(&mut self, _: graphics::Vector2f, _: &mut Data, _: event::GeneralEvent) {}
}
