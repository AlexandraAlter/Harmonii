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
#[commands(ping)]
pub struct Utils;

#[command]
async fn ping(ctx: &Context, msg: &Message, _: Args) -> CommandResult {
    if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}
