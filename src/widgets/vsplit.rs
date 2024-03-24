use std::{collections::HashSet, marker::PhantomData};

use crate::{
    actual_widget::{ActualWidget, ActualWidgetId, ActualWidgetIdMaker},
    event::{GeneralEvent, TargetedEvent},
    graphics, layout,
    widgets::Widget,
};

pub struct VSplit<Data, Left: Widget<Data>, Right: Widget<Data>> {
    left: Left,
    right: Right,

    _phantom: PhantomData<fn(&mut Data)>,
}

pub struct VSplitActualWidget<Data, Left: ActualWidget<Data>, Right: ActualWidget<Data>> {
    left: Left,
    right: Right,

    size: graphics::Vector2f,

    _phantom: PhantomData<fn(&mut Data)>,
    _private: (),
}

impl<Data, Left: Widget<Data>, Right: Widget<Data>> VSplit<Data, Left, Right> {
    pub fn new(left: Left, right: Right) -> Self {
        Self { left, right, _phantom: PhantomData }
    }
}

impl<Data, Left: Widget<Data>, Right: Widget<Data>> Widget<Data> for VSplit<Data, Left, Right> {
    type ActualWidget = VSplitActualWidget<Data, <Left as Widget<Data>>::ActualWidget, <Right as Widget<Data>>::ActualWidget>;

    fn to_actual_widget(self, id_maker: &mut ActualWidgetIdMaker) -> Self::ActualWidget {
        VSplitActualWidget { left: self.left.to_actual_widget(id_maker), right: self.right.to_actual_widget(id_maker), size: graphics::Vector2f::new(0.0, 0.0), _phantom: PhantomData, _private: () }
    }

    fn update_actual_widget(self, actual_widget: &mut Self::ActualWidget, id_maker: &mut ActualWidgetIdMaker) {
        self.left.update_actual_widget(&mut actual_widget.left, id_maker);
        self.right.update_actual_widget(&mut actual_widget.right, id_maker);
    }
}
impl<Data, Left: ActualWidget<Data>, Right: ActualWidget<Data>> ActualWidget<Data> for VSplitActualWidget<Data, Left, Right> {
    fn layout(&mut self, graphics_context: &graphics::GraphicsContext, sc: layout::SizeConstraints) {
        let half_sc = layout::SizeConstraints { min: graphics::Vector2f::new(sc.min.x / 2.0, sc.min.y), max: graphics::Vector2f::new(sc.max.x / 2.0, sc.max.y) };
        self.left.layout(graphics_context, half_sc);
        self.right.layout(graphics_context, half_sc);
        self.size = sc.clamp_size(self.left.size() + self.right.size());
    }

    fn draw(&self, graphics_context: &graphics::GraphicsContext, target: &mut dyn graphics::RenderTarget, top_left: graphics::Vector2f, hover: &HashSet<ActualWidgetId>) {
        self.left.draw(graphics_context, target, top_left, hover);
        self.right.draw(graphics_context, target, top_left + graphics::Vector2f::new(self.left.size().x, 0.0), hover);
    }

    fn size(&self) -> graphics::Vector2f {
        self.size
    }

    fn find_hover(&self, top_left: graphics::Vector2f, mouse: graphics::Vector2f) -> Box<(dyn Iterator<Item = (ActualWidgetId, bool)> + '_)> {
        Box::new(self.left.find_hover(top_left, mouse).chain(self.right.find_hover(top_left + graphics::Vector2f::new(self.left.size().x, 0.0), mouse)))
    }

    fn send_targeted_event(&mut self, top_left: graphics::Vector2f, data: &mut Data, target: ActualWidgetId, event: TargetedEvent) {
        self.left.send_targeted_event(top_left, data, target, event);
        self.right.send_targeted_event(top_left, data, target, event);
    }

    fn targeted_event(&mut self, _: graphics::Vector2f, _: &mut Data, _: TargetedEvent) {}

    fn general_event(&mut self, _: graphics::Vector2f, _: &mut Data, _: GeneralEvent) {}
}
