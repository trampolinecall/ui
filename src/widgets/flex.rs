use crate::graphics;

#[macro_use]
pub mod fixed_amount;
pub mod homogeneous;

pub mod _layout {
    // TODO: somehow figure out how to not make this need to be exported
    use crate::{
        actual_widget::{
            animated::{Animated, AnimatedValue, Lerpable},
            ActualWidget,
        },
        graphics::{self, GraphicsContext},
        layout::SizeConstraints,
        widgets::flex::{Direction, ItemSettings},
    };

    #[inline]
    pub fn animated_settings(settings: Animated<ItemSettings>) -> ItemSettings {
        match settings.get() {
            AnimatedValue::Steady(s) => *s,
            AnimatedValue::Animating { before: ItemSettings::Flex(before_flex), after: ItemSettings::Flex(after_flex), amount } => ItemSettings::Flex(before_flex.lerp(after_flex, amount)),
            AnimatedValue::Animating { before: _, after, amount: _ } => *after,
        }
    }

    // phase 1 of flex layout: lay out fixed elements and count up total flex scaling factors
    pub fn phase1<'w, 'o, 'c, 'p, Data: 'w>(
        graphics_context: &GraphicsContext,
        sc: SizeConstraints,
        direction: Direction,
        items: impl Iterator<Item = (ItemSettings, &'w mut dyn ActualWidget<Data>)>,
    ) -> (f32, f32) {
        let mut total_flex_scale = 0.0;
        let mut major_size_left = direction.take_major_component(sc.max);
        for (settings, child) in items {
            match settings {
                ItemSettings::Fixed => {
                    child.layout(graphics_context, sc.with_no_min());
                    major_size_left -= direction.take_major_component(child.size());
                }
                ItemSettings::Flex(scale) => {
                    total_flex_scale += scale;
                }
            };
        }
        (total_flex_scale, major_size_left)
    }

    // phase 2 of flex layout: lay out all of the flex children
    pub fn phase2<'w, 'o, 'c, 'p, Data: 'w>(
        graphics_context: &GraphicsContext,
        sc: SizeConstraints,
        direction: Direction,
        (total_flex_scale, major_size_left): (f32, f32), // phase 1 output
        items: impl Iterator<Item = (ItemSettings, &'w mut dyn ActualWidget<Data>)>,
    ) {
        for (settings, child) in items {
            if let ItemSettings::Flex(scale) = settings {
                let child_sc = SizeConstraints {
                    min: graphics::Vector2f::new(0.0, 0.0),
                    max: direction.make_vector_in_direction(scale / total_flex_scale * major_size_left, direction.take_minor_component(sc.max)),
                };
                child.layout(graphics_context, child_sc);
            };
        }
    }

    // phase 3 of flex layout: assign each of the offsets and calcaulte own_size
    pub fn phase3<'w, 'o, 'c, 'p, Data: 'w>(
        sc: SizeConstraints,
        direction: Direction,
        items: impl Iterator<Item = (&'o mut graphics::Vector2f, &'w mut dyn ActualWidget<Data>)>,
    ) -> graphics::Vector2f {
        let mut major_offset = 0.0;
        let mut max_minor_size = 0.0;
        for (offset, child) in items {
            let calculated_offset = direction.make_vector_in_direction(major_offset, 0.0);
            major_offset += direction.take_major_component(child.size());
            let item_minor_size = direction.take_minor_component(child.size());
            max_minor_size = if item_minor_size > max_minor_size { item_minor_size } else { max_minor_size };
            *offset = calculated_offset;
        }
        sc.clamp_size(direction.make_vector_in_direction(major_offset, max_minor_size))
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum ItemSettings {
    Fixed,
    Flex(f32),
}

#[derive(Copy, Clone)]
pub enum Direction {
    Horizontal,
    Vertical,
}

impl Direction {
    pub fn make_vector_in_direction<T>(&self, major_component: T, minor_component: T) -> graphics::Vector2<T> {
        match self {
            Direction::Horizontal => graphics::Vector2::new(major_component, minor_component),
            Direction::Vertical => graphics::Vector2::new(minor_component, major_component),
        }
    }

    pub fn take_major_component<T>(&self, v: graphics::Vector2<T>) -> T {
        match self {
            Direction::Horizontal => v.x,
            Direction::Vertical => v.y,
        }
    }

    pub fn take_minor_component<T>(&self, v: graphics::Vector2<T>) -> T {
        match self {
            Direction::Horizontal => v.y,
            Direction::Vertical => v.x,
        }
    }
}
