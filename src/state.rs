use std::collections::HashMap;
use std::fs::File;
use std::io::{ErrorKind, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, LazyLock, Mutex, MutexGuard};
use std::time::{Duration, Instant};
use pumpkin_plugin_api::command_wit::Player;
use pumpkin_plugin_api::common::NamedColor;
use pumpkin_plugin_api::Context;
use pumpkin_plugin_api::text::TextComponent;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub static STATE_PATH: &str = "state.json";

pub static STATE: LazyLock<EState> = LazyLock::new(
    || EState {
        balances: Arc::new(Mutex::new(HashMap::new())),
        latest_chat_instants: Arc::new(Mutex::new(HashMap::new()))
    }
);

pub struct Milestone {
    pub amount: i64,
    pub message: &'static str
}

pub static MILESTONES: &[Milestone] = &[
    Milestone {
        amount: 1,
        message: "Your first E! Say more Es to earn more Es! That's an E-conomy Plugin! Use /e to find out your balance!"
    },
    Milestone {
        amount: 10,
        message: "10 Es! Can you continue this stellar streak? Remember, E is just a currency."
    },
    Milestone {
        amount: 25,
        message: "25 Es! Anyhow, make sure to mix different types of E in one message for a higher multiplier! Search E & Experiment E!"
    },
    Milestone {
        amount: 100,
        message: "100 Es! Are you spamming them or are they organic? Just wanted to know, because higher e-density LOWERS e-reward!!! Yeah, don't make your Es too close to each other!"
    },
    Milestone {
        amount: 250,
        message: "250 Es! What do you want me to say next? Oh right, don't spam too quickly!"
    },
    Milestone {
        amount: 500,
        message: "500 Es! Reach a specific milestone, and you'll get something special!"
    },
    Milestone {
        amount: 1_000,
        message: "1,000 Es! Congrats, but this ain't it! You must be tired by now! Hopefully now is a good time to tell you that accented and 'more weirder' Es (like Σ) give more Es! Maybe a helpful tip is better than a milestone reward?"
    },
    Milestone {
        amount: 2_000,
        message: "2,000 Es! Try greek characters and explore more E-like characters!"
    },
    Milestone {
        amount: 5_000,
        message: "5,000 Es! Your reward is...knowing a new symbol ⓔ to use! Oh right...you can't copy it..."
    },
    Milestone {
        amount: 10_000,
        message: "10,000 Es! How are you having fun?!?"
    },
    Milestone {
        amount: 25_000,
        message: "25,000 Es! You don't touch grass at all!"
    },
    Milestone {
        amount: 100_000,
        message: "100,000 Es! What even is an E anymore? This is (not) the last milestone!"
    },
    Milestone {
        amount: 250_000,
        message: "250,000 Es! This ain't it either!"
    },
    Milestone {
        amount: 500_000,
        message: "500,000 Es! Finally! You have completed the second last milestone..? Wait, so this isn't the last one..."
    },
    Milestone {
        amount: 1_000_000,
        message: "1,000,000 Es! The actual last one! Y? YYYYY??? Thanks for wasting your precious time on this stupid ''economy'' plugin..."
    }
];

/// The current state of the E plugin.
pub struct EState {
    balances: Arc<Mutex<HashMap<Uuid, i64>>>,
    latest_chat_instants: Arc<Mutex<HashMap<Uuid, Instant>>>
}

/// The serializable intermediate of the [`EState`]
#[derive(Serialize)]
pub struct SerializableEState<'a> {
    balances: &'a HashMap<Uuid, i64>
}

/// The deserializable intermediate of the [`EState`]
#[derive(Deserialize)]
pub struct DeserializableEState {
    balances: HashMap<Uuid, i64>
}

impl EState {
    pub fn get_e(target: &Uuid) -> i64 {
        *STATE.balances.lock().expect("E").get(target).unwrap_or(&0i64)
    }

    pub fn add_e(target: Uuid, amount: i64) {
        let mut map = STATE.balances.lock().expect("E");
        *map.entry(target).or_insert(0) += amount;
    }

    pub fn remove_e(target: Uuid, amount: i64) {
        let mut map = STATE.balances.lock().expect("E");
        *map.entry(target).or_insert(0) -= amount;
    }

