use iced::executor;
use iced::mouse;
use iced::theme::Theme;
use iced::widget::canvas::Program;
use iced::widget::canvas::{
    self,
    stroke::{self, Stroke},
    Cache, Canvas, Path,
};
use iced::Settings;
use iced::{Application, Color, Command, Element, Length, Point, Rectangle};
use schnuffel_types::graph::Node;

struct App {
    state: State,
}

#[derive(Debug)]
struct State {
    graph_cache: Cache,
    graph: VisualGraph,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    MouseClick(Point),
    MouseDrag(Point),
}

#[derive(Debug, Clone)]
struct VisualNode {
    pub node: Node,
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub is_clicked: bool,
}

impl Default for VisualNode {
    fn default() -> Self {
        Self {
            node: Node::Person("John Doe".to_string()),
            x: 0.0,
            y: 0.0,
            radius: 10.0,
            is_clicked: false,
        }
    }
}

#[derive(Debug, Clone, Default)]
struct VisualEdge {
    from: VisualNode,
    to: VisualNode,
}

#[derive(Debug, Clone, Default)]
struct VisualGraph {
    nodes: Vec<VisualNode>,
    edges: Vec<VisualEdge>,
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            App {
                state: State::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("schnuffel")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::MouseClick(position) => {
                for node in &mut self.state.graph.nodes {
                    if (position.x - node.x).powf(2.0) + (position.y - node.y).powf(2.0)
                        < node.radius
                    {
                        node.is_clicked = true;
                    }
                }
            }
            Message::MouseDrag(position) => {
                for node in &mut self.state.graph.nodes {
                    if node.is_clicked {
                        node.x = position.x;
                        node.y = position.y;
                    }
                }
            }
        };
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        Canvas::new(&self.state)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::Light
    }
}

impl State {
    pub fn new() -> Self {
        Self {
            graph_cache: Cache::default(),
            graph: VisualGraph::default(),
        }
    }
}

impl Program<Message> for State {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced_renderer::Renderer,
        _theme: &iced_style::Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<<iced_renderer::Renderer as canvas::Renderer>::Geometry> {
        let graph = self.graph_cache.draw(renderer, bounds.size(), |frame| {
            for node in &self.graph.nodes {
                let to_draw = Path::circle(Point::new(node.x, node.y), node.radius);
                frame.fill(&to_draw, Color::BLACK);
            }
            for edge in &self.graph.edges {
                let to_draw = Path::line(
                    Point::new(edge.from.x, edge.from.y),
                    Point::new(edge.to.x, edge.to.y),
                );
                frame.stroke(
                    &to_draw,
                    Stroke {
                        style: stroke::Style::Solid(Color::BLACK),
                        width: 1.0,
                        ..Stroke::default()
                    },
                );
            }
        });
        vec![graph]
    }

    fn update(
        &self,
        _state: &mut Self::State,
        event: canvas::Event,
        _bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> (canvas::event::Status, Option<Message>) {
        let uncaptured = (canvas::event::Status::Ignored, None);
        match event {
            canvas::Event::Mouse(event) => match event {
                mouse::Event::ButtonPressed(button) => match button {
                    mouse::Button::Left => match cursor.position() {
                        Some(position) => (
                            canvas::event::Status::Captured,
                            Some(Message::MouseClick(position)),
                        ),
                        None => uncaptured,
                    },
                    _ => uncaptured,
                },
                mouse::Event::CursorMoved { position } => (
                    canvas::event::Status::Captured,
                    Some(Message::MouseDrag(position)),
                ),
                _ => uncaptured,
            },
            _ => uncaptured,
        }
    }
}

fn main() -> iced::Result {
    App::run(Settings::default())
}
