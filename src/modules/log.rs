use serenity::client::Context;
use serenity::model::channel::{Message, Reaction};
use serenity::model::id::ChannelId;
use serenity::prelude::EventHandler;

pub struct MessageDeleteLog;

impl EventHandler for MessageDeleteLog {
    fn message_delete(&self, ctx: Context, channel_id: ChannelId, message_id: MessageId) {
        if channel_id != ChannelId(01) {
            return;
        }


        let embed = EmbedBuilder::new()
            .color(0xff0000) 
            .title("Message Deleted")
            .field("Message ID", message_id.to_string(), false)
            .field("Channel ID", channel_id.to_string(), false)
            .field("Author", message_id.author().to_string(), false)
            .field("Timestamp", message_id.timestamp().to_string(), false)
            .timestamp(message_id.timestamp())
            .build();

 
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
        if new_message.channel_id != ChannelId(01) {
            return;
        }

        
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
            .color(0x0000ff) 
            .title("Message Edited")
            .fields(fields)
            .timestamp(new_message.timestamp)
            .build();

        
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
