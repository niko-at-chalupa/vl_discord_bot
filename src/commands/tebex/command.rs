use crate::commands::ConditionalCommand;
use crate::types::{Context, Data, Error};
use std::sync::Arc;
use qrcode::QrCode;
use image::Luma;
use std::io::Cursor;
use poise::serenity_prelude::CreateEmbed;
use tebex_headless_rust::handlers::misc::get_public_api_key;
use tebex_headless_rust::handlers::webstores::get_webstore;
use poise::serenity_prelude as serenity;

pub struct Store;

/// Show the server's webstore URL & info
#[poise::command(slash_command)]
pub async fn store(ctx: Context<'_>) -> Result<(), Error> {    
    ctx.defer_ephemeral().await?;

    let webstore = get_webstore().await?;

    let name = webstore.name;
    let currency = webstore.currency;
    let domain = webstore.webstore_url;
    //let icon = webstore.logo;
    // let's use it later

    let qr_code = QrCode::new(&domain)?;
    let img = qr_code.render::<Luma<u8>>().module_dimensions(8, 8).build();

    let mut buf = Cursor::new(Vec::new());
    img.write_to(&mut buf, image::ImageFormat::Png)?;

    let attachment = serenity::CreateAttachment::bytes(buf.into_inner(), "qr.png");

    let embed = CreateEmbed::new()
        .title(name)
        .url(domain)
        .description(format!(r#"Currency: **{currency}**"#))
        .thumbnail("attachment://qr.png");

    ctx.send(poise::CreateReply::default()
        .ephemeral(true)
        .embed(embed)
        .attachment(attachment)
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