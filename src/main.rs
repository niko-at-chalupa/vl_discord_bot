pub mod commands;
pub mod config;
pub mod types;
pub mod ui;

use clap::Parser;
use poise::serenity_prelude as serenity;

use crate::config::{Args, Config};
use crate::types::Data;

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();

    let args = Args::parse();

    if args.config_gen {
        if let Err(e) = Config::generate_default() {
            eprintln!("Error generating config: {}", e);
            std::process::exit(1);
        }
        return;
    }

    let config = Config::load(args.config).expect("Failed to load config");

    let token = std::env::var("TOKEN").expect(
        "The environment variable TOKEN is unset, please set it in `.env` or by exporting it",
    );
    let intents = serenity::GatewayIntents::non_privileged();

    let commands = crate::commands::all_commands().await;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: commands,
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                println!("Logging in as {}", &ctx.cache.current_user().name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(std::sync::Arc::new(Data { config }))
            })
        })
        .build();

    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await
        .unwrap();

    let shard_manager = client.shard_manager.clone();
    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to listen for Ctrl+C");
        println!("Shutting down...");
        shard_manager.shutdown_all().await;
    });

    client.start().await.unwrap();
}
