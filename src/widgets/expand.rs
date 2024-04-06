use std::{marker::PhantomData, collections::HashSet};

use crate::{
    event, graphics, layout,
    actual_widget::{ActualWidget, ActualWidgetId, ActualWidgetIdMaker},
    widgets::Widget,
};

pub struct Expand<Data, Child: Widget<Data>> {
    child: Child,

    _phantom: PhantomData<fn(&mut Data)>,
}

pub struct ExpandActualWidget<Data, Child: ActualWidget<Data>> {
    child: Child,

    _phantom: PhantomData<fn(&mut Data)>,
}

impl<Data, Child: Widget<Data>> Expand<Data, Child> {
    pub(crate) fn new(child: Child) -> Self {
        Self { child, _phantom: PhantomData }
    }
}

impl<Data, Child: Widget<Data>> Widget<Data> for Expand<Data, Child> {
    type ActualWidget = ExpandActualWidget<Data, <Child as Widget<Data>>::ActualWidget>;

    fn to_actual_widget(self, id_maker: &mut ActualWidgetIdMaker) -> Self::ActualWidget {
        ExpandActualWidget { child: self.child.to_actual_widget(id_maker), _phantom: PhantomData }
    }

    fn update_actual_widget(self, actual_widget: &mut Self::ActualWidget, id_maker: &mut ActualWidgetIdMaker) {
        self.child.update_actual_widget(&mut actual_widget.child, id_maker);
    }
}

impl<Data, Child: ActualWidget<Data>> ActualWidget<Data> for ExpandActualWidget<Data, Child> {
    fn layout(&mut self, graphics_context: &graphics::GraphicsContext, sc: layout::SizeConstraints) {
        self.child.layout(graphics_context, layout::SizeConstraints { min: sc.max, max: sc.max });
    }

    fn draw(&self, graphics_context: &graphics::GraphicsContext, target: &mut dyn graphics::RenderTarget, top_left: graphics::Vector2f, hover: &HashSet<ActualWidgetId>) {
        self.child.draw(graphics_context, target, top_left, hover);
    }

    fn find_hover(&self, top_left: graphics::Vector2f, mouse: graphics::Vector2f) -> Box<(dyn Iterator<Item = (ActualWidgetId, bool)> + '_)> {
        self.child.find_hover(top_left, mouse)
    }

    fn size(&self) -> graphics::Vector2f {
        self.child.size()
    }

    fn dispatch_event(&mut self, top_left: graphics::Vector2f, data: &mut Data, target: ActualWidgetId, event: event::TargetedEvent) {
        self.child.dispatch_event(top_left, data, target, event);
    }

    fn targeted_event(&mut self, _: graphics::Vector2f, _: &mut Data, _: event::TargetedEvent) {}
    fn general_event(&mut self, _: graphics::Vector2f, _: &mut Data, _: event::GeneralEvent) {}
}
