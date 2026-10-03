use poise::{CreateReply, serenity_prelude as serenity};

use crate::Context;
use crate::Error;

fn timestamp_str(dt: i64, with_time: bool) -> String {
    format!(
        "<t:{}:{}> (<t:{}:R>)",
        dt,
        if with_time { "f" } else { "D" },
        dt
    )
}

#[poise::command(prefix_command, slash_command)]
pub async fn userinfo(ctx: Context<'_>, member: Option<serenity::Member>) -> Result<(), Error> {
    let user = match member {
        Some(member) => member,
        None => match ctx.author_member().await {
            Some(member) => member.into_owned(),
            None => return Ok(()),
        },
    };

    let mutual_servers = ctx
        .cache()
        .guilds()
        .into_iter()
        .filter_map(|guild_id| {
            let guild = ctx.cache().guild(guild_id)?;
            guild
                .members
                .contains_key(&user.user.id)
                .then(|| guild.id.clone())
        })
        .collect::<Vec<_>>()
        .len();

    let embed = serenity::builder::CreateEmbed::new()
        .title(user.user.name.as_str())
        .color(match user.colour(ctx.cache()) {
            Some(colour) => {
                if colour != serenity::Color::default() {
                    colour
                } else {
                    serenity::Color::default()
                }
            }
            None => serenity::Color::default(),
        })
        .description(
            [
                if mutual_servers > 0 {
                    Some(format!("-# **Mutual Servers:** {mutual_servers}"))
                } else {
                    None
                },
                Some(format!("- **ID:** `{}`", user.user.id)),
                Some(format!(
                    "- **Created:** {}",
                    timestamp_str(user.user.created_at().timestamp(), true)
                )),
            ]
            .into_iter()
            .flatten()
            .collect::<Vec<String>>()
            .join("\n"),
        );

    ctx.send(CreateReply::default().embed(embed)).await?;
    Ok(())
}
