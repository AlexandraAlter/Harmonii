use serenity::{
    model::channel::Message,
    client::Context,
    framework::standard::{
        Args, CommandResult,
        macros::{command, group},
    },
};

#[group]
#[commands(clean)]
struct Cleaning;

#[command]
pub async fn clean(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    Ok(())
}

