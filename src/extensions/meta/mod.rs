use poise::CreateReply;
use poise::serenity_prelude as serenity;
use serenity::model::channel::Embed;

use crate::Context;
use crate::Error;

#[poise::command(prefix_command)]
pub async fn userinfo(ctx: Context<'_>, user: Option<serenity::Member>) -> Result<(), Error> {
    let user = match user {
        Some(member) => Cow::Borrowed(member),
        None => ctx.author_member().await?,
    };

    let embed = serenity::builder::CreateEmbed::new()
        .title(user.display_name())
        .colour(user.colour());

    ctx.reply(embed).await?;
    Ok(())
}
