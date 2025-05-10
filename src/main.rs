use std::fmt::Display;

use discord_rich_presence::activity::{ActivityType, Assets};
use discord_rich_presence::{DiscordIpc, DiscordIpcClient, activity};
use iced::widget::{
    Column, Renderer, Row, button, checkbox, column, container, row, text, text_input,
};
use iced::{Element, Length, Theme};
use iced_aw::{DropDown, drop_down};

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
    ChangeAssets(Option<Asset>),
    ChangeActivityType(u8),
    Select(ActivityTypeChoice),
    AssetToggle(bool),
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
    assets: Option<Asset>,
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
            checkbox("Assets", self.asset_menu).on_toggle(Message::AssetToggle),
            button(text("Start IPC Server")).on_press(Message::Start),
            drop_down,
            text_input(
                "Large Image Asset Name",
                &self
                    .assets
                    .clone()
                    .unwrap_or(Asset {
                        ..Default::default()
                    })
                    .large_image
            ),
            text_input(
                "Small Image Asset Name",
                &self
                    .assets
                    .clone()
                    .unwrap_or(Asset {
                        ..Default::default()
                    })
                    .small_image
            ),
            text_input(
                "Large text",
                &self
                    .assets
                    .clone()
                    .unwrap_or(Asset {
                        ..Default::default()
                    })
                    .large_text
            ),
            text_input(
                "Small text",
                &self
                    .assets
                    .clone()
                    .unwrap_or(Asset {
                        ..Default::default()
                    })
                    .small_text
            )
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
            Message::ChangeAssets(assets) => {
                self.assets = assets;
            }
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
            Message::AssetToggle(toggled) => self.asset_menu = toggled,
            Message::Start => {
                let mut client = DiscordIpcClient::new("1276619507460214804");

                let assets = self.assets.as_mut().unwrap();

                let payload = activity::Activity::new()
                    .state(&self.state)
                    .details(&self.details)
                    .assets(
                        Assets::new()
                            .large_text(assets.large_text.as_mut_str())
                            .large_image(assets.large_image.as_mut_str())
                            .small_text(assets.small_text.as_mut_str())
                            .small_image(assets.small_image.as_mut_str()),
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
