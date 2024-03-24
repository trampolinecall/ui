pub mod animated;
pub mod util;

use std::collections::HashSet;

use crate::{event, graphics, layout};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct ActualWidgetId(u64);

pub struct ActualWidgetIdMaker(u64);
impl ActualWidgetIdMaker {
    pub(crate) fn new() -> ActualWidgetIdMaker {
        ActualWidgetIdMaker(0)
    }
    pub(crate) fn next_id(&mut self) -> ActualWidgetId {
        let id = ActualWidgetId(self.0);
        self.0 += 1;
        id
    }
}

pub trait ActualWidget<Data: ?Sized> {
    // TODO: automate send_targeted_event by having iter_children_by_z method?

    fn layout(&mut self, graphics_context: &graphics::GraphicsContext, sc: layout::SizeConstraints);
    fn draw(&self, graphics_context: &graphics::GraphicsContext, target: &mut dyn graphics::RenderTarget, top_left: graphics::Vector2f, hover: &HashSet<ActualWidgetId>);

    fn size(&self) -> graphics::Vector2f;

    fn find_hover(&self, top_left: graphics::Vector2f, mouse: graphics::Vector2f) -> Box<dyn Iterator<Item = (ActualWidgetId, bool)> + '_>;

    fn send_targeted_event(&mut self, top_left: graphics::Vector2f, data: &mut Data, target: ActualWidgetId, event: event::TargetedEvent);
    fn targeted_event(&mut self, top_left: graphics::Vector2f, data: &mut Data, event: event::TargetedEvent);
    fn general_event(&mut self, top_left: graphics::Vector2f, data: &mut Data, event: event::GeneralEvent);
}
