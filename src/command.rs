use pumpkin_plugin_api::command::{Command, CommandError};
use pumpkin_plugin_api::commands::CommandHandler;
use pumpkin_plugin_api::common::NamedColor;
use pumpkin_plugin_api::Context;
use pumpkin_plugin_api::permission::{Permission, PermissionDefault};
use pumpkin_plugin_api::text::TextComponent;
use uuid::Uuid;
use crate::state::EState;

fn init_command_tree() -> Command {
    let names = ["e".to_string()];
    let description = "The command for the E plugin's E-conomy!";

    Command::new(&names, description).execute(ECommandExecutor)
}

pub fn register_command(context: &Context) -> pumpkin_plugin_api::Result<()> {
    context.register_permission(&Permission {
        node: "e:e".to_string(),
        description: "The permission required for the command of the E plugin's E-conomy!".to_string(),
        default: PermissionDefault::Allow,
        children: Vec::new()
    })?;

    context.register_command(init_command_tree(), "e:e");

    Ok(())
}

struct ECommandExecutor;

impl CommandHandler for ECommandExecutor {
    fn handle(
        &self,
        sender: pumpkin_plugin_api::command::CommandSender,
        _server: pumpkin_plugin_api::Server,
        _args: pumpkin_plugin_api::command::ConsumedArgs,
    ) -> pumpkin_plugin_api::Result<i32, CommandError> {
        let Some(player) = sender.as_player() else {
            let error = TextComponent::translate("permissions.requires.player", vec![]);
            return Err(CommandError::CommandFailed(error));
        };

        let uuid = Uuid::parse_str(&player.get_id()).expect("E");
        let amount = EState::get_e(&uuid);

        let message = TextComponent::text("");

        let symbol = TextComponent::text("E ");
        symbol.color_named(NamedColor::Red);
        symbol.bold(true);

        message.add_child(symbol);

        let text = TextComponent::text("You currently have ");
        text.color_named(NamedColor::Yellow);
        message.add_child(text);

        let text = TextComponent::text("E");
        text.color_named(NamedColor::Red);
        message.add_child(text);

        let text = TextComponent::text(&amount.to_string());
        text.color_named(NamedColor::White);
        message.add_child(text);

        let text = TextComponent::text(" in your balance!");
        text.color_named(NamedColor::Yellow);
        message.add_child(text);

        player.send_system_message(message, false);

        Ok(amount as i32)
    }
}