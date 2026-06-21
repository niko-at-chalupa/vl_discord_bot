use crate::commands::ConditionalCommand;
use crate::types::{Context, Data, Error};
use std::sync::Arc;

pub struct Store;

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

impl ConditionalCommand for Store {
    fn should_register(&self) -> bool {
        true
    }

    fn command(&self) -> poise::Command<Arc<Data>, Error> {
        store()
    }
}