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
pub async fn clean(_ctx: &Context, _msg: &Message, _args: Args) -> CommandResult {
    Ok(())
}

