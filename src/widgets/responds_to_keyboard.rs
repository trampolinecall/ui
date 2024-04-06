// TODO: this should probably be removed when i figure out a better event dispatch system
use std::{collections::HashSet, marker::PhantomData};

use crate::{
    actual_widget::{ActualWidget, ActualWidgetId, ActualWidgetIdMaker},
    event, graphics, layout,
    widgets::Widget,
};

pub struct RespondsToKeyboard<Data, Child: Widget<Data>, Callback: Fn(&mut Data)> {
    key: sfml::window::Key,
    on_press: Callback,
    child: Child,

    _phantom: PhantomData<fn(&mut Data)>,
}

pub struct RespondsToKeyboardActualWidget<Data, Child: ActualWidget<Data>, Callback: Fn(&mut Data)> {
    id: ActualWidgetId,
    key: sfml::window::Key,
    on_press: Callback,
    child: Child,

    _phantom: PhantomData<fn(&mut Data)>,
    _private: (),
}

impl<Data, Child: Widget<Data>, Callback: Fn(&mut Data)> RespondsToKeyboard<Data, Child, Callback> {
    pub fn new(key: sfml::window::Key, on_press: Callback, child: Child) -> Self {
        Self { key, on_press, child, _phantom: PhantomData }
    }
}

impl<Data, Child: Widget<Data>, Callback: Fn(&mut Data)> Widget<Data> for RespondsToKeyboard<Data, Child, Callback> {
    type ActualWidget = RespondsToKeyboardActualWidget<Data, <Child as Widget<Data>>::ActualWidget, Callback>;

    fn to_actual_widget(self, id_maker: &mut ActualWidgetIdMaker) -> Self::ActualWidget {
        RespondsToKeyboardActualWidget { id: id_maker.next_id(), key: self.key, on_press: self.on_press, child: self.child.to_actual_widget(id_maker), _phantom: PhantomData, _private: () }
    }

    fn update_actual_widget(self, actual_widget: &mut Self::ActualWidget, id_maker: &mut ActualWidgetIdMaker) {
        actual_widget.key = self.key;
        actual_widget.on_press = self.on_press;
        self.child.update_actual_widget(&mut actual_widget.child, id_maker);
    }
}

impl<Data, Child: ActualWidget<Data>, Callback: Fn(&mut Data)> ActualWidget<Data> for RespondsToKeyboardActualWidget<Data, Child, Callback> {
    fn layout(&mut self, graphics_context: &graphics::GraphicsContext, sc: layout::SizeConstraints) {
        self.child.layout(graphics_context, sc);
    }

    fn draw(&self, graphics_context: &graphics::GraphicsContext, target: &mut dyn graphics::RenderTarget, top_left: graphics::Vector2f, hover: &HashSet<ActualWidgetId>) {
        self.child.draw(graphics_context, target, top_left, hover);
    }

    fn find_hover(&self, top_left: graphics::Vector2f, mouse: graphics::Vector2f) -> Box<(dyn Iterator<Item = (ActualWidgetId, bool)> + '_)> {
        Box::new(if graphics::FloatRect::from_vecs(top_left, self.size()).contains(mouse) { Some((self.id, true)) } else { None }.into_iter().chain(self.child.find_hover(top_left, mouse)))
    }

    fn size(&self) -> graphics::Vector2f {
        self.child.size()
    }

    fn dispatch_event(&mut self, top_left: graphics::Vector2f, data: &mut Data, target: ActualWidgetId, event: event::TargetedEvent) {
        if target == self.id {
            self.targeted_event(top_left, data, event);
        }

        self.child.dispatch_event(top_left, data, target, event);
    }

    fn targeted_event(&mut self, _: graphics::Vector2f, _: &mut Data, _: event::TargetedEvent) {}
    fn general_event(&mut self, top_left: graphics::Vector2f, data: &mut Data, event: event::GeneralEvent) {
        match event {
            event::GeneralEvent::MouseMoved(_) => {}
            event::GeneralEvent::LeftMouseUp => {}
            event::GeneralEvent::RightMouseUp => {}
            event::GeneralEvent::KeyPressed { code, .. } => {
                if code == self.key {
                    // TODO: modifier keys?
                    (self.on_press)(data);
                }
            }
        }

        self.child.general_event(top_left, data, event);
    }
}
