use discord_rich_presence::activity::{ActivityType, Assets, Button};
use discord_rich_presence::{DiscordIpc, DiscordIpcClient, activity};
use iced::widget::{Column, Renderer, Row, button, column, container, row, text, text_input};
use iced::{Element, Length, Theme};
use iced_aw::{DropDown, drop_down};
use std::fmt::Display;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Default)]
struct Asset {
    large_image: String,
    large_text: String,
    small_image: String,
    small_text: String,
}

impl<'a> From<&'a Asset> for Assets<'a> {
    fn from(val: &'a Asset) -> Assets<'a> {
        Assets::new()
            .large_text(&val.large_text)
            .large_image(&val.large_image)
            .small_text(&val.small_text)
            .small_image(&val.small_image)
    }
}

#[derive(Debug, Clone, Default)]
struct ActivityButton {
    label: String,
    url: String,
}

#[derive(Debug, Clone)]
enum Message {
    ChangeState(String),
    ChangeDetails(String),
    ChangeAssets(String, String),
    ChangeButtons(String, String),
    Select(ActivityTypeChoice),
    Dismiss,
    Expand,
    Start,
}

#[derive(Clone, Debug, Default)]
enum ActivityTypeChoice {
    #[default]
    Playing,
    Listening,
    Watching,
    Competing,
}

impl Display for ActivityTypeChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<ActivityTypeChoice> for ActivityType {
    fn from(val: ActivityTypeChoice) -> Self {
        match val {
            ActivityTypeChoice::Playing => ActivityType::Playing,
            ActivityTypeChoice::Listening => ActivityType::Listening,
            ActivityTypeChoice::Watching => ActivityType::Watching,
            ActivityTypeChoice::Competing => ActivityType::Competing,
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            buttons: vec![ActivityButton::default(), ActivityButton::default()],
            state: String::new(),
            details: String::new(),
            assets: Asset::default(),
            selected: ActivityTypeChoice::default(),
            expanded: false,
        }
    }
}

const CHOICES: [ActivityTypeChoice; 4] = [
    ActivityTypeChoice::Playing,
    ActivityTypeChoice::Listening,
    ActivityTypeChoice::Watching,
    ActivityTypeChoice::Competing,
];

struct App {
    state: String,
    details: String,
    assets: Asset,
    buttons: Vec<ActivityButton>,
    selected: ActivityTypeChoice,
    expanded: bool,
}

impl App {
    fn view(&self) -> Element<Message> {
        macro_rules! asset_input {
            ($label:expr, $field:ident) => {
                text_input($label, &self.assets.$field)
                    .on_input(|c| Message::ChangeAssets(stringify!($field).to_string(), c))
            };
        }

        let underlay: Row<'_, Message, _, _> = row![
            text(format!("Selected: {}", self.selected)),
            button(text("expand")).on_press(Message::Expand)
        ];

        let overlay: iced::widget::Column<'_, _, Theme, Renderer> =
            Column::with_children(CHOICES.map(|choice| {
                row![
                    text(choice.to_string()),
                    button(text("choose")).on_press(Message::Select(choice))
                ]
                .into()
            }));

        let drop_down = DropDown::new(underlay, overlay, self.expanded)
            .width(Length::Fill)
            .on_dismiss(Message::Dismiss)
            .alignment(drop_down::Alignment::Bottom);

        let column = column![
            text_input("Title (State)", &self.state).on_input(Message::ChangeState),
            text_input("Details", &self.details).on_input(Message::ChangeDetails),
            drop_down,
            // Assets
            asset_input!("Large Image Asset Name", large_image),
            asset_input!("Small Image Asset Name", small_image),
            asset_input!("Large text", large_text),
            asset_input!("Small text", small_text),
            // Buttons
            text_input("Button 1 Label", &self.buttons.index(0).clone().label)
                .on_input(|c| Message::ChangeButtons("label_one".to_string(), c)),
            // Start RPC
            button(text("Start Rich Presence")).on_press(Message::Start),
        ];

        container(column).center(Length::Fill).into()
    }
    fn update(&mut self, message: Message) {
        match message {
            Message::ChangeState(state) => {
                self.state = state;
            }
            Message::ChangeDetails(details) => {
                self.details = details;
            }
            Message::ChangeAssets(assets, input) => match assets.as_str() {
                "small_text" => self.assets.small_text = input,
                "large_text" => self.assets.large_text = input,
                "small_image" => self.assets.small_image = input,
                "large_image" => self.assets.large_image = input,
                _ => unreachable!(),
            },
            Message::ChangeButtons(button, input) => match button.as_str() {
                "label_one" => self.buttons.index_mut(0).label = input,
                "url_one" => self.buttons.index_mut(0).url = input,
                "label_two" => self.buttons.index_mut(1).label = input,
                "large_image" => self.buttons.index_mut(1).url = input,
                _ => unreachable!(),
            },
            Message::Select(choice) => {
                self.selected = choice;
                self.expanded = false;
            }
            Message::Dismiss => self.expanded = false,
            Message::Expand => self.expanded = !self.expanded,
            Message::Start => {
                let mut client = DiscordIpcClient::new("1276619507460214804")
                    .expect("failed to connect to client ID");

                let payload = activity::Activity::new()
                    .state(&self.state)
                    .details(&self.details)
                    .assets((&self.assets).into())
                    .buttons(
                        self.buttons
                            .iter()
                            .map(|b| Button::new(b.label.as_str(), b.url.as_str()))
                            .collect(),
                    )
                    .activity_type(self.selected.clone().into());

                client
                    .connect()
                    .expect("something went wrong while connecting");
                client
                    .set_activity(payload)
                    .expect("something went wrong while setting activity");
            }
        }
    }
}

fn main() -> iced::Result {
    iced::application("presence", App::update, App::view)
        .theme(|_| Theme::Dark)
        .window_size((400.0, 400.0))
        .run()
}
