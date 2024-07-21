use iced::executor;
use iced::theme::Theme;
use iced::Settings;
use iced::{Application, Command, Element};
use schnuffel::views::graph::helpers::update_graph;
use schnuffel::views::graph::GraphState;
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

fn main() -> iced::Result {
    App::run(Settings {
        antialiasing: true,
        ..Default::default()
    })
}
