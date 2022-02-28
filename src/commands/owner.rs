use poise::{builtins, command};

use super::{CommandResult, Context};

#[command(prefix_command, hide_in_help)]
pub async fn register_commands(ctx: Context<'_>, #[flag] global: bool) -> CommandResult {
    let owner = ctx.data().config.owner_id;
    let author = ctx.author().id.0;

    if author == owner {
        builtins::register_application_commands(ctx, global).await?;
    } else if let Context::Prefix(p_ctx) = ctx {
        p_ctx
            .msg
            .reply(
                ctx.discord(),
                "❌ **|** Você não é dono do bot para realizar essa ação.",
            )
            .await?;
    }

    Ok(())
}
