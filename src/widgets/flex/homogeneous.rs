use std::{collections::HashSet, marker::PhantomData};

use crate::{
    actual_widget::{animated::Animated, ActualWidget, ActualWidgetId, ActualWidgetIdMaker},
    event::{GeneralEvent, TargetedEvent},
    graphics, layout,
    widgets::{
        flex::{Direction, ItemSettings, _layout},
        Widget,
    },
};

pub(crate) struct Flex<Data, Child: Widget<Data>> {
    direction: Direction,
    children: Vec<(ItemSettings, Child)>,

    _phantom: PhantomData<fn(&mut Data)>,
}
pub(crate) struct FlexActualWidget<Data, Child: ActualWidget<Data>> {
    direction: Direction,
    children: Vec<(Animated<ItemSettings>, graphics::Vector2f, Child)>,

    own_size: graphics::Vector2f,

    _phantom: PhantomData<fn(&mut Data)>,
    _private: (),
}

impl<Data, Child: Widget<Data>> Flex<Data, Child> {
    pub(crate) fn new(direction: Direction, children: Vec<(ItemSettings, Child)>) -> Self {
        Self { direction, children, _phantom: PhantomData }
    }
    pub(crate) fn new_horizontal(children: Vec<(ItemSettings, Child)>) -> Self {
        Self::new(Direction::Horizontal, children)
    }
    pub(crate) fn new_vertical(children: Vec<(ItemSettings, Child)>) -> Self {
        Self::new(Direction::Vertical, children)
    }
}

impl<Data, Child: Widget<Data>> Widget<Data> for Flex<Data, Child> {
    type ActualWidget = FlexActualWidget<Data, <Child as Widget<Data>>::ActualWidget>;

    fn to_actual_widget(self, id_maker: &mut ActualWidgetIdMaker) -> Self::ActualWidget {
        FlexActualWidget {
            direction: self.direction,
            children: self.children.into_iter().map(|(settings, child)| (Animated::new(settings), graphics::Vector2f::new(0.0, 0.0), child.to_actual_widget(id_maker))).collect(),
            own_size: graphics::Vector2f::new(0.0, 0.0),
            _phantom: PhantomData,
            _private: (),
        }
    }

    fn update_actual_widget(self, actual_widget: &mut Self::ActualWidget, id_maker: &mut ActualWidgetIdMaker) {
        let ro_children = std::mem::take(&mut actual_widget.children);
        let ro_children_infinite = ro_children.into_iter().map(Some).chain(std::iter::repeat_with(|| None));

        let new_ro_children = self
            .children
            .into_iter()
            .zip(ro_children_infinite)
            .map(|((settings, widget), ro)| match ro {
                Some((mut old_settings, offset, mut ro)) => {
                    widget.update_actual_widget(&mut ro, id_maker);
                    old_settings.set(settings);
                    (old_settings, offset, ro)
                }
                None => (Animated::new(settings), graphics::Vector2f::new(0.0, 0.0), widget.to_actual_widget(id_maker)),
            })
            .collect();

        actual_widget.children = new_ro_children;
    }
}
impl<Data, Child: ActualWidget<Data>> ActualWidget<Data> for FlexActualWidget<Data, Child> {
    fn layout(&mut self, graphics_context: &graphics::GraphicsContext, sc: layout::SizeConstraints) {
        let phase1_result = _layout::phase1(
            graphics_context,
            sc,
            self.direction,
            self.children.iter_mut().map(move |(settings, _, child)| (_layout::animated_settings(*settings), child as &mut dyn ActualWidget<Data>)),
        );
        _layout::phase2(
            graphics_context,
            sc,
            self.direction,
            phase1_result,
            self.children.iter_mut().map(move |(settings, _, child)| (_layout::animated_settings(*settings), child as &mut dyn ActualWidget<Data>)),
        );
        self.own_size = _layout::phase3(sc, self.direction, self.children.iter_mut().map(move |(_, offset, child)| (offset, child as &mut dyn ActualWidget<Data>)));
    }

    fn draw(&self, graphics_context: &graphics::GraphicsContext, target: &mut dyn graphics::RenderTarget, top_left: graphics::Vector2f, hover: &HashSet<ActualWidgetId>) {
        for (_, offset, child) in &self.children {
            child.draw(graphics_context, target, top_left + *offset, hover);
        }
    }

    fn find_hover(&self, top_left: graphics::Vector2f, mouse: graphics::Vector2f) -> Box<(dyn Iterator<Item = (ActualWidgetId, bool)> + '_)> {
        Box::new(self.children.iter().flat_map(move |(_, offset, child)| child.find_hover(top_left + *offset, mouse)))
    }

    fn size(&self) -> graphics::Vector2f {
        self.own_size
    }

    fn dispatch_event(&mut self, top_left: graphics::Vector2f, data: &mut Data, target: ActualWidgetId, event: TargetedEvent) {
        for (_, offset, child) in &mut self.children {
            child.dispatch_event(top_left + *offset, data, target, event);
        }
    }

    fn targeted_event(&mut self, _: graphics::Vector2f, _: &mut Data, _: TargetedEvent) {}
    fn general_event(&mut self, top_left: graphics::Vector2f, data: &mut Data, event: GeneralEvent) {
        for (_, _, child) in &mut self.children {
            child.general_event(top_left, data, event);
        }
    }
}
