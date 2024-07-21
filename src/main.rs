use iced::executor;
use iced::theme::Theme;
use iced::widget::canvas::Canvas;
use iced::Settings;
use iced::{Application, Command, Element, Length};
use schnuffel::views::graph::GraphState;
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
            View::Graph(state) => Canvas::new(state)
                .width(Length::Fill)
                .height(Length::Fill)
                .into(),
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
                if (position.x - node.x).powf(2.0) + (position.y - node.y).powf(2.0)
                    < node.radius.powf(2.0)
                {
                    node.is_clicked = true;
                }
            }
            state.update_state(state.graph.clone());
        }
        Message::MouseDrag(position) => {
            for node in &mut state.graph.nodes {
                if node.is_clicked {
                    node.x = position.x;
                    node.y = position.y;
                }
            }
            state.update_state(state.graph.clone());
        }
        Message::MouseRelease => {
            for node in &mut state.graph.nodes {
                node.is_clicked = false;
            }
            state.update_state(state.graph.clone());
        }
    };
}

fn main() -> iced::Result {
    App::run(Settings {
        antialiasing: true,
        ..Default::default()
    })
}
