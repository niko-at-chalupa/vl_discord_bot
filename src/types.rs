use std::sync::Arc;
use crate::config::Config;

pub struct Data {
    pub config: Config,
} // User data, which is stored and accessible in all command invocations
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Arc<Data>, Error>;
