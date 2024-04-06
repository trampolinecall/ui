use std::{collections::HashSet, marker::PhantomData};

use crate::{
    actual_widget::{ActualWidget, ActualWidgetId, ActualWidgetIdMaker},
    event, graphics, layout,
    widgets::Widget,
};

pub struct MinSize<Data, Child: Widget<Data>> {
    child: Child,
    min_size: graphics::Vector2f,

    _phantom: PhantomData<fn(&mut Data)>,
}

pub struct MinSizeActualWidget<Data, Child: ActualWidget<Data>> {
    child: Child,
    min_size: graphics::Vector2f,

    _phantom: PhantomData<fn(&mut Data)>,
}

impl<Data, Child: Widget<Data>> MinSize<Data, Child> {
    pub fn new(child: Child, min_size: graphics::Vector2f) -> Self {
        Self { child, min_size, _phantom: PhantomData }
    }
}

impl<Data, Child: Widget<Data>> Widget<Data> for MinSize<Data, Child> {
    type ActualWidget = MinSizeActualWidget<Data, <Child as Widget<Data>>::ActualWidget>;

    fn to_actual_widget(self, id_maker: &mut ActualWidgetIdMaker) -> Self::ActualWidget {
        MinSizeActualWidget { child: self.child.to_actual_widget(id_maker), min_size: self.min_size, _phantom: PhantomData }
    }

    fn update_actual_widget(self, actual_widget: &mut Self::ActualWidget, id_maker: &mut ActualWidgetIdMaker) {
        actual_widget.min_size = self.min_size;
        self.child.update_actual_widget(&mut actual_widget.child, id_maker);
    }
}

impl<Data, Child: ActualWidget<Data>> ActualWidget<Data> for MinSizeActualWidget<Data, Child> {
    fn layout(&mut self, graphics_context: &graphics::GraphicsContext, sc: layout::SizeConstraints) {
        let size = sc.clamp_size(self.min_size);
        self.child.layout(graphics_context, layout::SizeConstraints { min: size, max: sc.max });
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
