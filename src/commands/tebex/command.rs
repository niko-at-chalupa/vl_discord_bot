use crate::commands::ConditionalCommand;
use crate::types::{Context, Data, Error};
use std::sync::Arc;

use tebex_headless_rust::handlers::misc::get_public_api_key;

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
        let public_api_key = get_public_api_key();
        match public_api_key {
            Ok(_) => { return true; },
            Err(_) => { return false; }
        }
    }

    fn command(&self) -> poise::Command<Arc<Data>, Error> {
        store()
    }
}