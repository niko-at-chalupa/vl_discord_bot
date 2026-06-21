use crate::commands::Commands;

pub mod command;
use crate::commands::tebex::command::*;

pub async fn commands() -> Commands {
    Commands {
        commands: vec![],
        conditional_commands: vec![
            Box::new(Store),
        ]
    }
}