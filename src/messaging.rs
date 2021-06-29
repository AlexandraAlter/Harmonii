use serenity::{
    client::Context,
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::channel::Message,
    utils::{content_safe, ContentSafeOptions},
};

#[group]
#[commands(move_msg, say)]
struct Messaging;

#[command("move")]
pub async fn move_msg(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    Ok(())
}

#[command]
async fn say(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let settings = if let Some(guild_id) = msg.guild_id {
        ContentSafeOptions::default()
            .clean_channel(false)
            .display_as_member_from(guild_id)
    } else {
        ContentSafeOptions::default()
            .clean_channel(false)
            .clean_role(false)
    };

    let content = content_safe(&ctx.cache, &args.rest(), &settings).await;

    msg.channel_id.say(&ctx.http, &content).await?;

    Ok(())
}
