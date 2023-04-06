use serenity::client::Context;
use serenity::model::channel::{Message, Reaction};
use serenity::model::id::ChannelId;
use serenity::prelude::EventHandler;

pub struct MessageDeleteLog;

impl EventHandler for MessageDeleteLog {
    fn message_delete(&self, ctx: Context, channel_id: ChannelId, message_id: MessageId) {
        // check if the message is from the desired channel ID
        if channel_id != ChannelId(01) {
            return;
        }

        // create an embed for the message deletion log
        let embed = EmbedBuilder::new()
            .color(0xff0000) // red color for the embed
            .title("Message Deleted")
            .field("Message ID", message_id.to_string(), false)
            .field("Channel ID", channel_id.to_string(), false)
            .field("Author", message_id.author().to_string(), false)
            .field("Timestamp", message_id.timestamp().to_string(), false)
            .timestamp(message_id.timestamp())
            .build();

        // send the embed to the desired channel ID
        if let Err(why) = ChannelId(01).send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.clone_from(embed.as_ref().unwrap());
                e
            })
        }) {
            println!("Error sending message: {:?}", why);
        }
    }
}

pub struct MessageEditLog;

impl EventHandler for MessageEditLog {
    fn message_update(&self, ctx: Context, old_message: Option<Message>, new_message: Message, _event_type: MessageUpdateEvent) {
        // check if the message is from the desired channel ID
        if new_message.channel_id != ChannelId(01) {
            return;
        }

        // create an embed for the message edit log
        let mut fields = vec![
            ("Message ID", new_message.id.to_string(), false),
            ("Channel ID", new_message.channel_id.to_string(), false),
            ("Author", new_message.author.to_string(), false),
            ("Timestamp", new_message.timestamp.to_string(), false),
        ];

        if let Some(old) = old_message {
            if old.content != new_message.content {
                fields.push(("Old message", old.content.clone(), false));
                fields.push(("New message", new_message.content.clone(), false));
            }
        }

        let embed = EmbedBuilder::new()
            .color(0x0000ff) // blue color for the embed
            .title("Message Edited")
            .fields(fields)
            .timestamp(new_message.timestamp)
            .build();

        // send the embed to the desired channel ID
        if let Err(why) = ChannelId(01).send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.clone_from(embed.as_ref().unwrap());
                e
            })
        }) {
            println!("Error sending message: {:?}", why);
        }
    }
}
