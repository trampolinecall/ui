// graphics utilities

#[allow(unused_imports)]
pub use sfml::{
    graphics::{CircleShape, Color, FloatRect, Font, Rect, RectangleShape, RenderTarget, RenderTexture, RenderWindow, Sprite, Text, Transformable},
    system::{Vector2, Vector2f, Vector2i, Vector2u},
    SfBox,
};

pub trait RectCenter<T> {
    fn center(&self) -> Vector2<T>;
}
impl<T: std::ops::Div + std::ops::Add<<T as std::ops::Div>::Output, Output = T> + From<i8> + Copy> RectCenter<T> for Rect<T> {
    fn center(&self) -> Vector2<T> {
        Vector2::new(self.left + self.width / 2.into(), self.top + self.height / 2.into())
    }
}
pub trait CenterText {
    fn center(&mut self);
    fn center_horizontally(&mut self);
    fn center_vertically(&mut self);
}
impl CenterText for Text<'_> {
    fn center(&mut self) {
        let bounds = self.local_bounds();
        self.set_origin((bounds.width / 2.0, bounds.height / 2.0));
    }

    fn center_horizontally(&mut self) {
        let bounds = self.local_bounds();
        self.set_origin((bounds.width / 2.0, self.origin().y));
    }

    fn center_vertically(&mut self) {
        let boudns = self.local_bounds();
        self.set_origin((self.origin().x, boudns.height / 2.0));
    }
}

pub struct GraphicsContext {
    pub default_render_context_settings: sfml::window::ContextSettings,
    pub fonts: Fonts,
}

pub struct Fonts {
    pub text_font: SfBox<Font>,
    pub monospace_font: SfBox<Font>,
}

impl Fonts {
    pub fn text_font(&self) -> &Font {
        &self.text_font
    }

    pub fn monospace_font(&self) -> &Font {
        &self.monospace_font
    }
}
