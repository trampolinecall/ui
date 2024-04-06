use std::{collections::HashSet, marker::PhantomData};

use crate::{
    actual_widget::{animated::Animated, ActualWidget, ActualWidgetId, ActualWidgetIdMaker},
    event, graphics, layout,
    widgets::Widget,
};

pub struct Padding<Data, Child: Widget<Data>> {
    child: Child,
    left: f32,
    top: f32,
    right: f32,
    bottom: f32,

    _phantom: PhantomData<fn(&mut Data)>,
}

pub struct PaddingActualWidget<Data, Child: ActualWidget<Data>> {
    child: Child,
    size: graphics::Vector2f,
    left: Animated<f32>,
    top: Animated<f32>,
    right: Animated<f32>,
    bottom: Animated<f32>,

    _phantom: PhantomData<fn(&mut Data)>,
}

impl<Data, Child: Widget<Data>> Padding<Data, Child> {
    pub fn new(child: Child, left: f32, top: f32, right: f32, bottom: f32) -> Self {
        Self { child, left, top, right, bottom, _phantom: PhantomData }
    }
    pub fn all_around(child: Child, pad: f32) -> Self {
        Self::new(child, pad, pad, pad, pad)
    }
}

impl<Data, Child: Widget<Data>> Widget<Data> for Padding<Data, Child> {
    type ActualWidget = PaddingActualWidget<Data, <Child as Widget<Data>>::ActualWidget>;

    fn to_actual_widget(self, id_maker: &mut ActualWidgetIdMaker) -> Self::ActualWidget {
        PaddingActualWidget {
            child: self.child.to_actual_widget(id_maker),
            size: graphics::Vector2f::new(0.0, 0.0),
            left: Animated::new(self.left),
            top: Animated::new(self.top),
            right: Animated::new(self.right),
            bottom: Animated::new(self.bottom),
            _phantom: PhantomData,
        }
    }

    fn update_actual_widget(self, actual_widget: &mut Self::ActualWidget, id_maker: &mut ActualWidgetIdMaker) {
        self.child.update_actual_widget(&mut actual_widget.child, id_maker);
        actual_widget.left.set(self.left);
        actual_widget.right.set(self.right);
        actual_widget.top.set(self.top);
        actual_widget.bottom.set(self.bottom);
    }
}

impl<Data, Child: ActualWidget<Data>> ActualWidget<Data> for PaddingActualWidget<Data, Child> {
    fn layout(&mut self, graphics_context: &graphics::GraphicsContext, sc: layout::SizeConstraints) {
        let shrunk_sc = sc.shrink(graphics::Vector2f::new(self.left.get_lerped() + self.right.get_lerped(), self.top.get_lerped() + self.bottom.get_lerped()));
        self.child.layout(graphics_context, shrunk_sc);
        self.size = sc.clamp_size(self.child.size() + graphics::Vector2f::new(self.left.get_lerped() + self.right.get_lerped(), self.top.get_lerped() + self.bottom.get_lerped()));
    }

    fn draw(&self, graphics_context: &graphics::GraphicsContext, target: &mut dyn graphics::RenderTarget, top_left: graphics::Vector2f, hover: &HashSet<ActualWidgetId>) {
        // TODO: calculate offset better in order to account for cases where the padding must be cut off because it would be too big to fit in the size constraints
        self.child.draw(graphics_context, target, top_left + graphics::Vector2f::new(self.left.get_lerped(), self.top.get_lerped()), hover);
    }

    fn find_hover(&self, top_left: graphics::Vector2f, mouse: graphics::Vector2f) -> Box<(dyn Iterator<Item = (ActualWidgetId, bool)> + '_)> {
        self.child.find_hover(top_left + graphics::Vector2f::new(self.left.get_lerped(), self.top.get_lerped()), mouse)
    }

    fn size(&self) -> graphics::Vector2f {
        self.size
    }

    fn dispatch_event(&mut self, top_left: graphics::Vector2f, data: &mut Data, target: ActualWidgetId, event: event::TargetedEvent) {
        self.child.dispatch_event(top_left + graphics::Vector2f::new(self.left.get_lerped(), self.top.get_lerped()), data, target, event);
    }

    fn targeted_event(&mut self, _: graphics::Vector2f, _: &mut Data, _: event::TargetedEvent) {}
    fn general_event(&mut self, top_left: graphics::Vector2f, data: &mut Data, event: event::GeneralEvent) {
        self.child.general_event(top_left, data, event);
    }
}
