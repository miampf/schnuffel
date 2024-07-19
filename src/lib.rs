use iced::Point;

pub mod plugin;
pub mod views;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    MouseClick(Point),
    MouseDrag(Point),
    MouseRelease,
}
