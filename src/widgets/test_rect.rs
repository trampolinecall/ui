use std::collections::HashSet;

use sfml::graphics::Shape;

use crate::{
    actual_widget::{ActualWidget, ActualWidgetId, ActualWidgetIdMaker},
    event::{GeneralEvent, TargetedEvent},
    graphics, layout,
    widgets::Widget,
};

pub struct TestRect {
    color: graphics::Color,
    size: graphics::Vector2f,
}
pub struct TestRectActualWidget {
    id: ActualWidgetId,
    color: graphics::Color,
    ideal_size: graphics::Vector2f,
    layout_size: graphics::Vector2f,
    _private: (),
}

impl TestRect {
    pub fn new(color: graphics::Color, size: graphics::Vector2f) -> TestRect {
        TestRect { color, size }
    }
}

impl<Data> Widget<Data> for TestRect {
    type ActualWidget = TestRectActualWidget;

    fn to_actual_widget(self, id_maker: &mut ActualWidgetIdMaker) -> Self::ActualWidget {
        TestRectActualWidget {
            color: self.color,
            ideal_size: self.size,
            _private: (),
            layout_size: graphics::Vector2f::new(0.0, 0.0),
            id: id_maker.next_id(),
        }
    }

    fn update_actual_widget(self, actual_widget: &mut Self::ActualWidget, _: &mut ActualWidgetIdMaker) {
        // TODO: animate?
        actual_widget.color = self.color;
        actual_widget.ideal_size = self.size;
    }
}

impl<Data> ActualWidget<Data> for TestRectActualWidget {
    fn layout(&mut self, _: &graphics::GraphicsContext, sc: layout::SizeConstraints) {
        self.layout_size = sc.clamp_size(self.ideal_size);
    }

    fn draw(
        &self,
        _: &graphics::GraphicsContext,
        target: &mut dyn graphics::RenderTarget,
        top_left: graphics::Vector2f,
        hover: &HashSet<ActualWidgetId>,
    ) {
        let rect = graphics::FloatRect::from_vecs(top_left, self.layout_size);
        let mut rect_shape = graphics::RectangleShape::from_rect(rect);
        rect_shape.set_fill_color(self.color);

        if hover.contains(&self.id) {
            rect_shape.set_outline_color(graphics::Color::rgba(255, 255, 255, 100)); // TODO: pick a better color for this
            rect_shape.set_outline_thickness(5.0);
        }

        target.draw(&rect_shape);
    }

    fn find_hover(&self, top_left: graphics::Vector2f, mouse: graphics::Vector2f) -> Box<(dyn Iterator<Item = (ActualWidgetId, bool)>)> {
        Box::new(if graphics::FloatRect::from_vecs(top_left, self.layout_size).contains(mouse) { Some((self.id, false)) } else { None }.into_iter())
    }

    fn size(&self) -> graphics::Vector2f {
        self.layout_size
    }

    fn dispatch_event(&mut self, top_left: graphics::Vector2f, data: &mut Data, target: ActualWidgetId, event: TargetedEvent) {
        if target == self.id {
            self.targeted_event(top_left, data, event);
        }
    }

    fn targeted_event(&mut self, _: graphics::Vector2f, _: &mut Data, _: TargetedEvent) {}
    fn general_event(&mut self, _: graphics::Vector2f, _: &mut Data, _: GeneralEvent) {}
}
