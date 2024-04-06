use std::collections::HashSet;

use sfml::{
    graphics::{RenderTarget, RenderWindow},
    window::{Event, Style},
};

use crate::{
    actual_widget::{ActualWidget, ActualWidgetIdMaker},
    event::{GeneralEvent, TargetedEvent},
    graphics::{self},
    layout::SizeConstraints,
    widgets::Widget,
};

pub fn run<Model, ModelAsWidget: Widget<Model>>(window_name: &'static str, window_size: (u32, u32), mut model: Model, model_to_widget: impl Fn(&Model) -> ModelAsWidget) {
    let mut id_maker = ActualWidgetIdMaker::new();
    let graphics_context = {
        let fonts = {
            // TODO: don't panic?
            let text_font_handle = font_kit::source::SystemSource::new()
                .select_best_match(&[font_kit::family_name::FamilyName::SansSerif, font_kit::family_name::FamilyName::Serif], &font_kit::properties::Properties::new())
                .expect("could not find appropriate text font font");
            let text_font = match text_font_handle {
                font_kit::handle::Handle::Path { path, font_index: _ } => sfml::graphics::Font::from_file(&path.to_string_lossy()).expect("could not load font"), // TODO: figure out how to handle font_index
                font_kit::handle::Handle::Memory { bytes: _, font_index: _ } => unimplemented!("loading font from memory"),
            };

            let monospace_font_handle = font_kit::source::SystemSource::new()
                .select_best_match(&[font_kit::family_name::FamilyName::Monospace], &font_kit::properties::Properties::new())
                .expect("could not find appropriate monospace font");
            let monospace_font = match monospace_font_handle {
                font_kit::handle::Handle::Path { path, font_index: _ } => sfml::graphics::Font::from_file(&path.to_string_lossy()).expect("could not load font"), // TODO: figure out how to handle font_index
                font_kit::handle::Handle::Memory { bytes: _, font_index: _ } => unimplemented!("loading font from memory"),
            };

            graphics::Fonts { text_font, monospace_font }
        };

        graphics::GraphicsContext { default_render_context_settings: sfml::window::ContextSettings { antialiasing_level: 0, ..Default::default() }, fonts }
    };

    let mut actual_widget = model_to_widget(&model).to_actual_widget(&mut id_maker);

    let mut window = RenderWindow::new(window_size, window_name, Style::DEFAULT, &graphics_context.default_render_context_settings);
    window.set_vertical_sync_enabled(true);

    while window.is_open() {
        // TODO: having this variable doesnt seem right
        let view_top_left = graphics::Vector2f::new(0.0, 0.0);

        // events
        while let Some(event) = window.poll_event() {
            match event {
                // TODO: put these in the event handler with everything else
                Event::Closed => window.close(),
                Event::Resized { width, height } => {
                    // update the view to the new size of the window
                    let visible_area = graphics::FloatRect::new(0.0, 0.0, width as f32, height as f32);
                    window.set_view(&sfml::graphics::View::from_rect(visible_area));
                }

                sfml::window::Event::MouseButtonPressed { button, x, y } => {
                    let mouse_position = graphics::Vector2f::new(x as f32, y as f32);
                    let hovered: Vec<_> = actual_widget.find_hover(view_top_left, mouse_position).collect();
                    for (hovered, clicks_can_pass_through) in hovered {
                        match button {
                            sfml::window::mouse::Button::Left => actual_widget.dispatch_event(view_top_left, &mut model, hovered, TargetedEvent::LeftMouseDown(mouse_position)),
                            sfml::window::mouse::Button::Right => actual_widget.dispatch_event(view_top_left, &mut model, hovered, TargetedEvent::RightMouseDown(mouse_position)),
                            _ => {}
                        }
                        if !clicks_can_pass_through {
                            break;
                        }
                    }
                }

                sfml::window::Event::MouseMoved { x, y } => actual_widget.general_event(view_top_left, &mut model, GeneralEvent::MouseMoved(graphics::Vector2f::new(x as f32, y as f32))),

                sfml::window::Event::MouseButtonReleased { button: sfml::window::mouse::Button::Left, x: _, y: _ } => actual_widget.general_event(view_top_left, &mut model, GeneralEvent::LeftMouseUp),
                sfml::window::Event::MouseButtonReleased { button: sfml::window::mouse::Button::Right, x: _, y: _ } => {
                    actual_widget.general_event(view_top_left, &mut model, GeneralEvent::RightMouseUp);
                }

                // TODO: proper event dispatch (including reworking the mouse events above)
                sfml::window::Event::KeyPressed { code, alt, ctrl, shift, system, scan } => {
                    actual_widget.general_event(view_top_left, &mut model, GeneralEvent::KeyPressed { code, alt, ctrl, shift, system });
                }

                _ => {}
            }
        }

        // draw
        window.set_active(true);
        model_to_widget(&model).update_actual_widget(&mut actual_widget, &mut id_maker);

        let size_constraints = SizeConstraints { min: graphics::Vector2f::new(0.0, 0.0), max: window.size().as_other() };

        actual_widget.layout(&graphics_context, size_constraints);

        let mouse_position = window.mouse_position().as_other();
        let hovered = {
            let mut hovered = HashSet::new();
            let iter = actual_widget.find_hover(view_top_left, mouse_position);
            for (actual_widget, clicks_can_pass_through) in iter {
                hovered.insert(actual_widget);
                if !clicks_can_pass_through {
                    break;
                }
            }
            hovered
        };

        window.clear(graphics::Color::BLACK);
        actual_widget.draw(&graphics_context, &mut window, view_top_left, &hovered);

        window.display();
    }
}
