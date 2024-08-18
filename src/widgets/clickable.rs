use std::{collections::HashSet, marker::PhantomData};

use crate::{
    actual_widget::{ActualWidget, ActualWidgetId, ActualWidgetIdMaker},
    event, graphics, layout,
    widgets::Widget,
};

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum MouseButton {
    Left,
    Right,
}
pub struct Clickable<Data, NormalChild: Widget<Data>, ChildOnClicked: Widget<Data>, Callback: Fn(&mut Data)> {
    mouse_button: MouseButton,
    on_click: Callback,
    normal_child: NormalChild,
    child_on_clicked: ChildOnClicked,

    _phantom: PhantomData<fn(&mut Data)>,
}

pub struct ClickableActualWidget<Data, NormalChild: ActualWidget<Data>, ChildOnClicked: ActualWidget<Data>, Callback: Fn(&mut Data)> {
    id: ActualWidgetId,
    mouse_button: MouseButton,
    on_click: Callback,
    normal_child: NormalChild,
    child_on_clicked: ChildOnClicked,

    clicked: bool,

    _phantom: PhantomData<fn(&mut Data)>,
    _private: (),
}

impl<Data, NormalChild: Widget<Data>, ChildOnClicked: Widget<Data>, Callback: Fn(&mut Data)> Clickable<Data, NormalChild, ChildOnClicked, Callback> {
    pub(crate) fn new(mouse_button: MouseButton, on_click: Callback, normal_child: NormalChild, child_on_clicked: ChildOnClicked) -> Self {
        Self { mouse_button, on_click, normal_child, child_on_clicked, _phantom: PhantomData }
    }
}

impl<Data, NormalChild: Widget<Data>, ChildOnClicked: Widget<Data>, Callback: Fn(&mut Data)> Widget<Data>
    for Clickable<Data, NormalChild, ChildOnClicked, Callback>
{
    type ActualWidget =
        ClickableActualWidget<Data, <NormalChild as Widget<Data>>::ActualWidget, <ChildOnClicked as Widget<Data>>::ActualWidget, Callback>;

    fn to_actual_widget(self, id_maker: &mut ActualWidgetIdMaker) -> Self::ActualWidget {
        ClickableActualWidget {
            id: id_maker.next_id(),
            mouse_button: self.mouse_button,
            on_click: self.on_click,
            normal_child: self.normal_child.to_actual_widget(id_maker),
            child_on_clicked: self.child_on_clicked.to_actual_widget(id_maker),
            clicked: false,
            _phantom: PhantomData,
            _private: (),
        }
    }

    fn update_actual_widget(self, actual_widget: &mut Self::ActualWidget, id_maker: &mut ActualWidgetIdMaker) {
        actual_widget.mouse_button = self.mouse_button;
        actual_widget.on_click = self.on_click;
        self.normal_child.update_actual_widget(&mut actual_widget.normal_child, id_maker);
        self.child_on_clicked.update_actual_widget(&mut actual_widget.child_on_clicked, id_maker);
    }
}

impl<Data, NormalChild: ActualWidget<Data>, ChildOnClicked: ActualWidget<Data>, Callback: Fn(&mut Data)> ActualWidget<Data>
    for ClickableActualWidget<Data, NormalChild, ChildOnClicked, Callback>
{
    fn layout(&mut self, graphics_context: &graphics::GraphicsContext, sc: layout::SizeConstraints) {
        self.normal_child.layout(graphics_context, sc);
        self.child_on_clicked.layout(graphics_context, sc);
    }

    fn draw(
        &self,
        graphics_context: &graphics::GraphicsContext,
        target: &mut dyn graphics::RenderTarget,
        top_left: graphics::Vector2f,
        hover: &HashSet<ActualWidgetId>,
    ) {
        if self.clicked {
            self.child_on_clicked.draw(graphics_context, target, top_left, hover);
        } else {
            self.normal_child.draw(graphics_context, target, top_left, hover);
        }
    }

    fn find_hover(&self, top_left: graphics::Vector2f, mouse: graphics::Vector2f) -> Box<(dyn Iterator<Item = (ActualWidgetId, bool)> + '_)> {
        Box::new(
            if graphics::FloatRect::from_vecs(top_left, self.size()).contains(mouse) { Some((self.id, true)) } else { None }
                .into_iter()
                .chain(if self.clicked { self.child_on_clicked.find_hover(top_left, mouse) } else { self.normal_child.find_hover(top_left, mouse) }),
        )
    }

    fn size(&self) -> graphics::Vector2f {
        if self.clicked {
            self.child_on_clicked.size()
        } else {
            self.normal_child.size()
        }
    }

    fn dispatch_event(&mut self, top_left: graphics::Vector2f, data: &mut Data, target: ActualWidgetId, event: event::TargetedEvent) {
        if target == self.id {
            self.targeted_event(top_left, data, event);
        }

        if self.clicked {
            self.child_on_clicked.dispatch_event(top_left, data, target, event);
        } else {
            self.normal_child.dispatch_event(top_left, data, target, event);
        }
    }

    fn targeted_event(&mut self, _: graphics::Vector2f, _: &mut Data, event: event::TargetedEvent) {
        match event {
            event::TargetedEvent::LeftMouseDown(_) => {
                if self.mouse_button == MouseButton::Left {
                    self.clicked = true;
                }
            }
            event::TargetedEvent::RightMouseDown(_) => {
                if self.mouse_button == MouseButton::Right {
                    self.clicked = true;
                }
            }
        }
    }
    fn general_event(&mut self, top_left: graphics::Vector2f, data: &mut Data, event: event::GeneralEvent) {
        match event {
            event::GeneralEvent::MouseMoved(new_mouse_pos) => {
                if !graphics::FloatRect::from_vecs(top_left, self.size()).contains(new_mouse_pos) {
                    self.clicked = false;
                }
            }
            event::GeneralEvent::LeftMouseUp => {
                if self.mouse_button == MouseButton::Left && self.clicked {
                    self.clicked = false;
                    (self.on_click)(data);
                }
            }
            event::GeneralEvent::RightMouseUp => {
                if self.mouse_button == MouseButton::Right && self.clicked {
                    self.clicked = false;
                    (self.on_click)(data);
                }
            }
            event::GeneralEvent::KeyPressed { .. } => {}
        }

        if self.clicked {
            self.child_on_clicked.general_event(top_left, data, event);
        } else {
            self.normal_child.general_event(top_left, data, event);
        }
    }
}
