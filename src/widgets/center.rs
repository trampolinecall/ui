use std::{marker::PhantomData, collections::HashSet};

use crate::{
    event, graphics, layout,
    actual_widget::{ActualWidget, ActualWidgetId, ActualWidgetIdMaker},
    widgets::Widget,
};

pub struct Center<Data, Child: Widget<Data>> {
    child: Child,

    _phantom: PhantomData<fn(&mut Data)>,
}

pub struct CenterActualWidget<Data, Child: ActualWidget<Data>> {
    child: Child,
    size: graphics::Vector2f,

    _phantom: PhantomData<fn(&mut Data)>,
}

impl<Data, Child: Widget<Data>> Center<Data, Child> {
    pub fn new(child: Child) -> Self {
        Self { child, _phantom: PhantomData }
    }
}

impl<Data, Child: Widget<Data>> Widget<Data> for Center<Data, Child> {
    type ActualWidget = CenterActualWidget<Data, <Child as Widget<Data>>::ActualWidget>;

    fn to_actual_widget(self, id_maker: &mut ActualWidgetIdMaker) -> Self::ActualWidget {
        CenterActualWidget { child: self.child.to_actual_widget(id_maker), size: graphics::Vector2f::new(0.0, 0.0), _phantom: PhantomData }
    }

    fn update_actual_widget(self, actual_widget: &mut Self::ActualWidget, id_maker: &mut ActualWidgetIdMaker) {
        self.child.update_actual_widget(&mut actual_widget.child, id_maker);
    }
}

impl<Data, Child: ActualWidget<Data>> ActualWidget<Data> for CenterActualWidget<Data, Child> {
    fn layout(&mut self, graphics_context: &graphics::GraphicsContext, sc: layout::SizeConstraints) {
        self.child.layout(graphics_context, sc.with_no_min());
        self.size = sc.max;
    }

    fn draw(&self, graphics_context: &graphics::GraphicsContext, target: &mut dyn graphics::RenderTarget, top_left: graphics::Vector2f, hover: &HashSet<ActualWidgetId>) {
        self.child.draw(graphics_context, target, center(top_left, self.size, self.child.size()), hover);
    }

    fn find_hover(&self, top_left: graphics::Vector2f, mouse: graphics::Vector2f) -> Box<(dyn Iterator<Item = (ActualWidgetId, bool)> + '_)> {
        self.child.find_hover(center(top_left, self.size, self.child.size()), mouse)
    }

    fn size(&self) -> graphics::Vector2f {
        self.size
    }

    fn dispatch_event(&mut self, top_left: graphics::Vector2f, data: &mut Data, target: ActualWidgetId, event: event::TargetedEvent) {
        self.child.dispatch_event(center(top_left, self.size, self.child.size()), data, target, event);
    }

    fn targeted_event(&mut self, _: graphics::Vector2f, _: &mut Data, _: event::TargetedEvent) {}
    fn general_event(&mut self, _: graphics::Vector2f, _: &mut Data, _: event::GeneralEvent) {}
}

fn center(top_left: graphics::Vector2f, max_size: graphics::Vector2f, child_size: graphics::Vector2f) -> graphics::Vector2f {
    top_left + max_size * 0.5 - (child_size / 2.0)
}
