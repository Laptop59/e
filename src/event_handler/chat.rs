use std::collections::HashSet;
use phf::phf_map;
use pumpkin_plugin_api::events::{EventData, EventHandler, PlayerChatEvent};
use pumpkin_plugin_api::Server;
use pumpkin_plugin_api::text::{RgbColor, TextComponent};
use uuid::Uuid;
use crate::state::EState;

pub const E_REWARDS: phf::Map<char, i64> = phf_map! {
    'e' | 'E' => 1,
    'é' | 'É' | 'è' | 'È' |
    'ê' | 'Ê' | 'ë' | 'Ë' => 3,
    'ē' | 'Ē' | 'ĕ' | 'Ĕ' | 'ė' | 'Ė' => 5,
    'ę' | 'Ę' | 'ě' | 'Ě' => 7,
    'ｅ' | 'Ｅ' => 11, // idk lol
    'е' | 'Е' => 13, // cyrillic
    'Ε' => 15, // epsilon
    'Σ' => 16, // sigma
    'ε' => 17, // ditto
    'ϵ' => 19, // ditto
    'ḕ' | 'ḗ' | 'ḙ' | 'ḛ' | 'ḝ' | 'ẹ' | 'ẻ' | 'ẽ' | 'ế' | 'ề' | 'ể' | 'ễ' | 'ệ' => 23,
    'ἐ' | 'ἑ' | 'ἒ' | 'ἓ' | 'ἔ' | 'ἕ' | 'ὲ' | 'έ' => 27,
    'Ḕ' | 'Ḗ' | 'Ḙ' | 'Ḛ' | 'Ḝ' | 'Ẹ' | 'Ẻ' | 'Ẽ' | 'Ế' | 'Ề' | 'Ể' | 'Ễ' | 'Ệ' | 'Ȅ' | 'ȅ' => 33,
    'Ὲ' | 'Έ' | 'Ἐ' | 'Ἑ' | 'Ἒ' | 'Ἓ' | 'Ἔ' | 'Ἕ' => 37,
    'ⓔ' | 'Ⓔ' => 43,
    '⒠' | '🄔' => 47,
    'ℯ' => 51,
    '∃' => 57,
    '∊' => 63,
    '€' => 69,
    'ℰ' => 73,
    'ℇ' | 'Є' | 'Э' | 'э' | 'є' => 77,
    'ҽ' => 81,
    '𑢦' | 'ͤ' | 'ₑ' => 85,
    'ɇ' | 'Ɇ' | 'ӭ' | 'Ӭ' => 87,
    '𐌄' | '𜳚' | '🄴' => 91,
    '𖹏' | '𞓤' => 94,
    '🅔' | '🅴' => 97,
    '🇪' => 101,
    'Ʃ' => 105, // not sigma
    'Ƹ' | 'ƹ' => 111
};

enum MessagePart<'a> {
    Text(&'a str),
    E(char, i64)
}

pub struct ChatHandler;
impl EventHandler<PlayerChatEvent> for ChatHandler {
    fn handle(
        &self,
        server: Server,
        mut event: EventData<PlayerChatEvent>,
    ) -> EventData<PlayerChatEvent> {
        // We must cancel the event and use our own handler!
        event.cancelled = true;

        let sender = &event.player;

        let message = &event.message;
        let mut es_earned = 0;

        let uuid = Uuid::parse_str(&sender.get_id()).expect("E");
        let message_parts: Vec<MessagePart> = split_by_e_chars(message, &mut es_earned, uuid);

        for player in server.get_all_players() {
            // Is there a way to clone text components?
            // I don't think there is a way.

            let chat_message = TextComponent::text("");
            chat_message.add_text("<");
            chat_message.add_child(sender.get_display_name());
            chat_message.add_text("> ");
            for message_part in &message_parts {
                match message_part {
                    MessagePart::Text(text) => chat_message.add_text(text),
                    MessagePart::E(c, reward) => {
                        let mut buf = [0u8; 4];
                        let e = TextComponent::text(c.encode_utf8(&mut buf));
                        e.color_rgb(get_e_color(*reward));
                        chat_message.add_child(e);
                    }
                }
            }

            player.send_system_message(chat_message, false);
        }

        EState::award_es(&sender, es_earned);

        event
    }
}

fn split_by_e_chars<'a>(message: &'a str, es_earned: &mut i64, uuid: Uuid) -> Vec<MessagePart<'a>> {
    let mut last: usize = 0;
    let mut result = Vec::new();

    let mut used_es: HashSet<char> = HashSet::new();
    let mut multiplier: f64 = 0.0;
    let mut e_density: i64 = 0;

    for (i, c) in message.char_indices() {
        let reward = get_e_reward(c);
        if reward > 0 { // Why are we accounting for negatives lol
            e_density += reward;

            if last < i {
                result.push(MessagePart::Text(&message[last..i]));
            }
            result.push(MessagePart::E(c, reward));

            *es_earned += reward;
            // e-density hinders the multiplier
            multiplier *= 0.9977f64.powf(e_density as f64);
            if used_es.insert(c) {
                // e-density hinders the multiplier increases!
                multiplier += 1.0 + multiplier * 0.07 * 0.9965f64.powf(e_density as f64);
            }
            last = i + c.len_utf8();
        }

        e_density = 0.max(e_density - 10); // make low values of e-rewards not increase e-density.
        e_density /= 2; // e-density diminishes quickly
    }

    if last < message.len() {
        result.push(MessagePart::Text(&message[last..]));
    }

    // Multiplier decreases as latest chat ago decreases.
    if let Some(ago) = EState::get_latest_chat_ago(&uuid) {
        let seconds = ago.as_secs_f64();
        if seconds < 1.0 {
            multiplier *= seconds.powi(3);
        }
    }

    EState::set_latest_chat_instant(uuid);

    // Multiplication bonus: unique Es!
    let es_multiplied = *es_earned as f64 * multiplier;
    *es_earned = es_multiplied.ceil() as i64;

    result
}

fn get_e_reward(c: char) -> i64 {
    *E_REWARDS.get(&c).unwrap_or(&0)
}

fn get_e_color(reward: i64) -> RgbColor {
    let reward = reward as f64;
    let mut modifier = reward.log(1.04);
    if modifier.is_nan() {
        modifier = 0.0;
    } else if modifier.is_infinite() {
        modifier = 1000.0;
    }
    if modifier < 0.0 {
        modifier = 0.0;
    }
    let modifier = modifier as u8;
    let v = 0xCCu8.checked_sub(modifier).unwrap_or(0x00);

    RgbColor { r: 0xFF, g: v, b: v }
}