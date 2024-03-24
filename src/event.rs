use crate::graphics;

#[derive(Copy, Clone)]
pub enum TargetedEvent {
    LeftMouseDown(graphics::Vector2f),
    RightMouseDown(graphics::Vector2f),
}
#[derive(Copy, Clone)]
pub enum GeneralEvent {
    MouseMoved(graphics::Vector2f),
    LeftMouseUp,
    RightMouseUp,
    KeyPressed { code: sfml::window::Key, alt: bool, ctrl: bool, shift: bool, system: bool },
}
