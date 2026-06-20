use crate::commands::Commands;
use crate::types::Context;
use crate::types::Error;

pub async fn commands() -> Commands {
    Commands {
        commands: vec![
            store(),
        ],
        conditional_commands: vec![]
    }
}

/// Show the server's webstore URL & info
#[poise::command(slash_command)]
pub async fn store(ctx: Context<'_>) -> Result<(), Error> {    
    ctx.defer_ephemeral().await?;

    ctx.send(poise::CreateReply::default()
        .ephemeral(true)
        .content("todo"),
    ).await?;
    
    Ok(())
}