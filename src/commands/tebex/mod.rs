use crate::commands::Commands;

pub mod command;
mod etc;
use crate::commands::tebex::command::*;
use crate::ui::{COLOR_ERROR, COLOR_RESET, CROSS};
use tebex_headless_rust::handlers::misc::get_public_api_key;

pub async fn commands() -> Commands {
    let public_api_key = get_public_api_key();
    if let Err(_) = public_api_key {
        println!(
            "\n{}{} Certain Tebex commands fail because the environment variable TEBEX_PUBLIC_KEY is unset. Please set it in your .env file.{}",
            COLOR_ERROR, CROSS, COLOR_RESET
        )
    }

    Commands {
        commands: vec![],
        conditional_commands: vec![Box::new(Store)],
    }
}
