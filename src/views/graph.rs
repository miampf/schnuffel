use crate::Message;
use iced::mouse;
use iced::widget::canvas::{
    self,
    stroke::{self, Stroke},
    Cache, Path, Program,
};
use iced::{Color, Point, Rectangle};
use schnuffel_types::graph::Node;

use super::ViewState;

#[derive(Debug, Default)]
pub struct GraphState {
    pub graph_cache: Cache,
    pub graph: VisualGraph,
}

impl ViewState for GraphState {
    type UpdateType = VisualGraph;

    fn update_state(&mut self, new: VisualGraph) {
        self.graph = new;
        self.graph_cache.clear();
    }
}

#[derive(Debug, Clone)]
pub struct VisualNode {
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
pub struct VisualEdge {
    pub from: usize,
    pub to: usize,
}

#[derive(Debug, Clone)]
pub struct VisualGraph {
    pub nodes: Vec<VisualNode>,
    pub edges: Vec<VisualEdge>,
}

impl Default for VisualGraph {
    fn default() -> Self {
        Self {
            nodes: vec![
                VisualNode {
                    id: 0,
                    x: 10.0,
                    y: 10.0,
                    radius: 5.0,
                    ..Default::default()
                },
                VisualNode {
                    id: 1,
                    x: 50.0,
                    y: 50.0,
                    radius: 10.0,
                    ..Default::default()
                },
            ],
            edges: vec![VisualEdge { from: 0, to: 1 }],
        }
    }
}

impl Program<Message> for GraphState {
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
