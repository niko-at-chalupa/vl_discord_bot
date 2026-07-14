use crate::commands::Commands;
use crate::types::Context;
use crate::types::Error;

pub async fn commands() -> Commands {
    Commands {
        commands: vec![ping()],
        conditional_commands: vec![],
    }
}

/// Ping the bot, to check if it's running okay.
#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let start = std::time::Instant::now();

    ctx.defer_ephemeral().await?;

    let duration = start.elapsed();
    let response = ctx
        .data()
        .config
        .messages
        .ping
        .response_message
        .replace("[latency]", &format!("`{:?}ms`", duration.as_millis()));

    ctx.send(
        poise::CreateReply::default()
            .ephemeral(true)
            .content(response),
    )
    .await?;

    Ok(())
}
