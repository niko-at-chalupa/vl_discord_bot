use crate::types::Error;
use crate::types::Data;
use std::sync::Arc;
use std::collections::HashMap;
use crate::ui;

pub mod core;

pub trait ConditionalCommand {
    fn command(&self) -> poise::Command<Arc<Data>, Error>;
    
    fn should_register(&self) -> bool {
        true
    }
}

pub struct Commands {
    pub commands: Vec<poise::Command<Arc<Data>, Error>>,
    pub conditional_commands: Vec<Box<dyn ConditionalCommand>>,
}

pub async fn all_commands() -> Vec<poise::Command<Arc<Data>, Error>> {
    println!("Registering commands...\n");

    let mut command_modules: HashMap<&'static str, Commands> = HashMap::new();
    let mut final_commands: Vec<poise::Command<Arc<Data>, Error>> = vec![];

    command_modules.insert("core", core::commands().await);

    for module in command_modules {
        println!("[{}]", module.0);

        for safe_command in module.1.commands.into_iter() {
            println!("{}{} {} - {}", ui::COLOR_SUCCESS, ui::CHECK, safe_command.name, safe_command.description.as_deref().unwrap_or("[no description]"));
            final_commands.push(safe_command);
        }
        for conditional_command in module.1.conditional_commands.into_iter() {
            let command = conditional_command.command();
            if conditional_command.should_register() {
                println!("{}{} {} - {}", ui::COLOR_SUCCESS, ui::CHECK, command.name, command.description.as_deref().unwrap_or("[no description]"));
                final_commands.push(command);
            } else {
                println!("{}{} {} - {}", ui::COLOR_ERROR, ui::CROSS, command.name, command.description.as_deref().unwrap_or("[no description]"));
            }
        }
    }
    print!("{}", ui::COLOR_RESET);
    final_commands
}