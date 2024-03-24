use crate::graphics;

#[derive(Copy, Clone, PartialEq)]
pub struct SizeConstraints {
    pub min: graphics::Vector2f,
    pub max: graphics::Vector2f,
}
impl SizeConstraints {
    pub fn with_no_min(&self) -> SizeConstraints {
        SizeConstraints { min: graphics::Vector2f::new(0.0, 0.0), max: self.max }
    }

    pub fn clamp_size(&self, size: graphics::Vector2f) -> graphics::Vector2f {
        graphics::Vector2f::new(size.x.clamp(self.min.x, self.max.x), size.y.clamp(self.min.y, self.max.y))
    }

    pub fn shrink(&self, amount: graphics::Vector2f) -> SizeConstraints {
        SizeConstraints {
            min: graphics::Vector2f::new((self.min.x - amount.x).max(0.0), (self.min.y - amount.y).max(0.0)),
            max: graphics::Vector2f::new((self.max.x - amount.x).max(0.0), (self.max.y - amount.y).max(0.0)),
        }
    }
}
