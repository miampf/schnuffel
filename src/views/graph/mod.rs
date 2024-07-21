pub mod constants;
pub mod helpers;

use crate::Message;
use iced::widget::row;
use iced::widget::scrollable::{Direction, Properties};
use iced::{mouse, Theme};
use iced::{
    widget::canvas::{
        self,
        stroke::{self, Stroke},
        Cache, Canvas, Path, Program,
    },
    Element,
};
use iced::{Color, Point, Rectangle};
use schnuffel_types::graph::{DNSRecord, Domain, Node};

use super::ViewState;
use helpers::build_info_column;

pub fn view(state: &GraphState) -> Element<'_, Message, Theme, iced::Renderer> {
    iced::widget::responsive(move |size| {
        row!(
            Canvas::new(state)
                .width((size.width / 3.0) * 2.0) // 2/3 of the space belong to the canvas
                .height(size.height),
            iced::widget::scrollable(build_info_column(state))
                .width(size.width / 3.0) // 1/3 of the space belongs to the node info
                .height(size.height)
                .direction(Direction::Vertical(Properties::default()))
        )
        .into()
    })
    .into()
}

#[derive(Debug)]
pub struct GraphState {
    pub graph_cache: Cache,
    pub graph: VisualGraph,
    pub zoom_factor: f32,
    pub is_panning: bool,
    pub panning_start_point: Point,
}

impl Default for GraphState {
    fn default() -> Self {
        Self {
            graph_cache: Cache::default(),
            graph: VisualGraph::default(),
            zoom_factor: 1.0,
            is_panning: false,
            panning_start_point: Point::default(),
        }
    }
}

pub struct GraphStateUpdate {
    pub graph: VisualGraph,
    pub zoom_factor: f32,
}

impl ViewState for GraphState {
    type UpdateType = GraphStateUpdate;

    fn update_state(&mut self, new: GraphStateUpdate) {
        self.graph = new.graph;
        self.zoom_factor = new.zoom_factor;
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
    pub is_dragged: bool,
    pub is_selected: bool,
}

impl Default for VisualNode {
    fn default() -> Self {
        Self {
            node: Node::Person("Foo Bar".to_string()),
            id: 0,
            x: 0.0,
            y: 0.0,
            radius: 10.0,
            is_dragged: false,
            is_selected: false,
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
                VisualNode {
                    id: 2,
                    x: 100.0,
                    y: 100.0,
                    radius: 20.0,
                    node: Node::DNSEntry {
                        nameserver: Domain {
                            domain: "ns1.example.com".to_string(),
                        },
                        record: DNSRecord::SRV {
                            service: "SFTP Server".to_string(),
                            protocol: "ftp".to_string(),
                            from: "example.com".to_string(),
                            to: "ftp.example.com".to_string(),
                            to_port: 21,
                        },
                    },
                    ..Default::default()
                },
            ],
            edges: vec![VisualEdge { from: 0, to: 1 }, VisualEdge { from: 1, to: 2 }],
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
            // draw all nodes
            for node in &self.graph.nodes {
                let to_draw = Path::circle(
                    Point::new(node.x, node.y),
                    node.radius * self.zoom_factor * constants::NODE_ZOOM_SCALING,
                );
                frame.fill(&to_draw, Color::BLACK);
            }

            // draw all edges
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
                        width: 1.0 * self.zoom_factor,
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
        // is returned if we dont do anything with the event
        let uncaptured = (canvas::event::Status::Ignored, None);

        // send update messages to the app
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
                    mouse::Button::Middle => match cursor.position() {
                        Some(position) => (
                            canvas::event::Status::Captured,
                            Some(Message::MiddleMouseClick(position)),
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
                    mouse::Button::Middle => {
                        (canvas::event::Status::Captured, Some(Message::MouseRelease))
                    }
                    _ => uncaptured,
                },
                mouse::Event::WheelScrolled { delta } => (
                    canvas::event::Status::Captured,
                    Some(Message::MouseScroll(delta)),
                ),
                _ => uncaptured,
            },
            _ => uncaptured,
        }
    }
}
