use crate::commands::ConditionalCommand;
use crate::types::{Context, Data, Error};
use image::Luma;
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::CreateEmbed;
use qrcode::QrCode;
use rand::seq::SliceRandom;
use std::io::Cursor;
use std::sync::Arc;
use tebex_headless_rust::handlers::misc::get_public_api_key;
use tebex_headless_rust::handlers::package::get_all_packages;
use tebex_headless_rust::handlers::webstores::get_webstore;

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

    let mut packages = get_all_packages(None, None).await?;

    {
        let seed = [235; 32];
        let mut rng = rand_chacha::ChaCha8Rng::from_seed(seed);
        packages.shuffle(&mut rng);
    }

    let top_three = packages.iter().take(3);

    let qr_code = QrCode::new(&domain)?;

    let img = qr_code.render::<Luma<u8>>().module_dimensions(8, 8).build();

    let mut buf = Cursor::new(Vec::new());
    img.write_to(&mut buf, image::ImageFormat::Png)?;

    let attachment = serenity::CreateAttachment::bytes(buf.into_inner(), "qr.png");

    let embed = CreateEmbed::new()
        .title(&name)
        .url(&domain)
        .description(format!(r#"Currency: **{currency}**"#))
        .thumbnail("attachment://qr.png");

    let mut embed_two = CreateEmbed::new().title("Featured Packages");

    if let Some(color) = parse_color(&ctx.data().config.tebex.featured_embed_color) {
        embed_two = embed_two.color(color);
    }

    for pkg in top_three {
        embed_two = embed_two.field(
            format!("{}", &pkg.name),
            format!(
                "{}",
                crate::commands::tebex::etc::html_to_discord_md(&pkg.description)
            ),
            false,
        );
    }

    ctx.send(
        poise::CreateReply::default()
            .ephemeral(true)
            .embed(embed)
            .embed(embed_two)
            .attachment(attachment),
    )
    .await?;

    Ok(())
}

fn parse_color(hex: &str) -> Option<serenity::Color> {
    let hex = hex.trim_start_matches('#');
    u32::from_str_radix(hex, 16).map(serenity::Color::from).ok()
}

impl ConditionalCommand for Store {
    fn should_register(&self) -> bool {
        let public_api_key = get_public_api_key();
        match public_api_key {
            Ok(_) => {
                return true;
            }
            Err(_) => {
                return false;
            }
        }
    }

    fn command(&self) -> poise::Command<Arc<Data>, Error> {
        store()
    }
}
