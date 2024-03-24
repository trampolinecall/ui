use crate::graphics;

#[macro_use]
pub mod fixed_amount;
pub mod homogeneous;

pub mod _layout {
    // TODO: somehow figure out how to not make this need to be exported
    use crate::{
        graphics::{self, GraphicsContext},
        layout::SizeConstraints,
        actual_widget::{
            animated::{Animated, AnimatedValue, Lerpable},
            ActualWidget,
        },
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

    #[inline]
    pub fn first_phase_step<Data>(
        graphics_context: &GraphicsContext,
        sc: SizeConstraints,
        direction: Direction,
        total_flex_scale: &mut f32,
        major_size_left: &mut f32,
        item_settings: ItemSettings,
        item: &mut impl ActualWidget<Data>,
    ) {
        match item_settings {
            ItemSettings::Fixed => {
                item.layout(graphics_context, sc.with_no_min());
                *major_size_left -= direction.take_major_component(item.size());
            }
            ItemSettings::Flex(scale) => {
                *total_flex_scale += scale;
            }
        }
    }

    #[inline]
    pub fn second_phase_step<Data>(
        graphics_context: &GraphicsContext,
        sc: SizeConstraints,
        direction: Direction,
        total_flex_scale: f32,
        major_size_left: f32,
        item_settings: ItemSettings,
        item: &mut impl ActualWidget<Data>,
    ) {
        if let ItemSettings::Flex(scale) = item_settings {
            let child_sc =
                SizeConstraints { min: graphics::Vector2f::new(0.0, 0.0), max: direction.make_vector_in_direction(scale / total_flex_scale * major_size_left, direction.take_minor_component(sc.max)) };
            item.layout(graphics_context, child_sc);
        }
    }

    #[inline]
    pub fn third_phase_step<Data>(direction: Direction, major_offset: &mut f32, max_minor_size: &mut f32, item: &impl ActualWidget<Data>) -> graphics::Vector2f {
        let offset = direction.make_vector_in_direction(*major_offset, 0.0);
        *major_offset += direction.take_major_component(item.size());
        let item_minor_size = direction.take_minor_component(item.size());
        *max_minor_size = if item_minor_size > *max_minor_size { item_minor_size } else { *max_minor_size };
        offset
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
