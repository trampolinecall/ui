use std::collections::HashSet;

use sfml::graphics::{Font, Transformable};

use crate::{
    actual_widget::{util, ActualWidget, ActualWidgetId, ActualWidgetIdMaker},
    event, graphics, layout,
    widgets::Widget,
};

// ideally this would just have a reference to the Font object but rust doenst support higher kinded type parameters so i couldnt get it to work
pub struct Label<GetFont: Fn(&graphics::Fonts) -> &Font> {
    text: String,
    get_font: GetFont,
    font_size: u32,
}
pub struct LabelActualWidget<GetFont: Fn(&graphics::Fonts) -> &Font> {
    id: ActualWidgetId,
    text: String,
    get_font: GetFont,
    font_size: u32,
    size: graphics::Vector2f,
    _private: (),
}

impl<GetFont: Fn(&graphics::Fonts) -> &Font> Label<GetFont> {
    pub fn new(text: String, get_font: GetFont, font_size: u32) -> Label<GetFont> {
        Label { text, get_font, font_size }
    }
}

impl<GetFont: Fn(&graphics::Fonts) -> &Font, Data> Widget<Data> for Label<GetFont> {
    type ActualWidget = LabelActualWidget<GetFont>;

    fn to_actual_widget(self, id_maker: &mut ActualWidgetIdMaker) -> Self::ActualWidget {
        LabelActualWidget { id: id_maker.next_id(), text: self.text, get_font: self.get_font, font_size: self.font_size, size: graphics::Vector2f::new(0.0, 0.0), _private: () }
    }

    fn update_actual_widget(self, actual_widget: &mut Self::ActualWidget, _: &mut ActualWidgetIdMaker) {
        actual_widget.text = self.text;
        actual_widget.get_font = self.get_font;
        actual_widget.font_size = self.font_size;
    }
}

impl<GetFont: Fn(&graphics::Fonts) -> &Font, Data> ActualWidget<Data> for LabelActualWidget<GetFont> {
    fn layout(&mut self, graphics_context: &graphics::GraphicsContext, sc: layout::SizeConstraints) {
        let text = graphics::Text::new(&self.text, (self.get_font)(&graphics_context.fonts), self.font_size);
        let global_bounds = text.global_bounds();
        self.size = sc.clamp_size(graphics::Vector2f::new(global_bounds.left + global_bounds.width, global_bounds.top + global_bounds.height));
    }

    fn draw(&self, graphics_context: &graphics::GraphicsContext, target: &mut dyn graphics::RenderTarget, top_left: graphics::Vector2f, _: &HashSet<ActualWidgetId>) {
        // TODO: deal with overflow better than by clipping
        // TODO: also fix messy rendering that is caused by clipping
        util::clip(graphics_context, target, graphics::FloatRect::from_vecs(top_left, self.size), |target, top_left| {
            let mut text = graphics::Text::new(&self.text, (self.get_font)(&graphics_context.fonts), self.font_size);
            text.set_position(top_left);
            text.set_fill_color(graphics::Color::WHITE); // TODO: control text color
            target.draw(&text);
        });
    }

    fn find_hover(&self, top_left: graphics::Vector2f, mouse: graphics::Vector2f) -> Box<(dyn Iterator<Item = (ActualWidgetId, bool)>)> {
        Box::new(if graphics::FloatRect::from_vecs(top_left, self.size).contains(mouse) { Some((self.id, true)) } else { None }.into_iter())
    }

    fn size(&self) -> graphics::Vector2f {
        self.size
    }

    fn send_targeted_event(&mut self, top_left: graphics::Vector2f, data: &mut Data, target: ActualWidgetId, event: event::TargetedEvent) {
        if target == self.id {
            self.targeted_event(top_left, data, event);
        }
    }

    fn targeted_event(&mut self, _: graphics::Vector2f, _: &mut Data, _: event::TargetedEvent) {}
    fn general_event(&mut self, _: graphics::Vector2f, _: &mut Data, _: event::GeneralEvent) {}
}
