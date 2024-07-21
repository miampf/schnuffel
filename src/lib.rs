use iced::{mouse::ScrollDelta, Point};

pub mod plugin;
pub mod views;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    MouseClick(Point),
    MouseDrag(Point),
    MouseRelease,
    MouseScroll(ScrollDelta),
    MiddleMouseClick(Point),
}
