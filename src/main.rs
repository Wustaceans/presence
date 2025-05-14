use discord_rich_presence::activity::{ActivityType, Assets, Button as Temp};
use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use iced::widget::{button, column, container, row, text, text_input, Column, Renderer, Row};
use iced::{Element, Length, Theme};
use iced_aw::{drop_down, DropDown};
use std::fmt::Display;
use std::ops::{Index, IndexMut};

type AppResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Clone, Default)]
struct Asset {
    large_image: String,
    large_text: String,
    small_image: String,
    small_text: String,
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
            text_input("Large Image Asset Name", &self.assets.clone().large_image)
                .on_input(|c| Message::ChangeAssets("large_image".to_string(), c)),
            text_input("Small Image Asset Name", &self.assets.clone().small_image)
                .on_input(|c| Message::ChangeAssets("small_image".to_string(), c)),
            text_input("Large text", &self.assets.clone().large_text)
                .on_input(|c| Message::ChangeAssets("large_text".to_string(), c)),
            text_input("Small text", &self.assets.clone().small_text)
                .on_input(|c| Message::ChangeAssets("small_text".to_string(), c)),
            // buttons
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
                "small_text" => self.assets.small_text = input.to_string(),
                "large_text" => self.assets.large_text = input.to_string(),
                "small_image" => self.assets.small_image = input.to_string(),
                "large_image" => self.assets.large_image = input.to_string(),
                _ => unreachable!(),
            },
            Message::ChangeButtons(button, input) => match button {
                val if val == "label_one".to_string() => {
                    self.buttons.index_mut(0).label = input.to_string()
                }
                val if val == "url_one".to_string() => {
                    self.buttons.index_mut(0).url = input.to_string()
                }
                val if val == "label_two".to_string() => {
                    self.buttons.index_mut(1).label = input.to_string()
                }
                val if val == "large_image".to_string() => {
                    self.buttons.index_mut(1).url = input.to_string()
                }
                _ => unreachable!(),
            },
            Message::Select(choice) => {
                self.selected = choice;
                self.expanded = false;
            }
            Message::Dismiss => self.expanded = false,
            Message::Expand => self.expanded = !self.expanded,
            Message::Start => {
                let mut client = DiscordIpcClient::new("1276619507460214804");

                let payload = activity::Activity::new()
                    .state(&self.state)
                    .details(&self.details)
                    .assets(
                        Assets::new()
                            .large_text(self.assets.large_text.as_mut_str())
                            .large_image(self.assets.large_image.as_mut_str())
                            .small_text(self.assets.small_text.as_mut_str())
                            .small_image(self.assets.small_image.as_mut_str()),
                    )
                    .buttons(
                        self.buttons
                            .iter()
                            .map(|b| Temp::new(b.label.as_str(), b.url.as_str()))
                            .collect(),
                    )
                    .activity_type(self.selected.clone().into());
                let _ = client.as_mut().expect("").connect();
                let _ = client.as_mut().expect("").set_activity(payload);
            }
        }
    }
}

fn main() -> AppResult<()> {
    iced::application("presence", App::update, App::view)
        .theme(|_| Theme::Dark)
        .window_size((400.0, 400.0))
        .run()?;

    // kill the presence when program is closed
    std::thread::park();

    Ok(())
}
