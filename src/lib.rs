mod event_handler;
mod state;
mod command;

use pumpkin_plugin_api::{
    Context, Plugin, PluginMetadata, events::EventPriority
};
use tracing::*;
use crate::command::register_command;
use crate::event_handler::chat::ChatHandler;
use crate::event_handler::join::JoinHandler;
use crate::state::EState;

struct EPlugin;

impl Plugin for EPlugin {
    fn new() -> Self {
        EPlugin
    }

    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            name: "e".into(),
            version: env!("CARGO_PKG_VERSION").into(),
            authors: vec!["Laptop59".into()],
            description: "A simple E plugin! EEEEE".into(),
        }
    }

    fn on_load(&mut self, context: Context) -> pumpkin_plugin_api::Result<()> {
        info!("Hello E!");

        context.register_event_handler(ChatHandler, EventPriority::Highest, true)?;
        context.register_event_handler(JoinHandler, EventPriority::Highest, true)?;
        register_command(&context)?;

        EState::load_from_disk(&context);

        Ok(())
    }

    fn on_unload(&mut self, context: Context) -> pumpkin_plugin_api::Result<()> {
        info!("Goodbye E!");

        EState::save_to_disk(&context);

        Ok(())
    }
}

pumpkin_plugin_api::register_plugin!(EPlugin);