    pub fn set_e(target: Uuid, amount: i64) {
        let mut map = STATE.balances.lock().expect("E");
        *map.entry(target).or_insert(0) = amount;
    }

    pub fn set_latest_chat_instant(target: Uuid) {
        let when = Instant::now();
        let mut map = STATE.latest_chat_instants.lock().expect("E");
        map.insert(target, when);
    }

    pub fn get_latest_chat_ago(target: &Uuid) -> Option<Duration> {
        let map = STATE.latest_chat_instants.lock().expect("E");
        Some(Instant::now() - *map.get(target)?)
    }

    pub fn award_es(player: &Player, es: i64) {
        let uuid = Uuid::parse_str(&player.get_id()).expect("E");

        let old = Self::get_e(&uuid);
        Self::add_e(uuid, es);
        let new = Self::get_e(&uuid);

        for milestone in MILESTONES {
            if old < milestone.amount && new >= milestone.amount {
                let title = TextComponent::text("E");
                title.color_named(NamedColor::Red);
                title.bold(true);

                let subtitle = TextComponent::text("New milestone achieved!");
                title.color_named(NamedColor::Green);

                player.show_title(title);
                player.show_subtitle(subtitle);

                let message = TextComponent::text("");
                let symbol = TextComponent::text("E ");
                symbol.color_named(NamedColor::Red);
                symbol.bold(true);
                message.add_child(symbol);

                let header = TextComponent::text("New milestone unlocked for ");
                header.color_named(NamedColor::Green);
                message.add_child(header);

                let header = TextComponent::text("E");
                header.color_named(NamedColor::Red);
                message.add_child(header);

                let header = TextComponent::text(&milestone.amount.to_string());
                header.color_named(NamedColor::White);
                message.add_child(header);

                let header = TextComponent::text(": ");
                header.color_named(NamedColor::Green);
                message.add_child(header);

                let milestone_text = TextComponent::text(milestone.message);
                milestone_text.color_named(NamedColor::Gold);
                message.add_child(milestone_text);

                player.send_system_message(message, false);
            }
        }

        if es != 0 {
            let actionbar = TextComponent::text("+ E");
            actionbar.color_named(NamedColor::Red);
            let actionbar2 = TextComponent::text(&es.to_string());
            actionbar2.color_named(NamedColor::White);

            actionbar.add_child(actionbar2);

            player.show_actionbar(actionbar);
        }
    }

    pub fn save_to_disk(context: &Context) {
        let mut path = PathBuf::from(context.get_data_folder());
        path.push(STATE_PATH);

        let state = SerializableEState {
            balances: &*STATE.balances.lock().expect("E")
        };

        match serde_json::to_string(&state) {
            Ok(json) => {
                match EState::try_write(&*path, json.as_bytes()) {
                    Ok(()) => tracing::info!("Successfully saved E-state file!"),
                    Err(error) => tracing::error!("Could not create E-state file: {error}")
                }
            },
            Err(error) => tracing::error!("Could not serialize E-state: {error}")
        }
    }

    pub fn load_from_disk(context: &Context) {
        let mut path = PathBuf::from(context.get_data_folder());
        path.push(STATE_PATH);

        match EState::try_read(&*path) {
            Ok(data) => {
                match serde_json::from_str::<DeserializableEState>(&data) {
                    Ok(state) => {
                        *STATE.balances.lock().expect("E") = state.balances;
                        tracing::info!("Successfully loaded E-state file!");
                    }
                    Err(error) => tracing::error!("Could not deserialize E-state: {error}")
                }
            },
            Err(error) => {
                if error.kind() != ErrorKind::NotFound {
                    tracing::error!("Could not read E-state file: {error}")
                }
            }
        }
    }

    fn try_read(path: &Path) -> std::io::Result<String> {
        let mut file = File::open(path)?;
        let mut data: String = String::new();
        file.read_to_string(&mut data)?;
        Ok(data)
    }

    fn try_write(path: &Path, data: &[u8]) -> std::io::Result<()> {
        let mut file = File::create(path)?;
        file.write_all(data)?;
        Ok(())
    }
}