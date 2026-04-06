use pumpkin_plugin_api::{
    Context, Plugin, PluginMetadata, Server, common::NamedColor, events::{EventData, EventHandler, EventPriority, PlayerChatEvent, PlayerJoinEvent}, text::TextComponent
};
use tracing::*;

struct ChatHandler;
impl EventHandler<PlayerChatEvent> for ChatHandler {
    fn handle<'a>(
        &'a self,
        _server: Server,
        mut event: EventData<PlayerChatEvent>,
    ) -> EventData<PlayerChatEvent> { 
        unsafe {
            let vec = event.message.as_mut_vec();
            // SAFETY: b'E' is valid ASCII, so the resulting string will be valid UTF-8
            vec.fill(b'E');
        }
        event
    }
}

struct JoinHandler;
impl EventHandler<PlayerJoinEvent> for JoinHandler {
    fn handle<'a>(
        &'a self,
        _server: Server,
        event: EventData<PlayerJoinEvent>,
    ) -> EventData<PlayerJoinEvent> { 
        let component = TextComponent::text("E");
        component.color_named(NamedColor::Gold);
        component.bold(true);

        event.player.show_title(component).expect("E");
        event.player.send_title_animation(0, 1000000000, 10).expect("E");
        event
    }
}

struct EPlugin;
impl Plugin for EPlugin {
    fn new() -> Self {
        EPlugin
    }

    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            name: "E".into(),
            version: env!("CARGO_PKG_VERSION").into(),
            authors: vec!["Laptop59".into()],
            description: "A simple E plugin! EEEEE".into(),
        }
    }

    fn on_load(&mut self, context: Context) -> pumpkin_plugin_api::Result<()> {
        info!("Hello E!");
        context.register_event_handler(ChatHandler, EventPriority::Highest, true)?;
        context.register_event_handler(JoinHandler, EventPriority::Highest, true)?;
        Ok(())
    }

    fn on_unload(&mut self, _context: Context) -> pumpkin_plugin_api::Result<()> {
        info!("Goodbye E!");
        Ok(())
    }
}

pumpkin_plugin_api::register_plugin!(EPlugin);
