use i18n::t;
use serenity::client::Context;
use serenity::model::id::RoleId;
use serenity::model::prelude::interaction::message_component::MessageComponentInteraction;
use serenity::model::prelude::interaction::InteractionResponseType;
use tracing::{error, info};

fn get_response_message(added: bool, locale: String) -> &'static str {
    match locale.as_str() {
        "cs" => match added {
            true => t!("modules.reaction_roles.added", "cs"),
            false => t!("modules.reaction_roles.removed", "cs"),
        },
        _ => match added {
            true => t!("modules.reaction_roles.added"),
            false => t!("modules.reaction_roles.removed"),
        },
    }
}

async fn respond(ctx: &Context, component: MessageComponentInteraction, added: bool) {
    let locale = component.locale.clone();
    if let Err(e) = component
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| {
                    message
                        .content(get_response_message(added, locale))
                        .ephemeral(true)
                })
        })
        .await
    {
        error!("Could not respond to reaction role interaction {e}");
    }
}

pub(crate) async fn run(ctx: &Context, component: MessageComponentInteraction) {
    let id = match component
        .data
        .custom_id
        .chars()
        .skip(14)
        .collect::<String>()
        .parse::<u64>()
    {
        Ok(id) => id,
        Err(e) => return error!("There was an error while parsing reaction role id: {e}"),
    };

    let mut member = match component.member.clone() {
        Some(m) => m,
        None => return,
    };

    let role_id = RoleId::from(id);

    if !member.roles.contains(&role_id) {
        member
            .add_role(&ctx.http, role_id)
            .await
            .expect("Couldn't add a reaction role");
        info!("Added reaction {role_id} to {}", member.user.name);
        respond(ctx, component, true).await;
    } else {
        member
            .remove_role(&ctx.http, role_id)
            .await
            .expect("Couldn't add a reaction role");
        info!("Removed reaction {role_id} from {}", member.user.name);
        respond(ctx, component, false).await;
    }
}
