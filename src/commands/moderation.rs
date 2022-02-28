use poise::{command, serenity_prelude::User};

use super::{CommandResult, Context};

#[command(
    slash_command,
    ephemeral,
    required_permissions = "MANAGE_MESSAGES",
    required_bot_permissions = "MANAGE_MESSAGES"
)]
/// Deleta varias mensagens de um chat.
pub async fn prune(
    ctx: Context<'_>,
    #[description = "Quantidade de mensagens a serem removidas"]
    #[min = 2]
    #[max = 100]
    amount: u8,
    #[description = "Usuário que enviou a mensagem"] user: Option<User>,
) -> CommandResult {
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
