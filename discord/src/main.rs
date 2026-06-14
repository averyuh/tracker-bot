mod tracker;

use std::collections::HashMap;

use log::{info};
use serenity::all::{ChannelId, CreateMessage};
use serenity::model::channel::Message;
use serenity::prelude::*;
use serenity::{all::Ready, async_trait};

use crate::tracker::{Tracker, TrackerConfig};

pub struct Handler {
    // keyed by TrackerConfig::name
    trackers: HashMap<&'static str, Tracker>,
}

impl Handler {
    pub fn new(trackers: impl IntoIterator<Item = Tracker>) -> Self {
        Self {
            trackers: trackers.into_iter().map(|t| (t.name, t)).collect(),
        }
    }
}

// TODO: Poise
#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, event: Ready) {
        info!("Bot User \"{}\" is now ready.", event.user.display_name())
    }

    // TODO: Poise
    async fn message(&self, ctx: Context, msg: Message) {
        if let Some(query) = msg.content.strip_prefix("!search ") {
            let tracker = self.trackers.get("ye").unwrap();

            let results = tracker.search(query);

            if let Err(e) =  tracker.send_embed(&ctx, &msg, query, &results).await {
                eprintln!("Pagination error: {e}");
            }
        }
    }
}

// Add as many as you need — 'static so they can be referenced from spawned tasks
static TRACKERS: &[TrackerConfig] = &[
    TrackerConfig {
        name: "ye",
        url: "https://yetracker.net/htmlview/sheet?headers=false&gid=34972268"
    },
    //TrackerConfig { name: "other", url: "https://othertracker.net/..." },
];

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().unwrap();
    pretty_env_logger::init();
    info!("Tracker bot starting...");

    let mut set = tokio::task::JoinSet::new();
    for config in TRACKERS.iter() {
        set.spawn(Tracker::build(config));
    }

    let mut trackers = Vec::with_capacity(TRACKERS.len());
    while let Some(res) = set.join_next().await {
        trackers.push(res.unwrap()?); // outer ? = JoinError (task panicked), inner ? = your anyhow::Result
    }
    let handler = Handler::new(trackers);

    let token = std::env::var("TOKEN").expect("No 'TOKEN' env var found.");
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(handler)
        .await?;

    client.start().await?;
    Ok(())
}
