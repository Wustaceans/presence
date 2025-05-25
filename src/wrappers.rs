use discord_rich_presence::activity::{ActivityType, Assets};
use std::fmt::Display;

#[derive(Debug, Clone, Default)]
pub struct Asset {
    pub large_image: String,
    pub large_text: String,
    pub small_image: String,
    pub small_text: String,
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
pub struct ActivityButton {
    pub label: String,
    pub url: String,
}

#[derive(Clone, Debug, Default)]
pub enum ActivityTypeChoice {
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
