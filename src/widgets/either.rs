use std::{marker::PhantomData, collections::HashSet};

use crate::{
    event::{GeneralEvent, TargetedEvent},
    graphics, layout,
    actual_widget::{ActualWidget, ActualWidgetId, ActualWidgetIdMaker},
    widgets::Widget,
};

pub enum Either<Data, Left: Widget<Data>, Right: Widget<Data>> {
    Left(Left),
    Right(Right, PhantomData<fn(&mut Data)>),
}

pub enum EitherActualWidget<Data, Left: ActualWidget<Data>, Right: ActualWidget<Data>> {
    Left(Left),
    Right(Right, PhantomData<fn(&mut Data)>),
}

impl<Data, Left: Widget<Data>, Right: Widget<Data>> Either<Data, Left, Right> {
    pub(crate) fn new_left(left: Left) -> Self {
        Self::Left(left)
    }
    pub(crate) fn new_right(right: Right) -> Self {
        Self::Right(right, PhantomData)
    }
}

impl<Data, Left: Widget<Data>, Right: Widget<Data>> Widget<Data> for Either<Data, Left, Right> {
    type ActualWidget = EitherActualWidget<Data, <Left as Widget<Data>>::ActualWidget, <Right as Widget<Data>>::ActualWidget>;

    fn to_actual_widget(self, id_maker: &mut ActualWidgetIdMaker) -> Self::ActualWidget {
        match self {
            Either::Left(l) => EitherActualWidget::Left(l.to_actual_widget(id_maker)),
            Either::Right(r, phantom) => EitherActualWidget::Right(r.to_actual_widget(id_maker), phantom),
        }
    }

    fn update_actual_widget(self, actual_widget: &mut Self::ActualWidget, id_maker: &mut ActualWidgetIdMaker) {
        match (self, actual_widget) {
            (Either::Left(widget), EitherActualWidget::Left(ro)) => widget.update_actual_widget(ro, id_maker),
            (Either::Right(widget, _), EitherActualWidget::Right(ro, _)) => widget.update_actual_widget(ro, id_maker),
            (self_, actual_widget) => *actual_widget = self_.to_actual_widget(id_maker),
        }
    }
}
impl<Data, Left: ActualWidget<Data>, Right: ActualWidget<Data>> ActualWidget<Data> for EitherActualWidget<Data, Left, Right> {
    fn layout(&mut self, graphics_context: &graphics::GraphicsContext, sc: layout::SizeConstraints) {
        match self {
            EitherActualWidget::Left(l) => l.layout(graphics_context, sc),
            EitherActualWidget::Right(r, _) => r.layout(graphics_context, sc),
        }
    }

    fn draw(&self, graphics_context: &graphics::GraphicsContext, target: &mut dyn graphics::RenderTarget, top_left: graphics::Vector2f, hover: &HashSet<ActualWidgetId>) {
        match self {
            EitherActualWidget::Left(l) => l.draw(graphics_context, target, top_left, hover),
            EitherActualWidget::Right(r, _) => r.draw(graphics_context, target, top_left, hover),
        }
    }

    fn find_hover(&self, top_left: graphics::Vector2f, mouse: graphics::Vector2f) -> Box<(dyn Iterator<Item = (ActualWidgetId, bool)> + '_)> {
        match self {
            EitherActualWidget::Left(l) => l.find_hover(top_left, mouse),
            EitherActualWidget::Right(r, _) => r.find_hover(top_left, mouse),
        }
    }

    fn size(&self) -> graphics::Vector2f {
        match self {
            EitherActualWidget::Left(l) => l.size(),
            EitherActualWidget::Right(r, _) => r.size(),
        }
    }

    fn dispatch_event(&mut self, top_left: graphics::Vector2f, data: &mut Data, target: ActualWidgetId, event: TargetedEvent) {
        match self {
            EitherActualWidget::Left(l) => l.dispatch_event(top_left, data, target, event),
            EitherActualWidget::Right(r, _) => r.dispatch_event(top_left, data, target, event),
        }
    }

    fn targeted_event(&mut self, _: graphics::Vector2f, _: &mut Data, _: TargetedEvent) {}
    fn general_event(&mut self, top_left: graphics::Vector2f, data: &mut Data, event: GeneralEvent) {
        match self {
            EitherActualWidget::Left(l) => l.general_event(top_left, data, event),
            EitherActualWidget::Right(r, _) => r.general_event(top_left, data, event),
        }
    }
}
