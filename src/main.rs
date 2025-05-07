use discord_rich_presence::activity::{ActivityType, Assets};
use discord_rich_presence::{DiscordIpc, DiscordIpcClient, activity};
use iced::widget::{column, container};
use iced::{Element, Theme};

#[derive(Debug, Clone)]
enum Message {
    ChangeState(String),
    ChangeDetails(String),
    // ChangeAssets(Assets),
    // ChangeActivityType(ActivityType),
}

#[derive(Default)]
struct App {}

impl App {
    fn view(&self) -> Element<Message> {
        container(column![]).into()
    }

    fn update(&mut self, message: Message) {}
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = DiscordIpcClient::new("1276619507460214804")?;
    client.connect()?;

    let payload = activity::Activity::new()
        .state("I love my wife")
        .details("All hail Ferris")
        .assets(
            Assets::new()
                .large_text("I love my wife")
                .large_image("cat")
                .small_text("meow")
                .small_image("cat"),
        );
    // client.set_activity(payload)?;
    // build a program with the given settings and then autostart it on system boot if user desires

    iced::application("presence", App::update, App::view)
        .theme(|_| Theme::Dark)
        .run()?;

    // kill the presence when program is closed
    std::thread::park();

    Ok(())
}
