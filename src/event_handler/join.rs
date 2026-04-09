use pumpkin_plugin_api::common::NamedColor;
use pumpkin_plugin_api::events::{EventData, EventHandler, PlayerJoinEvent};
use pumpkin_plugin_api::Server;
use pumpkin_plugin_api::text::TextComponent;

pub struct JoinHandler;
impl EventHandler<PlayerJoinEvent> for JoinHandler {
    fn handle(
        &self,
        _server: Server,
        event: EventData<PlayerJoinEvent>,
    ) -> EventData<PlayerJoinEvent> {
        let title = TextComponent::text("E");
        title.color_named(NamedColor::Red);
        title.bold(true);

        let subtitle = TextComponent::text("Welcome! Try chatting!");
        title.color_named(NamedColor::Yellow);

        event.player.show_title(title);
        event.player.show_subtitle(subtitle);
        event.player.send_title_animation(0, 40, 20);
        event
    }
}