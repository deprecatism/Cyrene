use std::borrow::Cow;

use dotenv;
use poise::serenity_prelude as serenity;

struct User {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, User, Error>;

mod extensions;

#[tokio::main]
async fn main() {
    dotenv::from_path(".env").unwrap();
    let token = std::env::var("TOKEN").expect("missing TOKEN");
    let intents = serenity::GatewayIntents::all();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![extensions::meta::userinfo()],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some(Cow::Borrowed("cy ")),
                ..Default::default()
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(User {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
