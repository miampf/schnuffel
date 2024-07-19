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
    MouseRelease,
}

#[derive(Debug, Clone)]
struct VisualNode {
    pub node: Node,
    pub id: usize,
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub is_clicked: bool,
}

impl Default for VisualNode {
    fn default() -> Self {
        Self {
            node: Node::Person("Foo Bar".to_string()),
            id: 0,
            x: 0.0,
            y: 0.0,
            radius: 10.0,
            is_clicked: false,
        }
    }
}

#[derive(Debug, Clone, Default)]
struct VisualEdge {
    from: usize,
    to: usize,
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
                        < node.radius.powf(2.0)
                    {
                        node.is_clicked = true;
                    }
                }
                self.state.update_values(self.state.graph.clone());
            }
            Message::MouseDrag(position) => {
                for node in &mut self.state.graph.nodes {
                    if node.is_clicked {
                        node.x = position.x;
                        node.y = position.y;
                    }
                }
                self.state.update_values(self.state.graph.clone());
            }
            Message::MouseRelease => {
                for node in &mut self.state.graph.nodes {
                    node.is_clicked = false;
                }
                self.state.update_values(self.state.graph.clone());
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
            graph: VisualGraph {
                nodes: vec![
                    VisualNode {
                        id: 0,
                        x: 50.0,
                        y: 50.0,
                        radius: 15.0,
                        ..Default::default()
                    },
                    VisualNode {
                        id: 1,
                        x: 100.0,
                        y: 100.0,
                        radius: 20.0,
                        ..Default::default()
                    },
                ],
                edges: vec![VisualEdge { from: 0, to: 1 }],
            },
        }
    }

    pub fn update_values(&mut self, graph: VisualGraph) {
        self.graph = graph;
        self.graph_cache.clear();
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
        // draw the graph
        let graph = self.graph_cache.draw(renderer, bounds.size(), |frame| {
            for node in &self.graph.nodes {
                let to_draw = Path::circle(Point::new(node.x, node.y), node.radius);
                frame.fill(&to_draw, Color::BLACK);
            }
            for edge in &self.graph.edges {
                let from_node = self.graph.nodes.iter().find(|n| n.id == edge.from).unwrap();
                let to_node = self.graph.nodes.iter().find(|n| n.id == edge.to).unwrap();
                let to_draw = Path::line(
                    Point::new(from_node.x, from_node.y),
                    Point::new(to_node.x, to_node.y),
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
                        // a node was clicked
                        Some(position) => (
                            canvas::event::Status::Captured,
                            Some(Message::MouseClick(position)),
                        ),
                        None => uncaptured,
                    },
                    _ => uncaptured,
                },
                mouse::Event::CursorMoved { position } => (
                    // if a node is clicked this will move the node
                    canvas::event::Status::Captured,
                    Some(Message::MouseDrag(position)),
                ),
                mouse::Event::ButtonReleased(button) => match button {
                    // this releases all nodes
                    mouse::Button::Left => {
                        (canvas::event::Status::Captured, Some(Message::MouseRelease))
                    }
                    _ => uncaptured,
                },
                _ => uncaptured,
            },
            _ => uncaptured,
        }
    }
}

fn main() -> iced::Result {
    App::run(Settings {
        antialiasing: true,
        ..Default::default()
    })
}
