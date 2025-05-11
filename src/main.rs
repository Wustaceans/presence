use discord_rich_presence::activity::{ActivityType, Assets};
use discord_rich_presence::{DiscordIpc, DiscordIpcClient, activity};
use iced::widget::{
    Column, Renderer, Row, button, checkbox, column, container, row, text, text_input,
};
use iced::{Element, Length, Theme};
use iced_aw::{DropDown, drop_down};
use std::fmt::Display;

type AppResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Clone, Default)]
struct Asset {
    large_image: String,
    large_text: String,
    small_image: String,
    small_text: String,
}

#[derive(Debug, Clone)]
enum Message {
    ChangeState(String),
    ChangeDetails(String),
    ChangeAssets(String, String),
    ChangeActivityType(u8),
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
impl Into<ActivityType> for ActivityTypeChoice {
    fn into(self) -> ActivityType {
        match self {
            ActivityTypeChoice::Playing => ActivityType::Playing,
            ActivityTypeChoice::Listening => ActivityType::Listening,
            ActivityTypeChoice::Watching => ActivityType::Watching,
            ActivityTypeChoice::Competing => ActivityType::Competing,
        }
    }
}

const CHOICES: [ActivityTypeChoice; 4] = [
    ActivityTypeChoice::Playing,
    ActivityTypeChoice::Listening,
    ActivityTypeChoice::Watching,
    ActivityTypeChoice::Competing,
];

#[derive(Default)]
struct App {
    state: String,
    details: String,
    assets: Asset,
    asset_menu: bool,
    activity_type: ActivityTypeChoice,
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
            // open alternate asset menu
            button(text("Start IPC Server")).on_press(Message::Start),
            drop_down,
            text_input("Large Image Asset Name", &self.assets.clone().large_image)
                .on_input(|c| Message::ChangeAssets("large_image".to_string(), c)),
            text_input("Small Image Asset Name", &self.assets.clone().small_image)
                .on_input(|c| Message::ChangeAssets("small_image".to_string(), c)),
            text_input("Large text", &self.assets.clone().large_text)
                .on_input(|c| Message::ChangeAssets("large_text".to_string(), c)),
            text_input("Small text", &self.assets.clone().small_text)
                .on_input(|c| Message::ChangeAssets("small_text".to_string(), c))
        ];

        container(column).into()
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
            Message::ChangeActivityType(activity_type) => match activity_type {
                0 => self.activity_type = ActivityTypeChoice::Playing,
                2 => self.activity_type = ActivityTypeChoice::Listening,
                3 => self.activity_type = ActivityTypeChoice::Watching,
                5 => self.activity_type = ActivityTypeChoice::Competing,
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
        .run()?;

    // kill the presence when program is closed
    std::thread::park();

    Ok(())
}
