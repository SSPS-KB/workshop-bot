use serenity::client::Context;
use serenity::model::channel::{Message, Reaction};
use serenity::model::id::ChannelId;
use serenity::prelude::EventHandler;

pub struct MessageDeleteLog;

impl EventHandler for MessageDeleteLog {
    fn message_delete(&self, ctx: Context, channel_id: ChannelId, message_id: MessageId) {
        if channel_id != ChannelId(1093637714420711535) {
            return;
        }

        let embed = create_embed(message_id);
        
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

fn create_embed(message_id: MessageId) -> Embed {
    let timestamp = message_id.created_at();
    let fields = vec![
        ("Message ID", format!("{}", message_id), false),
        ("Channel ID", format!("{}", message_id.channel_id()), false),
        ("Author", format!("{}", message_id.author()), false),
        ("Timestamp", format!("{}", timestamp), false),
    ];

    EmbedBuilder::new()
        .color(0x00ff00) // green color for the embed
        .title("Message Deleted")
        .fields(fields)
        .timestamp(timestamp)
        .build()
        .unwrap()
}
