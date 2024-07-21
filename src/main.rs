use iced::executor;
use iced::mouse::ScrollDelta;
use iced::theme::Theme;
use iced::Settings;
use iced::{Application, Command, Element};
use schnuffel::views::graph::{GraphState, GraphStateUpdate, MAX_ZOOM, MIN_ZOOM, ZOOM_MULTIPLIER};
use schnuffel::views::ViewState;
use schnuffel::Message;

// all of our views
enum View {
    Graph(GraphState),
}

// the actual app
struct App {
    view: View,
}

// our app in the graph view
impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            App {
                view: View::Graph(GraphState::default()),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("schnuffel")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match &mut self.view {
            View::Graph(state) => {
                update_graph(state, message);
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        match &self.view {
            View::Graph(state) => schnuffel::views::graph::view(state),
        }
    }

    fn theme(&self) -> Self::Theme {
        Theme::Light
    }
}

fn update_graph(state: &mut GraphState, message: Message) {
    match message {
        Message::MouseClick(position) => {
            for node in &mut state.graph.nodes {
                // clear selections
                node.is_selected = false;

                if (position.x - node.x).powf(2.0) + (position.y - node.y).powf(2.0)
                    < (node.radius * state.zoom_factor).powf(2.0)
                {
                    node.is_dragged = true;
                    node.is_selected = true;
                }
            }
        }
        Message::MouseDrag(position) => {
            for node in &mut state.graph.nodes {
                if node.is_dragged {
                    node.x = position.x;
                    node.y = position.y;
                }
            }
        }
        Message::MouseRelease => {
            for node in &mut state.graph.nodes {
                node.is_dragged = false;
            }
        }
        Message::MouseScroll(delta) => {
            if let ScrollDelta::Lines { x: _, y } = delta {
                state.zoom_factor += y * ZOOM_MULTIPLIER;
                state.zoom_factor = state.zoom_factor.clamp(MIN_ZOOM, MAX_ZOOM);

                for node in &mut state.graph.nodes {
                    // update the node positions to reflect the zoom
                    if y < 0.0 && state.zoom_factor != MIN_ZOOM {
                        // zoom out
                        node.x += state.zoom_factor * 10.0;
                        node.y += state.zoom_factor * 10.0;
                    } else if y > 0.0 && state.zoom_factor != MAX_ZOOM {
                        // zoom in
                        node.x -= state.zoom_factor * 10.0;
                        node.y -= state.zoom_factor * 10.0;
                    }
                }
            }
        }
    };
    state.update_state(GraphStateUpdate {
        graph: state.graph.clone(),
        zoom_factor: state.zoom_factor,
    });
}

fn main() -> iced::Result {
    App::run(Settings {
        antialiasing: true,
        ..Default::default()
    })
}
