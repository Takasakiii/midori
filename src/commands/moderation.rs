use poise::{
    command,
    serenity_prelude::{Permissions, User},
};

use crate::extensions::{MessageExtension, UserBotExtension, UserExtension};

use super::{CommandResult, Context};

#[command(slash_command, ephemeral)]
/// Deleta varias mensagens de um chat.
pub async fn prune(
    ctx: Context<'_>,
    #[description = "Quantidade de mensagens a serem removidas"]
    #[min = 2]
    #[max = 100]
    amount: u8,
    #[description = "Usuário que enviou a mensagem"] user: Option<User>,
) -> CommandResult {
    if !ctx.check_bot_permissions(Permissions::MANAGE_MESSAGES) {
        ctx.send_error("Eu não tem permissão para deletar mensagens.")
            .await?;
        return Ok(());
    }

    if !ctx.check_user_permissions(Permissions::MANAGE_MESSAGES) {
        ctx.send_error("Você não tem permissão para deletar mensagens.")
            .await?;
        return Ok(());
    }

    let amount = amount as usize;

    let messages = if let Some(user) = user {
        let mut messages = ctx
            .discord()
            .http
            .get_messages(ctx.channel_id().0, "?limit=100")
            .await?
            .into_iter()
            .filter(|e| e.author.id == user.id)
            .collect::<Vec<_>>();

        if messages.len() == amount {
            messages
        } else {
            while messages.len() < amount {
                if let Some(message) = messages.last() {
                    let discord_messages = ctx
                        .discord()
                        .http
                        .get_messages(
                            ctx.channel_id().0,
                            &format!("?limit=100&before={}", message.id.0),
                        )
                        .await?
                        .into_iter()
                        .filter(|m| m.author.id == user.id)
                        .collect::<Vec<_>>();

                    messages.extend(discord_messages);
                }
            }
            messages
        }
    } else {
        ctx.discord()
            .http
            .get_messages(ctx.channel_id().0, &format!("?limit={}", amount))
            .await?
    };

    let messages = messages
        .iter()
        .enumerate()
        .filter(|(index, _)| index < &amount)
        .map(|(_, id)| id.id.0)
        .collect::<Vec<_>>();

    let messages_len = messages.len();

    ctx.discord()
        .http
        .delete_messages(
            ctx.channel_id().0,
            &serde_json::json!({ "messages": messages }),
        )
        .await?;

    ctx.say(format!("Foram removidas {} mensagens.", messages_len))
        .await?;

    Ok(())
}
