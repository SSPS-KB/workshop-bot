use serenity::client::Context;
use serenity::model::channel::{Message, Reaction};
use serenity::model::id::ChannelId;
use serenity::prelude::EventHandler;

pub struct MessageEditLog;

impl EventHandler for MessageEditLog {
    fn message_update(&self, ctx: Context, _old_message: Option<Message>, new_message: Message, _event_type: MessageUpdateEvent) {
        if new_message.channel_id != ChannelId(1093637714420711535) {
            return;
        }

        let embed = create_embed(_old_message, new_message);
        
        if let Err(why) = ChannelId(1093637714420711535).send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.clone_from(&embed);
                e
            })
        }) {
            println!("Error sending message: {:?}", why);
        }
    }
}

fn create_embed(old_message: Option<Message>, new_message: Message) -> Embed {
    let timestamp = new_message.timestamp;
    let mut fields = Vec::new();
    if let Some(old) = old_message {
        if old.content != new_message.content {
            fields.push(("Old message", old.content.clone(), false));
            fields.push(("New message", new_message.content.clone(), false));
        }
    }

    fields.push(("Message ID", format!("{}", new_message.id), false));
    fields.push(("Channel ID", format!("{}", new_message.channel_id), false));
    fields.push(("Author", format!("{}", new_message.author), false));
    fields.push(("Timestamp", format!("{}", timestamp), false));

    EmbedBuilder::new()
        .color(0x0000ff)
        .title("Message Edited")
        .fields(fields)
        .timestamp(timestamp)
        .build()
        .unwrap()
}
